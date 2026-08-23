use actix_htmx::Htmx;
use actix_web::{
    HttpResponse, Responder, ResponseError, get, post,
    web::{self, ServiceConfig},
};
use api::artist::{ArtistCreateForm, ArtistCreateSubmitForm};
use itertools::Itertools;
use log::info;
use serde::Deserialize;
use serde_qs::web::QsForm;
use thiserror::Error;

use crate::{
    domain::{
        artist::models::artist::{ArtistGenre, ArtistName, CreateArtistRequest},
        user::models::user::User,
    },
    inbound::http::{AppState, user::UiStateExtractor},
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(create_artist_form).service(create_artist);
}

#[derive(Error, Debug)]
enum HandlerError {
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
async fn create_artist_form(_user: User, state: UiStateExtractor, htmx: Htmx) -> impl Responder {
    let body = if htmx.is_htmx {
        ui::view::artist::create::render(&state, &ArtistCreateForm::default())
    } else {
        ui::view::artist::create::full_page(&state, &ArtistCreateForm::default())
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[post("")]
async fn create_artist(
    user: User,
    state: UiStateExtractor,
    app_state: web::Data<AppState>,
    form: QsForm<ArtistCreateSubmitForm>,
    htmx: Htmx,
) -> impl Responder {
    info!("Creating artist: {:?}", form);
    let req = CreateArtistRequest::new(
        ArtistName::try_new(form.name.clone()).expect("Invalid artist name"),
        form.image_url.clone(),
        form.website_url.clone(),
        form.genres
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|g| ArtistGenre::try_new(g.clone()))
            .try_collect()
            .expect("Invalid artist genres"),
    );

    let artist = app_state
        .artist_service
        .create_artist(&req, user.id())
        .await
        .expect("Failed to create artist");

    let id = artist.id().as_ref();
    let url = format!("{}/{}", ui::view::artist::BASE_PATH, id);
    if htmx.is_htmx {
        htmx.redirect_with_swap(url);
        HttpResponse::Created().finish()
    } else {
        HttpResponse::Found()
            .insert_header(("Location", url))
            .finish()
    }
}
