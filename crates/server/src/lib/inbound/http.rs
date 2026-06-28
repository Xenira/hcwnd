use std::{fs, sync::Arc, time::Duration};

use actix_files::Files;
use actix_htmx::HtmxMiddleware;
use actix_identity::IdentityMiddleware;
use actix_session::{
    storage::{RedisSessionStore, SessionStore},
    SessionMiddleware,
};
use actix_web::{
    cookie::Key,
    dev::ServiceResponse,
    http::{header, StatusCode},
    middleware::{Compress, ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpResponse,
};
use actix_web_helmet::Helmet;
use anyhow::Context as _;
use imgproxy::SignedUrlRepo;
use log::info;
use tracing_actix_web::TracingLogger;

use crate::domain::{
    artist::ports::ArtistService, event::ports::EventService, user::ports::UserService,
};

pub mod actix_macro;
pub mod handlers;
pub mod user;

#[derive(Debug, Clone)]
pub struct AppState<ES, AS, US>
where
    ES: EventService + Clone + Sync + Send + 'static,
    AS: ArtistService + Clone + Sync + Send + 'static,
    US: UserService + Clone + Sync + Send + 'static,
{
    event_service: Arc<ES>,
    artist_service: Arc<AS>,
    user_service: Arc<US>,
}

impl<ES, AS, US> AppState<ES, AS, US>
where
    ES: EventService + Clone + Sync + Send + 'static,
    AS: ArtistService + Clone + Sync + Send + 'static,
    US: UserService + Clone + Sync + Send + 'static,
{
    pub fn new(event_service: ES, artist_service: AS, user_service: US) -> Self {
        Self {
            event_service: Arc::new(event_service),
            artist_service: Arc::new(artist_service),
            user_service: Arc::new(user_service),
        }
    }
}

pub struct HttpServer;

impl HttpServer {
    pub async fn run<ES, AS, US>(
        event_service: ES,
        artist_service: AS,
        user_service: US,
        session_store: impl SessionStore + Sync + Send + Clone + 'static,
        signer: SignedUrlRepo,
    ) -> anyhow::Result<()>
    where
        ES: EventService + Sync + Send + Clone + 'static,
        AS: ArtistService + Sync + Send + Clone + 'static,
        US: UserService + Sync + Send + Clone + 'static,
    {
        let app_state = AppState::new(event_service, artist_service, user_service);
        let app_data = web::Data::new(app_state);

        let url_repo = web::Data::new(signer);
        let secret_key = if fs::exists("./cookie_secret")? {
            info!("Loading cookie secret from file");
            let secret_key_bytes =
                fs::read("./cookie_secret").context("Failed to read cookie secret from file")?;
            if secret_key_bytes.len() != 64 {
                anyhow::bail!(
                    "Invalid cookie secret length: expected 64 bytes, got {}",
                    secret_key_bytes.len()
                );
            }
            Key::from(&secret_key_bytes)
        } else {
            let secret_key = Key::generate();
            fs::write("./cookie_secret", secret_key.master())
                .context("Failed to write cookie secret to file")?;
            info!("Generated new cookie secret and saved to file");
            secret_key
        };

        let helmet = Helmet::default()
            .into_middleware()
            .expect("Failed to create Helmet middleware");

        info!("Starting HTTP server");
        actix_web::HttpServer::new(move || {
            let identity_middleware = IdentityMiddleware::builder()
                .visit_deadline(Some(Duration::from_hours(24 * 7)))
                .build();

            App::new()
                .app_data(app_data.clone())
                .app_data(url_repo.clone())
                .wrap(helmet.clone())
                .wrap(ErrorHandlers::new().handler(StatusCode::UNAUTHORIZED, handle_unauthorized))
                .wrap(identity_middleware)
                .wrap(SessionMiddleware::new(
                    session_store.clone(),
                    secret_key.clone(),
                ))
                .wrap(TracingLogger::default())
                .wrap(HtmxMiddleware)
                // .service(web::scope("/api").configure(controller::api_v1))
                // .service(Files::new("/static", "./dist/static"))
                .configure(handlers::configure::<ES, AS, US>)
                .wrap(Compress::default())
        })
        .bind(("0.0.0.0", 8090))?
        .run()
        .await
        .context("Failed to run HTTP server")
    }
}

fn handle_unauthorized<B>(
    mut res: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, _) = res.into_parts();

    let res = ServiceResponse::new(
        req,
        HttpResponse::Found()
            .append_header((header::LOCATION, "/login"))
            .finish(),
    )
    .map_into_boxed_body()
    .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
