use actix_htmx::Htmx;
use actix_web::{HttpResponse, Responder, ResponseError, get, web};
use anyhow::Context as _;
use api::UiState;
use domain::{
    event::models::event::{Event, EventId},
    user::models::user::User,
};
use thiserror::Error;
use ui::event::details::EventDetails;
use uuid::Uuid;

use crate::inbound::http::{
    AppState,
    user::{Locale, UiStateExtractor},
};

// pub mod act;
pub mod lineup;
pub mod timetable;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_event)
        .service(web::scope(api::routes::EVENT_TIMETABLE_ROUTE).configure(timetable::configure))
        .service(web::scope(api::routes::EVENT_LINEUP_ROUTE).configure(lineup::configure));
    //     .service(web::scope("/lineup").configure(lineup::configure))
    //     .service(web::scope("/act").configure(act::configure));
}

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

#[get("")]
async fn get_event(
    state: UiStateExtractor,
    app_state: web::Data<AppState>,
    path: web::Path<Uuid>,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    let event_id = EventId::new(path.into_inner());
    let event = app_state
        .event_service
        .get_event_by_id(&event_id)
        .await
        .context("Failed to fetch event")?;
    let event = app_state
        .event_mapper
        .map_event(&event)
        .context("Failed to map event")?;

    let body = if htmx.is_htmx {
        ui::view::event::detail::render(&state, &event)
    } else {
        ui::view::event::detail::full_page(&state, &event)
    };

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(body.into_string()))
}
