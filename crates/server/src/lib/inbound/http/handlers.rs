use actix_identity::Identity;
use actix_web::{
    HttpResponse, Responder, ResponseError,
    web::{self, Data, ServiceConfig},
};
use anyhow::Context as _;
use itertools::Itertools as _;
use thiserror::Error;
use ui::{
    event::{card::EventCard, list::EventListBuilder},
    index::{Index, IndexBuilder, IndexRoute, UiComponent as _},
};
use uuid::Uuid;

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::{
            models::event::{Event, EventListItem},
            ports::EventService,
        },
        user::{
            models::user::{User, UserId},
            ports::UserService,
        },
    },
    inbound::http::{AppState, user::UserExtractor},
};

pub mod artist;
pub mod assets;
pub mod create_event;
pub mod event;
pub mod login;
pub mod logout;
pub mod signup;

pub fn configure<ES, AS, US>(cfg: &mut ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.route("/", web::get().to(index::<ES, AS, US>))
        .service(
            web::scope(ui::event::create::BASE_ROUTE)
                .configure(create_event::configure::<ES, AS, US>),
        )
        .service(web::scope("/assets").configure(assets::configure))
        .service(web::scope("/event").configure(event::configure::<ES, AS, US>))
        .service(web::scope("/artist").configure(artist::configure::<ES, AS, US>))
        .service(web::scope("/signup").configure(signup::configure::<ES, AS, US>))
        .service(web::scope("/login").configure(login::configure::<ES, AS, US>))
        .service(web::scope("/logout").configure(logout::configure));
}

impl From<&EventListItem> for EventCard {
    fn from(event: &EventListItem) -> Self {
        let id = event.id().clone().into_inner();
        let title = event.name().clone().into_inner();
        // let description = event.description().map(|d| d.to_string());
        let image_url = event.image_url().clone().into_inner();
        EventCard {
            id,
            title,
            image_url,
            start_date: event.start_date(),
            start_time: event.start_time(),
            end_date: event.end_date(),
            end_time: event.end_time(),
        }
    }
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

async fn index<ES: EventService, AS: ArtistService, US: UserService>(
    app_state: Data<AppState<ES, AS, US>>,
    user: Option<UserExtractor<ES, AS, US>>,
) -> Result<impl Responder, HandlerError> {
    let events = app_state
        .event_service
        .list_events()
        .await
        .context("Failed to list events")?;
    dbg!(&events);
    let list = EventListBuilder::default()
        .events(events.iter().map_into().collect_vec())
        .page(1)
        .has_more(false)
        .build()
        .expect("Failed to build event list");
    let user = user.map(|u| u.user);
    // let list = EventListBuilder::default()
    //     .events(list_events(&event_repo, &image_repo, 1, 12).await?)
    //     .page(1)
    //     .has_more(false)
    //     .build()
    //     .expect("Failed to build event list");
    let route = IndexRoute::Home(list);
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(index_markup("Events", route, user).render_html()))
}

pub fn index_markup(title: &str, route: IndexRoute, user: Option<User>) -> Index {
    let user = user.map(|u| ui::user::User {
        username: u.name().as_ref().to_string(),
    });
    IndexBuilder::default()
        .title(title.to_string())
        .user(user)
        .outlet(route)
        .build()
        .expect("Failed to build index page")
}

// generalize_actix_handler!(Get, "/", index, IndexHandler<ES: EventService>);
