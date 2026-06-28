use actix_session::storage::RedisSessionStore;
use hcwnd::{
    config::Config,
    domain::{artist, event, user},
    inbound::http::HttpServer,
    outbound::pg::Pg,
};
use imgproxy::SignedUrlRepo;
use log::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = Config::from_env()?;
    let cfg = deadpool_redis::Config::from_url(config.valkey_url);
    let redis_pool: deadpool_redis::Pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis connection pool");
    let redis_session_store = RedisSessionStore::new_pooled(redis_pool)
        .await
        .expect("Failed to create Redis session store");

    info!("Setting up image url signing");
    let url_salt = config.image_signing_salt;
    let url_secret_key = config.image_signing_key;
    let signer = SignedUrlRepo::new(
        url_salt.clone(),
        url_secret_key.clone(),
        "image".to_string(),
    );

    let pg = Pg::new(&config.database_url).await?;
    let event_service =
        event::service::Service::new(pg.clone(), pg.clone(), pg.clone(), pg.clone());
    let artist_service = artist::service::Service::new(pg.clone());
    let user_service = user::service::Service::new(pg.clone());

    info!("Starting HTTP server on port {}", config.server_port);
    HttpServer::run(
        event_service,
        artist_service,
        user_service,
        redis_session_store,
        signer,
    )
    .await
}
