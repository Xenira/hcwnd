use actix_web::{
    HttpResponse, Responder, get,
    web::{self},
};
use es_entity::prelude::serde_json;
use serde::Serialize;

pub mod button;
pub mod card;
pub mod chip;
pub mod event_card;
pub mod input;
pub mod nav;
pub mod select;
pub mod tag;
pub mod typography;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/button").configure(button::configure))
        .service(web::scope("/chip").configure(chip::configure))
        .service(web::scope("/input").configure(input::configure))
        .service(web::scope("/typography").configure(typography::configure))
        .service(web::scope("/card").configure(card::configure))
        .service(web::scope("/event_card").configure(event_card::configure))
        .service(web::scope("/tag").configure(tag::configure))
        .service(web::scope("/nav").configure(nav::configure))
        .service(web::scope("/select").configure(select::configure));
}
