use actix_web::{
    get, post,
    web::{self, Query},
    HttpResponse, Responder,
};
use api::{
    act::SelectedAct,
    event::{AddActToTimetable, EventListEntry},
    stage::SelectedStage,
    PagedResult,
};
use chrono::NaiveDate;
use es_entity::{DbOp, ListDirection, PaginatedQueryArgs};
use itertools::Itertools as _;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    controller::{ActRepoData, DayRepoData, EventRepoData, StageRepoData},
    entity::{
        act::{ActId, NewActBuilder},
        day::Day,
        event::{event_cursor::EventsByStartDateCursor, EventId, NewEventBuilder},
        stage::NewStageBuilder,
        user::UserId,
    },
    error::Error,
    prelude::*,
    PostgresPool,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_events)
        .service(get_event)
        .service(create_event);
}

#[derive(Deserialize)]
struct EventQueryData {
    count: Option<usize>,
    after_id: Option<Uuid>,
    after_start_date: Option<NaiveDate>,
}

#[get("/events")]
async fn get_events(events: EventRepoData, query: Query<EventQueryData>) -> Result<impl Responder> {
    let query_args = PaginatedQueryArgs {
        first: query.count.unwrap_or(1).min(100),
        after: if let (Some(id), Some(start_date)) = (query.after_id, query.after_start_date) {
            Some(EventsByStartDateCursor {
                id: id.into(),
                start_date,
            })
        } else {
            None
        },
    };
    let events = events
        .list_by_start_date(query_args, ListDirection::Ascending)
        .await
        .unwrap();

    let items = events
        .entities
        .into_iter()
        .map_into::<EventListEntry>()
        .collect_vec();

    let result = PagedResult {
        items,
        has_next_page: events.has_next_page,
        after: events
            .end_cursor
            .map(|cursor| (cursor.id, cursor.start_date)),
    };
    Ok(HttpResponse::Ok().json(result))
}

#[get("/events/{event_id}")]
async fn get_event(events: EventRepoData, path: web::Path<EventId>) -> impl Responder {
    match events.find_by_id(path.into_inner()).await {
        Ok(event) => HttpResponse::Ok().json(event),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/events")]
async fn create_event(
    pool: PostgresPool,
    events_repo: EventRepoData,
    stages_repo: StageRepoData,
    days_repo: DayRepoData,
    data: web::Json<api::event::NewEvent>,
) -> Result<impl Responder> {
    let user_id: UserId = uuid::Uuid::new_v4().into();
    let new_event = NewEventBuilder::default()
        .id(uuid::Uuid::new_v4())
        .user_id(uuid::Uuid::new_v4())
        .name(data.name.clone())
        .description(data.description.clone())
        .website_url(data.website_url.clone())
        .image_url(data.image_url.clone())
        .start_date(data.start_date)
        .build()?;

    let mut op = DbOp::init(&pool).await?;
    let event = events_repo.create_in_op(&mut op, new_event).await?;
    stages_repo
        .create_in_op(
            &mut op,
            NewStageBuilder::default()
                .id(uuid::Uuid::new_v4())
                .event_id(event.id)
                .name("Main Stage")
                .user_id(uuid::Uuid::new_v4())
                .build()?,
        )
        .await?;
    for day in &data.days {
        let start_time = event.start_date.and_time(day.start_time);
        let end_time = Day::transform_date_from(start_time, day.end_time);
        days_repo
            .create_in_op(
                &mut op,
                crate::entity::day::NewDayBuilder::default()
                    .id(uuid::Uuid::new_v4())
                    .event_id(event.id)
                    .n(day.n)
                    .start_time(start_time)
                    .end_time(end_time)
                    .user_id(user_id)
                    .build()?,
            )
            .await?;
    }
    op.commit().await?;

    Ok(HttpResponse::Created().json(event))
}

#[post("/events/{event_id}/timetable")]
async fn add_act(
    pool: PostgresPool,
    stages_repo: StageRepoData,
    acts_repo: ActRepoData,
    days_repo: DayRepoData,
    data: web::Json<AddActToTimetable>,
    path: web::Path<EventId>,
) -> Result<impl Responder> {
    let event_id = path.into_inner();
    let user_id: UserId = uuid::Uuid::new_v4().into();
    let mut op = DbOp::init(&pool).await?;
    let mut act = match &data.act {
        SelectedAct::New(new_act) => {
            let new_act = NewActBuilder::default()
                .id(uuid::Uuid::new_v4())
                .event_id(event_id)
                .stage_id(new_act.stage_id.map(Into::into))
                .name(&new_act.name)
                .description(new_act.description.clone())
                .start_time(new_act.start_time)
                .end_time(new_act.end_time)
                .user_id(user_id)
                .build()?;

            acts_repo.create_in_op(&mut op, new_act).await?
        }
        SelectedAct::Existing(act_id) => acts_repo.find_by_id(ActId::from(*act_id)).await?,
    };

    let stage_id = match &data.stage {
        SelectedStage::New(new_stage) => {
            let new_stage = NewStageBuilder::default()
                .id(uuid::Uuid::new_v4())
                .event_id(event_id)
                .name(&new_stage.name)
                .user_id(user_id)
                .build()?;
            stages_repo.create_in_op(&mut op, new_stage).await?.id
        }
        SelectedStage::Existing(stage_id) => stage_id.clone().into(),
    };

    act.set_stage(Some(stage_id), user_id);

    if let Some(data_day) = &data.times {
        let Some(day) = days_repo.get_day(event_id, data_day.day).await? else {
            return Err(Error::NotFound);
        };
        let start_time = day.transform_time(data_day.start_time);
        let end_time = day.transform_time(data_day.end_time);
        act.set_time(Some((start_time, end_time)), user_id);
    }

    acts_repo.update_in_op(&mut op, &mut act).await?;
    op.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}
