use actix_web::{
    HttpResponse, Responder, get,
    web::{self},
};
use es_entity::prelude::serde_json;
use serde::Serialize;

pub mod button;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/button").configure(button::configure));
}
