use actix_htmx::Htmx;
use actix_web::{web, HttpResponse, Responder, ResponseError};
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
            models::artist::{ArtistGenre, ArtistName, CreateArtistRequest},
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

pub fn configure<ES, AS, US>(cfg: &mut web::ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    // cfg.route("/add", web::get().to(add_artist_form))
    //     .route("/{artist_id}", web::post().to(add_artist::<ES, AS>));
    // .route("/{artist_id}", web::delete().to(delete_artist::<ES, AS>));
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
