use actix_web::{HttpResponse, ResponseError, web};
use thiserror::Error;

pub fn configure(_cfg: &mut web::ServiceConfig) {
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
