use chrono::{Duration, NaiveDate, NaiveTime};
use es_entity::{DbOp, ListDirection, PaginatedQueryArgs};
use imgproxy::{ImageUrl, ProcessingOption, SignedUrlRepo};
use itertools::Itertools as _;
use ui::event::card::EventCard;

use crate::{
    entity::{
        day::{Day, DaysRepo, NewDayBuilder},
        event::{Event, EventRepo, NewEventBuilder},
        stage::{NewStageBuilder, StageRepo},
        user::UserId,
    },
    prelude::*,
};

pub async fn list_events(
    repo: &EventRepo,
    image_repo: &SignedUrlRepo,
    _page: usize,
    _page_size: usize,
) -> Result<Vec<EventCard>> {
    let query_args = PaginatedQueryArgs {
        first: 12,
        after: None,
    };

    let events = repo
        .list_by_start_date(query_args, ListDirection::Ascending)
        .await?
        .entities;

    events
        .into_iter()
        .map_into::<EventCard>()
        .map(|mut event| {
            event.image_url = image_repo
                .get(&ImageUrl::parse(&event.image_url)?.with_option(ProcessingOption::Raw(true)))?
                .to_string();
            Ok(event)
        })
        .collect::<Result<Vec<_>>>()
}

pub async fn create_event(
    events_repo: &EventRepo,
    stages_repo: &StageRepo,
    days_repo: &DaysRepo,
    user_id: UserId,
    name: String,
    description: Option<String>,
    website_url: String,
    image_url: String,
    start_date: NaiveDate,
    days: Vec<(NaiveTime, NaiveTime)>,
) -> Result<Event> {
    let new_event = NewEventBuilder::default()
        .id(uuid::Uuid::new_v4())
        .user_id(user_id)
        .name(name)
        .description(description)
        .website_url(website_url)
        .image_url(image_url)
        .start_date(start_date)
        .build()?;

    let mut op = DbOp::init(events_repo.pool()).await?;
    let event = events_repo.create_in_op(&mut op, new_event).await?;
    stages_repo
        .create_in_op(
            &mut op,
            NewStageBuilder::default()
                .id(uuid::Uuid::new_v4())
                .event_id(event.id)
                .name("Main Stage")
                .user_id(user_id)
                .build()?,
        )
        .await?;
    for (i, (day_start, day_end)) in days.into_iter().enumerate().take(10) {
        let start_time = (event.start_date + Duration::days(i as i64)).and_time(day_start);
        let end_time = Day::transform_date_from(start_time, day_end);
        days_repo
            .create_in_op(
                &mut op,
                NewDayBuilder::default()
                    .id(uuid::Uuid::new_v4())
                    .event_id(event.id)
                    .n(i as u16 + 1)
                    .start_time(start_time)
                    .end_time(end_time)
                    .user_id(user_id)
                    .build()?,
            )
            .await?;
    }
    op.commit().await?;

    Ok(event)
}
