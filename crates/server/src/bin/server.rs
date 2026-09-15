use std::sync::Arc;

use actix_session::storage::RedisSessionStore;
use config::{Config, Environment, File};
use hcwnd::{
    domain::{artist, event, user},
    inbound::http::HttpServer,
    outbound::{pg::Pg, rustfs::RustFs},
};
use imgproxy::SignedUrlRepo;
use log::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config_builder = Config::builder().add_source(File::with_name(".hcwnd").required(false));

    #[cfg(debug_assertions)]
    let config_builder = config_builder.add_source(File::with_name(".hcwnd.dev").required(false));

    let config = config_builder
        .add_source(File::with_name(".hcwnd.local").required(false))
        .add_source(File::with_name(".aws/credentials").required(false))
        .add_source(Environment::with_prefix("HC").separator("__"))
        .build()?
        .try_deserialize::<hcwnd::config::Config>()?;

    let cfg = deadpool_redis::Config::from_url(config.valkey_url);
    let redis_pool: deadpool_redis::Pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis connection pool");
    let redis_session_store = RedisSessionStore::new_pooled(redis_pool)
        .await
        .expect("Failed to create Redis session store");

    info!("Setting up image url signing");
    let url_salt = config.image_proxy.signing_salt;
    let url_secret_key = config.image_proxy.signing_key;
    let signer = SignedUrlRepo::new(url_salt, url_secret_key, "image".to_string());

    let pg = Pg::new(&config.database_url).await?;

    let s3 = RustFs::new(&config.s3).await;

    let event_service =
        event::service::Service::new(pg.clone(), pg.clone(), pg.clone(), pg.clone(), s3.clone());
    let artist_service = artist::service::Service::new(pg.clone());
    let user_service = user::service::Service::new(pg.clone());

    info!("Starting HTTP server on port {}", config.server_port);
    HttpServer::run(
        Arc::new(event_service),
        Arc::new(artist_service),
        Arc::new(user_service),
        redis_session_store,
        signer,
    )
    .await
}
