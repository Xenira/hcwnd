use actix_htmx::Htmx;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use anyhow::Context as _;
use api::UiState;
use thiserror::Error;
use ui::event::details::EventDetails;
use uuid::Uuid;

use crate::{
    domain::{
        event::models::event::{Event, EventId},
        user::models::user::User,
    },
    inbound::http::{user::Locale, AppState},
};

// pub mod act;
// pub mod lineup;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_event);
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

impl From<Event> for EventDetails {
    fn from(value: Event) -> Self {
        Self {
            id: value.id().clone().into_inner(),
            title: value.name().clone().into_inner(),
            description: value.description().clone().into_inner(),
            image_url: "".to_string(),
            start_date: value.start_date().date(),
        }
    }
}

#[get("")]
async fn get_event(
    user: Option<User>,
    locale: Locale<'_>,
    app_state: web::Data<AppState>,
    path: web::Path<Uuid>,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    let ui_state = user
        .as_ref()
        .map_or_else(|| UiState::from(locale), |u| UiState::from(u));

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
        ui::view::event::detail::render(&ui_state, &event)
    } else {
        ui::view::event::detail::full_page(&ui_state, &event)
    };

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(body.into_string()))
}
