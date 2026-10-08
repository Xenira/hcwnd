use actix_htmx::Htmx;
use actix_web::{HttpResponse, Responder, ResponseError, get, web};
use anyhow::Context;
use api::UiState;
use domain::{
    artist::ports::ArtistService, event::models::festival::Festival, user::models::user::User,
};
use thiserror::Error;

use crate::inbound::http::{
    AppState,
    event::RequestEvent,
    user::{Locale, RequestUser, UiStateExtractor},
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
    state: UiStateExtractor,
    app_state: web::Data<AppState>,
    event: RequestEvent,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    let event = app_state.event_mapper.map_festival(&event)?;

    let body = if htmx.is_htmx {
        ui::view::event::lineup::render(&state, &event, None)
    } else {
        ui::view::event::lineup::full_page(&state, &event, None)
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string()))
}
