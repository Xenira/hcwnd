use std::collections::HashMap;

use actix_htmx::Htmx;
use actix_web::{
    get, post,
    web::{self, Query},
    HttpResponse, Responder, ResponseError,
};
use anyhow::Context as _;
use api::{
    act::SelectedAct,
    day::NewEventDay,
    event::{AddActToTimetable, EventListEntry, NewEvent},
    stage::SelectedStage,
    PagedResult,
};
use chrono::{Duration, NaiveDate, NaiveTime};
use es_entity::{DbOp, ListDirection, PaginatedQueryArgs};
use itertools::Itertools as _;
use log::info;
use serde::Deserialize;
use thiserror::Error;
use ui::{
    event::create::days_step::{day_buttons, EventDay},
    index::UiComponent as _,
};
use url::Url;
use uuid::Uuid;

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::{
            models::{
                day::{CreateDayRequest, Day},
                event::{
                    CreateEventRequest, EventDays, EventDaysCreateRequests, EventDescription,
                    EventName, ImageUrl, WebsiteUrl,
                },
            },
            ports::EventService,
        },
        proposal::ProposalSource,
        user::{models::user::UserId, ports::UserService},
    },
    inbound::http::AppState,
};

pub mod day;
pub mod details;

pub fn configure<ES, AS, US>(cfg: &mut web::ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    // cfg.route("", web::post().to(create_event::<ES, AS, US>))
    //     .service(web::scope("/{event_id}").configure(details::configure::<ES, AS, US>));
}

// impl TryFrom<(NaiveDate, NewEventDay)> for CreateDayRequest {
//     type Error = anyhow::Error;
//
//     fn try_from(value: (NaiveDate, NewEventDay)) -> anyhow::Result<Self> {
//         let start_time = (value.0 + Duration::days(value.1.n as i64)).and_time(value.1.start_time);
//         let end_time = Day::transform_date_from(start_time, value.1.end_time);
//
//         Ok(CreateDayRequest::try_new(start_time, end_time)?)
//     }
// }

// impl TryFrom<NewEvent> for CreateEventRequest {
//     type Error = anyhow::Error;
//
//     fn try_from(value: NewEvent) -> anyhow::Result<Self> {
//         let name = EventName::try_new(value.name)?;
//         let description = value
//             .description
//             .map(|d| EventDescription::try_new(d))
//             .transpose()?;
//         let event_days = value
//             .days
//             .into_iter()
//             .map(|d| (value.start_date, d).try_into())
//             .try_collect()?;
//         let days = EventDaysCreateRequests::try_new(event_days)?;
//         let website_url = Url::parse(&value.website_url)?;
//         let website_url = WebsiteUrl::try_new(Url::parse(&value.website_url)?)?;
//         let image_url = ImageUrl::try_new(Url::parse(&value.image_url)?)?;
//         // TODO: get real proposal source from form
//         let source = ProposalSource::try_new("TODO: source".to_string())?;
//
//         Ok(Self::new(
//             name,
//             description,
//             days,
//             website_url,
//             image_url,
//             source,
//             None,
//         ))
//     }
// }

// #[derive(Deserialize)]
// struct EventQueryData {
//     count: Option<usize>,
//     after_id: Option<Uuid>,
//     after_start_date: Option<NaiveDate>,
// }
//
// #[get("/events")]
// async fn get_events(events: EventRepoData, query: Query<EventQueryData>) -> Result<impl Responder> {
//     let query_args = PaginatedQueryArgs {
//         first: query.count.unwrap_or(1).min(100),
//         after: if let (Some(id), Some(start_date)) = (query.after_id, query.after_start_date) {
//             Some(EventsByStartDateCursor {
//                 id: id.into(),
//                 start_date,
//             })
//         } else {
//             None
//         },
//     };
//     let events = events
//         .list_by_start_date(query_args, ListDirection::Ascending)
//         .await
//         .unwrap();
//
//     let items = events
//         .entities
//         .into_iter()
//         .map_into::<EventListEntry>()
//         .collect_vec();
//
//     let result = PagedResult {
//         items,
//         has_next_page: events.has_next_page,
//         after: events
//             .end_cursor
//             .map(|cursor| (cursor.id, cursor.start_date)),
//     };
//     Ok(HttpResponse::Ok().json(result))
// }
//
// #[get("/events/{event_id}")]
// async fn get_event(events: EventRepoData, path: web::Path<EventId>) -> impl Responder {
//     match events.find_by_id(path.into_inner()).await {
//         Ok(event) => HttpResponse::Ok().json(event),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }
//
#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    ServiceError(#[from] anyhow::Error),
}

impl ResponseError for HandlerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HandlerError::ServiceError(e) => {
                HttpResponse::InternalServerError().body(format!("Service error: {e}"))
            }
        }
    }
}

