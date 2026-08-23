use actix_htmx::Htmx;
use actix_web::{
    HttpResponse, Responder, ResponseError, get,
    web::{self, ServiceConfig},
};
use api::artist::ArtistCreateForm;
use itertools::Itertools;
use log::debug;
use serde::Deserialize;
use serde_qs::web::QsForm;
use thiserror::Error;

use crate::{
    domain::artist::models::artist::SearchArtistsQuery,
    inbound::http::{AppState, user::UiStateExtractor},
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(get_artists);
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

#[derive(Deserialize)]
struct CreateArtistForm {
    name: String,
    genres: String,
}

#[get("")]
async fn get_artists(
    state: UiStateExtractor,
    app_state: web::Data<AppState>,
    htmx: Htmx,
) -> impl Responder {
    let artists = app_state
        .artist_service
        .search_artists(&SearchArtistsQuery::default())
        .await
        .unwrap_or_default()
        .iter()
        .map(|a| app_state.artist_mapper.map_artist(a))
        .filter_map(|a| a.ok())
        .collect_vec();

    debug!("Found {} artists", artists.len());
    dbg!(&artists);

    let body = if htmx.is_htmx {
        ui::view::artist::list::render(&state, &artists)
    } else {
        ui::view::artist::list::full_page(&state, &artists)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

// #[get("/act")]
// async fn search_artist_for_act(
//     app_state: web::Data<AppState>,
//     query: web::Query<SearchArtistQuery>,
// ) -> Result<impl Responder, HandlerError> {
//     let artists = app_state
//         .artist_service
//         .artists_by_act(&query.name)
//         .await
//         .context("Failed to search artists")?;
//
//     let res = ArtistSearchResults {
//         artists: artists
//             .into_iter()
//             .map(|a| (a.id().clone().into_inner(), a.name().clone().into_inner()))
//             .collect_vec(),
//     }
//     .render_html();
//
//     Ok(HttpResponse::Ok().content_type("text/html").body(res))
// }
