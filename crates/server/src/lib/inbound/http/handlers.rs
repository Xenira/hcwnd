use actix_htmx::Htmx;
use actix_web::{
    get,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use anyhow::Context as _;
use thiserror::Error;

use crate::{
    domain::{event::ports::EventService, user::models::user::User},
    inbound::http::{user::Locale, AppState},
};

pub mod artist;
pub mod assets;
pub mod create_event;
pub mod event;
pub mod login;
pub mod logout;
pub mod signup;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index)
        .service(web::scope(ui::event::create::BASE_ROUTE).configure(create_event::configure))
        .service(web::scope("/assets").configure(assets::configure))
        .service(web::scope(api::routes::EVENT_ROUTE).configure(event::configure))
        .service(web::scope(api::routes::ARTIST_ROUTE).configure(artist::configure))
        .service(web::scope("/signup").configure(signup::configure))
        .service(web::scope("/login").configure(login::configure))
        .service(web::scope("/logout").configure(logout::configure));
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

#[get("/")]
async fn index(
    app_state: Data<AppState>,
    user: Option<User>,
    locale: Locale<'_>,
    htmx: Htmx,
) -> Result<impl Responder, HandlerError> {
    let state = user
        .as_ref()
        .map(api::UiState::from)
        .unwrap_or_else(|| api::UiState::from(&locale));

    let events = app_state
        .event_service
        .list_events()
        .await
        .context("Failed to list events")?
        .into_iter()
        .map(|event| app_state.event_mapper.map_event(&event))
        .collect::<Result<Vec<_>, _>>()?;
    // let list = EventListBuilder::default()
    //     .events(list_events(&event_repo, &image_repo, 1, 12).await?)
    //     .page(1)
    //     .has_more(false)
    //     .build()
    //     .expect("Failed to build event list");
    let body = if htmx.is_htmx {
        ui::event::list::render(&state, &events, 0, false)
    } else {
        ui::event::list::full_page(&state, &events)
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string()))
}