async fn create_event<ES: EventService, AS: ArtistService, US: UserService>(
    app_state: web::Data<AppState<ES, AS, US>>,
    form: web::Form<HashMap<String, String>>,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    info!("Received create event request with form data: {:?}", form);
    // let user_id: UserId = UserId::new(Uuid::new_v4());
    //
    // let name = form
    //     .get("name")
    //     .ok_or_else(|| anyhow::format_err!("Missing name"))?
    //     .clone();
    // let description = form.get("description").cloned();
    // let website = form
    //     .get("website")
    //     .ok_or_else(|| anyhow::format_err!("Missing website"))?
    //     .clone();
    // let image_url = form
    //     .get("image_url")
    //     .ok_or_else(|| anyhow::format_err!("Missing image_url"))?
    //     .clone();
    // let start_date_str = form
    //     .get("start_date")
    //     .ok_or_else(|| anyhow::format_err!("Missing start_date"))?;
    // let start_date = chrono::NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")
    //     .context("Invalid start_date format, expected YYYY-MM-DD")?;
    // let days: Vec<(usize, NaiveTime, NaiveTime)> = form
    //     .iter()
    //     .filter(|(key, _)| key.ends_with("_day_start_time") || key.ends_with("_day_end_time"))
    //     .sorted_by_key(|(key, _)| *key)
    //     .map(|(_, value)| value)
    //     .tuples()
    //     .enumerate()
    //     .map(|(i, (day_end, day_start))| {
    //         let day_start_time = NaiveTime::parse_from_str(day_start, "%H:%M")
    //             .with_context(|| format!("Invalid start time for day {}", i + 1))?;
    //         let day_end_time = NaiveTime::parse_from_str(day_end, "%H:%M")
    //             .with_context(|| format!("Invalid end time for day {}", i + 1))?;
    //         anyhow::Result::Ok((i, day_start_time, day_end_time))
    //     })
    //     .collect::<anyhow::Result<Vec<_>>>()?;
    //
    // let new_event = NewEvent {
    //     name,
    //     description,
    //     website_url: website,
    //     image_url,
    //     start_date,
    //     days: days
    //         .into_iter()
    //         .map(|(n, start_time, end_time)| NewEventDay {
    //             n: n as u16,
    //             start_time,
    //             end_time,
    //         })
    //         .collect(),
    // };
    //
    // let create_event_request = new_event.try_into()?;
    // let event_id = app_state
    //     .event_service
    //     .create_event(&create_event_request, &user_id)
    //     .await
    //     .context("Failed to create event")?;
    //
    // htmx.redirect(format!("/event/{}", event_id.into_inner()));

    // let new_event = NewEventBuilder::default()
    //     .id(uuid::Uuid::new_v4())
    //     .user_id(user_id)
    //     .name(name)
    //     .description(description)
    //     .website_url(website_url)
    //     .image_url(image_url)
    //     .start_date(start_date)
    //     .build()?;
    //
    // let mut op = DbOp::init(events_repo.pool()).await?;
    // let event = events_repo.create_in_op(&mut op, new_event).await?;
    // stages_repo
    //     .create_in_op(
    //         &mut op,
    //         NewStageBuilder::default()
    //             .id(uuid::Uuid::new_v4())
    //             .event_id(event.id)
    //             .name("Main Stage")
    //             .user_id(user_id)
    //             .build()?,
    //     )
    //     .await?;
    // for (i, (day_start, day_end)) in days.into_iter().enumerate().take(10) {
    //     let start_time = (event.start_date + Duration::days(i as i64)).and_time(day_start);
    //     let end_time = Day::transform_date_from(start_time, day_end);
    //     days_repo
    //         .create_in_op(
    //             &mut op,
    //             NewDayBuilder::default()
    //                 .id(uuid::Uuid::new_v4())
    //                 .event_id(event.id)
    //                 .n(i as u16 + 1)
    //                 .start_time(start_time)
    //                 .end_time(end_time)
    //                 .user_id(user_id)
    //                 .build()?,
    //         )
    //         .await?;
    // }
    // op.commit().await?;

    Ok(HttpResponse::Created().body("Event created successfully"))
}

//
// #[post("/events/{event_id}/timetable")]
// async fn add_act(
//     pool: PostgresPool,
//     stages_repo: StageRepoData,
//     acts_repo: ActRepoData,
//     days_repo: DayRepoData,
//     data: web::Json<AddActToTimetable>,
//     path: web::Path<EventId>,
// ) -> Result<impl Responder> {
//     let event_id = path.into_inner();
//     let user_id: UserId = uuid::Uuid::new_v4().into();
//     let mut op = DbOp::init(&pool).await?;
//     let mut act = match &data.act {
//         SelectedAct::New(new_act) => {
//             let new_act = NewActBuilder::default()
//                 .id(uuid::Uuid::new_v4())
//                 .event_id(event_id)
//                 .stage_id(new_act.stage_id.map(Into::into))
//                 .name(&new_act.name)
//                 .description(new_act.description.clone())
//                 .start_time(new_act.start_time)
//                 .end_time(new_act.end_time)
//                 .user_id(user_id)
//                 .build()?;
//
//             acts_repo.create_in_op(&mut op, new_act).await?
//         }
//         SelectedAct::Existing(act_id) => acts_repo.find_by_id(ActId::from(*act_id)).await?,
//     };
//
//     let stage_id = match &data.stage {
//         SelectedStage::New(new_stage) => {
//             let new_stage = NewStageBuilder::default()
//                 .id(uuid::Uuid::new_v4())
//                 .event_id(event_id)
//                 .name(&new_stage.name)
//                 .user_id(user_id)
//                 .build()?;
//             stages_repo.create_in_op(&mut op, new_stage).await?.id
//         }
//         SelectedStage::Existing(stage_id) => stage_id.clone().into(),
//     };
//
//     act.set_stage(Some(stage_id), user_id);
//
//     if let Some(data_day) = &data.times {
//         let Some(day) = days_repo.get_day(event_id, data_day.day).await? else {
//             return Err(Error::NotFound);
//         };
//         let start_time = day.transform_time(data_day.start_time);
//         let end_time = day.transform_time(data_day.end_time);
//         act.set_time(Some((start_time, end_time)), user_id);
//     }
//
//     acts_repo.update_in_op(&mut op, &mut act).await?;
//     op.commit().await?;
//
//     Ok(HttpResponse::NoContent().finish())
// }
