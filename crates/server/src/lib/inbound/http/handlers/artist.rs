use actix_htmx::Htmx;
use actix_web::{HttpResponse, Responder, ResponseError, get, web::ServiceConfig};
use api::artist::ArtistCreateForm;
use serde::Deserialize;
use serde_qs::web::QsForm;
use thiserror::Error;

use crate::inbound::http::user::UiStateExtractor;

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
async fn get_artists(state: UiStateExtractor, htmx: Htmx) -> impl Responder {
    let body = if htmx.is_htmx {
        ui::view::artist::list::render(&state, &vec![])
    } else {
        ui::view::artist::list::full_page(&state, &vec![])
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[derive(Deserialize)]
struct SearchArtistQuery {
    name: String,
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
