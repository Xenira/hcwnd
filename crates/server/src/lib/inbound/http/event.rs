use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use domain::event::models::event::{Event, EventId};
use futures::future::LocalBoxFuture;
use log::error;
use uuid::Uuid;

use crate::inbound::http::AppState;

pub struct RequestEvent(pub Event);

impl Deref for RequestEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for RequestEvent {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let app_state = req.app_data::<web::Data<AppState>>().cloned();
        let event_id = req.match_info().get("event_id").map(Uuid::parse_str);

        Box::pin(async move {
            let app_state = app_state.ok_or_else(|| {
                error!("App state is not configured");
                actix_web::error::ErrorInternalServerError("App state is not configured")
            })?;

            let event_id = event_id
                .ok_or_else(|| {
                    error!("Event ID is not provided in the request");
                    actix_web::error::ErrorBadRequest("Event ID is required")
                })?
                .or_else(|e| {
                    error!("Failed to parse event ID from request: {}", &e);
                    Err(actix_web::error::ErrorBadRequest(format!(
                        "Invalid event ID format: {}",
                        e
                    )))
                })?;

            Ok(RequestEvent(
                app_state
                    .event_service
                    .get_event_by_id(&EventId::new(event_id))
                    .await
                    .map_err(|e| {
                        error!("Failed to retrieve event from service: {}", &e);
                        actix_web::error::ErrorInternalServerError(format!(
                            "Failed to retrieve event: {}",
                            e
                        ))
                    })?,
            ))
        })
    }
}
