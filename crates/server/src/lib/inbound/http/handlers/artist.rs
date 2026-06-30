use actix_web::{HttpResponse, ResponseError, web::ServiceConfig};
use serde::Deserialize;
use thiserror::Error;

pub fn configure(_cfg: &mut ServiceConfig) {
    // cfg.service(add_artist_form)
    //     .service(search_artist_for_act)
    //     .service(add_artist);
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

// #[get("/add")]
// async fn add_artist_form() -> impl Responder {
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(ArtistCreate {}.render_html())
// }

#[derive(Deserialize)]
struct CreateArtistForm {
    name: String,
    genres: String,
}

// #[post("")]
// async fn add_artist(
//     app_state: web::Data<AppState>,
//     form: web::Form<CreateArtistForm>,
// ) -> Result<impl Responder, HandlerError> {
//     let author_id = UserId::new(Uuid::new_v4()); // TODO: Get from session
//     let name = ArtistName::try_new(form.name.to_string()).context("Invalid artist name")?;
//     let genres = form
//         .genres
//         .split(',')
//         .map(|s| s.trim().to_string())
//         .map(ArtistGenre::try_new)
//         .try_collect()
//         .context("Invalid genres")?;
//     let req = CreateArtistRequest::new(name, genres);
//
//     let artist = app_state
//         .artist_service
//         .create_artist(&req, &author_id)
//         .await
//         .context("Failed to create artist")?;
//
//     dbg!(artist);
//
//     Ok(HttpResponse::Created())
// }

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
