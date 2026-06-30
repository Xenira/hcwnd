use actix_htmx::Htmx;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use anyhow::Context as _;
use thiserror::Error;
use ui::{
    event::{details::EventDetails, EventRoute},
    index::{IndexRoute, UiComponent as _},
};
use uuid::Uuid;

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::{
            models::event::{Event, EventId},
            ports::EventService,
        },
        user::ports::UserService,
    },
    inbound::http::{handlers::index_markup, AppState},
};

pub mod act;
pub mod lineup;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_event)
        .service(web::scope("/lineup").configure(lineup::configure))
        .service(web::scope("/act").configure(act::configure));
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
    let event_details: EventDetails = event.into();
    let event_title = event_details.title.clone();
    let event_route = EventRoute::Details(event_details);
    let event = ui::event::EventBuilder::default()
        .id(event_id.into_inner())
        .outlet(event_route)
        .build()
        .expect("Failed to build event page");

    let body = if htmx.is_htmx {
        event.render_html()
    } else {
        let index_route = IndexRoute::Event(event);
        index_markup(event_title.as_str(), index_route, None).render_html()
    };

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(body))
}
