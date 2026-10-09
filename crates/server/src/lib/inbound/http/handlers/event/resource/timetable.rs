use actix_htmx::Htmx;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use anyhow::Context as _;
use api::UiState;
use domain::{
    festival::models::festival::{Festival, FestivalId},
    user::models::user::User,
};
use thiserror::Error;
use ui::event::details::EventDetails;
use uuid::Uuid;

use crate::inbound::http::{
    user::{Locale, UiStateExtractor},
    AppState,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_timetable);
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

// #[get("")]
// async fn get_timetable(
//     state: UiStateExtractor,
//     app_state: web::Data<AppState>,
//     path: web::Path<Uuid>,
//     htmx: Htmx,
// ) -> Result<impl Responder, HandlerError> {
//     let event_id = EventId::new(path.into_inner());
//     let event = app_state
//         .event_service
//         .get_event_by_id(&event_id)
//         .await
//         .context("Failed to fetch event")?;
//     let event = app_state
//         .event_mapper
//         .map_event(&event)
//         .context("Failed to map event")?;
//
//     let body = if htmx.is_htmx {
//         ui::view::event::timetable::render(&state, &event)
//     } else {
//         ui::view::event::timetable::full_page(&state, &event)
//     };
//
//     Ok(HttpResponse::Ok()
//         .insert_header(("Content-Type", "text/html"))
//         .body(body.into_string()))
// }
