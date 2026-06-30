use actix_htmx::Htmx;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use actix_web_lab::extract::UrlEncodedForm;
use anyhow::Context;
use log::info;
use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};
use thiserror::Error;
use ui::{
    act::card::{ActCard, ActCardBuilder, ActCardBuilderError},
    event::{lineup::EventLineupBuilder, EventRoute},
    index::{IndexRoute, UiComponent as _},
};
use url::Url;
use uuid::Uuid;

use crate::{
    domain::{
        artist::{
            models::artist::{ArtistId, ArtistName},
            ports::ArtistService,
        },
        event::{
            models::{
                act::{Act, ActImg, ActName, CreateActRequest},
                event::EventId,
            },
            ports::EventService,
        },
        user::{models::user::UserId, ports::UserService},
    },
    inbound::http::{
        handlers::{event::details::lineup::event_lineup, index_markup},
        AppState,
    },
};

pub mod artist;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // .service(create_act)
        .service(web::scope("/artist").configure(artist::configure));
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

#[serde_as]
#[derive(Debug, Deserialize)]
struct AddActData {
    name: String,
    #[serde_as(as = "NoneAsEmptyString")]
    image_url: Option<String>,
    select_artist: Vec<Uuid>,
}

// #[post("")]
// async fn create_act(
//     app_state: web::Data<AppState>,
//     path: web::Path<Uuid>,
//     htmx: Htmx,
//     form: UrlEncodedForm<AddActData>,
// ) -> Result<impl Responder, HandlerError> {
//     let event_id = EventId::new(path.into_inner());
//     let author_id = UserId::new(Uuid::new_v4()); // TODO: get actual user ID from session/auth
//
//     let name = ActName::try_new(form.name.clone()).context("Invalid act name")?;
//     let description = None; // TODO: add description field to form
//     let act_img = form
//         .image_url
//         .as_ref()
//         .map(|url| {
//             ActImg::try_new(Url::parse(url).context("Invalid image URL")?)
//                 .context("Failed to create ActImg")
//         })
//         .transpose()?;
//     let artist_ids = form
//         .select_artist
//         .clone()
//         .into_iter()
//         .map(ArtistId::new)
//         .collect();
//
//     let req = CreateActRequest::new(name, description, act_img, artist_ids);
//
//     info!(
//         "Creating act for event {} with name '{}'",
//         event_id.as_ref(),
//         req.name().as_ref()
//     );
//
//     app_state
//         .event_service
//         .create_act(&event_id, &req, &author_id)
//         .await
//         .context("Failed to create act")?;
//
//     info!("Successfully created act for event {}", event_id.as_ref());
//
//     let event = app_state
//         .event_service
//         .get_event_by_id(&event_id)
//         .await
//         .context("Failed to fetch event")?;
//
//     info!("Fetched event {} after creating act", event_id.as_ref());
//
//     let lineup = event_lineup(&event, app_state.artist_service.as_ref())
//         .await
//         .context("Failed to build event lineup")?;
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(lineup.render_html()))
// }

pub fn act_card(act: &Act, artists: &[ArtistName]) -> Result<ActCard, ActCardBuilderError> {
    dbg!(artists);
    ActCardBuilder::default()
        .id(act.id().clone().into_inner())
        .name(act.name().clone().into_inner())
        .image_url(act.act_img().map(|url| url.as_ref().to_string()))
        .artists(artists.iter().map(|a| a.as_ref().to_string()).collect())
        .build()
}
