use std::collections::HashMap;

use actix_htmx::Htmx;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::NaiveTime;
use itertools::Itertools as _;
use serde::Deserialize;
use ui::event::create::{day_buttons, EventCreate, EventDay};
use ui::index::{IndexRoute, UiComponent as _};

use crate::controller::{DayRepoData, EventRepoData, StageRepoData};
use crate::{prelude::*, service};

mod event_page;
mod timetable;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_event)
        .service(create_event_page)
        .service(add_event_day)
        .service(remove_event_day)
        .service(web::scope("/{event_id}").configure(event_page::configure));
}

#[post("")]
async fn create_event(
    event_repo: EventRepoData,
    stage_repo: StageRepoData,
    day_repo: DayRepoData,
    form: web::Form<HashMap<String, String>>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let name = form
        .get("name")
        .ok_or_else(|| crate::error::Error::Validation("Missing name".into()))?
        .clone();
    let description = form.get("description").cloned();
    let website = form
        .get("website")
        .ok_or_else(|| crate::error::Error::Validation("Missing website".into()))?
        .clone();
    let image_url = form
        .get("image_url")
        .ok_or_else(|| crate::error::Error::Validation("Missing image_url".into()))?
        .clone();
    let start_date_str = form
        .get("start_date")
        .ok_or_else(|| crate::error::Error::Validation("Missing start_date".into()))?;
    let start_date =
        chrono::NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d").map_err(|_| {
            crate::error::Error::Validation("Invalid start_date format, expected YYYY-MM-DD".into())
        })?;
    let days: Vec<(NaiveTime, NaiveTime)> = form
        .iter()
        .filter(|(key, _)| key.ends_with("_day_start_time") || key.ends_with("_day_end_time"))
        .sorted_by_key(|(key, _)| *key)
        .map(|(_, value)| value)
        .tuples()
        .enumerate()
        .map(|(i, (day_end, day_start))| {
            let day_start_time = NaiveTime::parse_from_str(day_start, "%H:%M").map_err(|_| {
                crate::error::Error::Validation(format!("Invalid start time for day {}", i + 1))
            })?;
            let day_end_time = NaiveTime::parse_from_str(day_end, "%H:%M").map_err(|_| {
                crate::error::Error::Validation(format!("Invalid end time for day {}", i + 1))
            })?;
            Result::Ok((day_start_time, day_end_time))
        })
        .try_collect()?;

    let event = service::event::create_event(
        &event_repo,
        &stage_repo,
        &day_repo,
        uuid::Uuid::new_v4().into(),
        name,
        description,
        website,
        image_url,
        start_date,
        days,
    )
    .await?;

    htmx.redirect(format!("/event/{}", event.id));

    Ok(HttpResponse::Created().body("Event created successfully"))
}

#[get("/create")]
async fn create_event_page(htmx: Htmx) -> impl Responder {
    let body = if htmx.is_htmx {
        EventCreate.render_html()
    } else {
        let route = IndexRoute::CreateEvent(EventCreate);
        service::index("Create Event", route).render_html()
    };

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(body)
}

#[derive(Debug, Deserialize)]
struct DayState {
    n: usize,
}

#[post("/day")]
async fn add_event_day(day: web::Query<DayState>) -> impl Responder {
    HttpResponse::Ok().body(
        EventDay {
            day: day.n,
            start_time: None,
            end_time: None,
        }
        .render_html()
            + day_buttons(day.n, true).render_html().as_str(),
    )
}
#[delete("/day")]
async fn remove_event_day(day: web::Query<DayState>) -> impl Responder {
    HttpResponse::Ok().body(day_buttons(day.n - 1, true).render_html())
}
