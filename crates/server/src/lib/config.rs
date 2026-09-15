use std::env;

use anyhow::Context;
use serde::Deserialize;
use url::Url;

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const VALKEY_URL_KEY: &str = "VALKEY_URL";
const SERVER_PORT_KEY: &str = "SERVER_PORT";
const IMAGE_SIGNING_SALT: &str = "URL_SIGNING_SALT";
const IMAGE_SIGNING_KEY: &str = "URL_SIGNING_KEY";

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub database_url: String,
    pub valkey_url: String,
    pub image_proxy: ImageProxyConfig,
    pub s3: S3Config,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct S3Config {
    pub endpoint: Url,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ImageProxyConfig {
    pub signing_key: String,
    pub signing_salt: String,
}
