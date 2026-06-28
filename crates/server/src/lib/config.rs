use std::env;

use anyhow::Context;

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const VALKEY_URL_KEY: &str = "VALKEY_URL";
const SERVER_PORT_KEY: &str = "SERVER_PORT";
const IMAGE_SIGNING_SALT: &str = "URL_SIGNING_SALT";
const IMAGE_SIGNING_KEY: &str = "URL_SIGNING_KEY";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub database_url: String,
    pub valkey_url: String,
    pub image_signing_salt: String,
    pub image_signing_key: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let server_port = load_env(SERVER_PORT_KEY)?;
        let database_url = load_env(DATABASE_URL_KEY)?;
        let valkey_url = load_env(VALKEY_URL_KEY)?;
        let image_signing_salt = load_env(IMAGE_SIGNING_SALT)?;
        let image_signing_key = load_env(IMAGE_SIGNING_KEY)?;

        Ok(Config {
            server_port,
            database_url,
            valkey_url,
            image_signing_salt,
            image_signing_key,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
