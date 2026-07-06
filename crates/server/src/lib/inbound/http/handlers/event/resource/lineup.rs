use actix_htmx::Htmx;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use anyhow::Context;
use api::UiState;
use thiserror::Error;

use crate::{
    domain::{artist::ports::ArtistService, event::models::event::Event, user::models::user::User},
    inbound::http::{user::Locale, AppState},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_lineup);
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
async fn get_lineup(
    user: Option<User>,
    locale: Locale<'_>,
    app_state: web::Data<AppState>,
    event: Event,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    let ui_state = user
        .as_ref()
        .map_or_else(|| UiState::from(locale), UiState::from);
    let event = app_state.event_mapper.map_event(&event)?;

    let body = if htmx.is_htmx {
        ui::view::event::lineup::render(&ui_state, &event, None)
    } else {
        ui::view::event::lineup::full_page(&ui_state, &event, None)
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string()))
}
