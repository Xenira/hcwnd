use actix_web::{
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use anyhow::Context as _;
use itertools::Itertools as _;
use serde::Deserialize;
use thiserror::Error;
use ui::{
    act::create::ArtistSearchResults,
    artist::ArtistCreate,
    event::{card::EventCard, list::EventListBuilder},
    index::{Index, IndexBuilder, IndexRoute, UiComponent as _},
};
use uuid::Uuid;

use crate::{
    domain::{
        artist::{
            models::artist::{ArtistGenre, ArtistName, CreateArtistRequest},
            ports::ArtistService,
        },
        event::{
            models::event::{Event, EventListItem},
            ports::EventService,
        },
        user::{models::user::UserId, ports::UserService},
    },
    inbound::http::AppState,
};

pub fn configure<ES, AS, US>(cfg: &mut ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.route("/add", web::get().to(add_artist_form))
        .route("/act", web::get().to(search_artist_for_act::<ES, AS, US>))
        .route("", web::post().to(add_artist::<ES, AS, US>));
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

async fn add_artist_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(ArtistCreate {}.render_html())
}

#[derive(Deserialize)]
struct CreateArtistForm {
    name: String,
    genres: String,
}

async fn add_artist<ES: EventService, AS: ArtistService, US: UserService>(
    app_state: web::Data<AppState<ES, AS, US>>,
    form: web::Form<CreateArtistForm>,
) -> Result<impl Responder, HandlerError> {
    let author_id = UserId::new(Uuid::new_v4()); // TODO: Get from session
    let name = ArtistName::try_new(form.name.to_string()).context("Invalid artist name")?;
    let genres = form
        .genres
        .split(',')
        .map(|s| s.trim().to_string())
        .map(ArtistGenre::try_new)
        .try_collect()
        .context("Invalid genres")?;
    let req = CreateArtistRequest::new(name, genres);

    let artist = app_state
        .artist_service
        .create_artist(&req, &author_id)
        .await
        .context("Failed to create artist")?;

    dbg!(artist);

    Ok(HttpResponse::Created())
}

#[derive(Deserialize)]
struct SearchArtistQuery {
    name: String,
}

async fn search_artist_for_act<ES: EventService, AS: ArtistService, US: UserService>(
    app_state: web::Data<AppState<ES, AS, US>>,
    query: web::Query<SearchArtistQuery>,
) -> Result<impl Responder, HandlerError> {
    let artists = app_state
        .artist_service
        .artists_by_act(&query.name)
        .await
        .context("Failed to search artists")?;

    let res = ArtistSearchResults {
        artists: artists
            .into_iter()
            .map(|a| (a.id().clone().into_inner(), a.name().clone().into_inner()))
            .collect_vec(),
    }
    .render_html();

    Ok(HttpResponse::Ok().content_type("text/html").body(res))
}
