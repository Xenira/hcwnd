// #![warn(clippy::all, clippy::pedantic)]
// #![deny(clippy::unwrap_used, unsafe_code)]
//
// use std::{env, time::Duration};
//
// use actix_files::Files;
// use actix_htmx::HtmxMiddleware;
// use actix_identity::IdentityMiddleware;
// use actix_session::{storage::RedisSessionStore, SessionMiddleware};
// use actix_web::{
//     cookie::Key, get, middleware::Compress, web, App, HttpResponse, HttpServer, Responder,
// };
// use actix_web_helmet::Helmet;
// use imgproxy::SignedUrlRepo;
// use log::{info, warn};
// use sqlx::PgPool;
// use tracing_actix_web::TracingLogger;
// use ui::{
//     event::list::EventListBuilder,
//     index::{IndexRoute, UiComponent as _},
// };
//
// use crate::{
//     controller::EventRepoData,
//     entity::{act::ActRepo, event::EventRepo, stage::StageRepo},
//     service::event::list_events,
// };
//
// mod controller;
// // mod entity;
// mod error;
// mod event;
// mod event_req_cache;
// mod model;
// mod service;
// mod user;
//
// pub mod prelude {
//     pub use crate::controller::{ActRepoData, DayRepoData, EventRepoData, StageRepoData};
//     pub use crate::error::Result;
//     pub use actix_htmx::Htmx;
//     pub use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
//     pub use itertools::Itertools as _;
//     pub use ui::index::UiComponent as _;
//     pub use uuid::Uuid;
// }
//
// pub type PostgresPool = web::Data<PgPool>;
//
// #[get("/")]
// async fn index(
//     event_repo: EventRepoData,
//     image_repo: web::Data<SignedUrlRepo>,
// ) -> crate::error::Result<impl Responder> {
//     let list = EventListBuilder::default()
//         .events(list_events(&event_repo, &image_repo, 1, 12).await?)
//         .page(1)
//         .has_more(false)
//         .build()
//         .expect("Failed to build event list");
//     let route = IndexRoute::Home(list);
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(service::index("Events", route).render_html()))
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();
//
//     info!("Connecting to database");
//     let pg_pool = PgPool::connect("postgresql://postgres:example@localhost:5432/timetable")
//         .await
//         .expect("Failed to connect to database");
//
//     info!("Running database migrations");
//     sqlx::migrate!("./migrations")
//         .run(&pg_pool)
//         .await
//         .expect("Failed to run database migrations");
//
//     let event_repo = web::Data::new(EventRepo {
//         pool: pg_pool.clone(),
//     });
//     let stage_repo = web::Data::new(StageRepo {
//         pool: pg_pool.clone(),
//     });
//     let act_repo = web::Data::new(ActRepo {
//         pool: pg_pool.clone(),
//     });
//     let day_repo = web::Data::new(entity::day::DaysRepo {
//         pool: pg_pool.clone(),
//     });
//     let pg_pool = web::Data::new(pg_pool);
//
//     info!("Setting up session store");
//     let secret_key = Key::generate();
//     let cfg = deadpool_redis::Config::from_url(
//         env::var("REDIS_URL").expect("REDIS_URL environment variable not set"),
//     );
//     let redis_pool: deadpool_redis::Pool = cfg
//         .create_pool(Some(deadpool_redis::Runtime::Tokio1))
//         .expect("Failed to create Redis connection pool");
//     let redis_session_store = RedisSessionStore::new_pooled(redis_pool)
//         .await
//         .expect("Failed to create Redis session store");
//
//     info!("Setting up image url signing");
//     let url_salt =
//         env::var("URL_SIGNING_SALT").expect("URL_SIGNING_SALT environment variable not set");
//     let url_secret_key = env::var("URL_SIGNING_SECRET_KEY")
//         .expect("URL_SIGNING_SECRET_KEY environment variable not set");
//     let signer = SignedUrlRepo::new(
//         url_salt.clone(),
//         url_secret_key.clone(),
//         "image".to_string(),
//     );
//
//     let url_repo = web::Data::new(signer);
//
//     let helmet = Helmet::default()
//         .into_middleware()
//         .expect("Failed to create Helmet middleware");
//
//     info!("Starting HTTP server");
//     HttpServer::new(move || {
//         let identity_middleware = IdentityMiddleware::builder()
//             .visit_deadline(Some(Duration::from_hours(24 * 7)))
//             .build();
//
//         App::new()
//             .app_data(pg_pool.clone())
//             .app_data(event_repo.clone())
//             .app_data(stage_repo.clone())
//             .app_data(act_repo.clone())
//             .app_data(day_repo.clone())
//             .app_data(url_repo.clone())
//             .wrap(helmet.clone())
//             .wrap(identity_middleware)
//             .wrap(SessionMiddleware::new(
//                 redis_session_store.clone(),
//                 secret_key.clone(),
//             ))
//             .wrap(TracingLogger::default())
//             .wrap(HtmxMiddleware)
//             .service(web::scope("/api").configure(controller::api_v1))
//             .service(Files::new("/static", "./dist/static"))
//             .service(index)
//             .service(web::scope("/event").configure(event::configure))
//             .wrap(Compress::default())
//     })
//     .bind(("0.0.0.0", 8090))?
//     .run()
//     .await
// }
