use actix_web::{
    HttpResponse, Responder, get,
    web::{self},
};
use es_entity::prelude::serde_json;
use serde::Serialize;

pub mod autocomplete;
pub mod button;
pub mod card;
pub mod chip;
pub mod event_card;
pub mod input;
pub mod nav;
pub mod select;
pub mod step_indicator;
pub mod tag;
pub mod tombstone;
pub mod typography;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg // Configure the routes for each component
        .service(web::scope("/autocomplete").configure(autocomplete::configure))
        .service(web::scope("/button").configure(button::configure))
        .service(web::scope("/card").configure(card::configure))
        .service(web::scope("/chip").configure(chip::configure))
        .service(web::scope("/event_card").configure(event_card::configure))
        .service(web::scope("/input").configure(input::configure))
        .service(web::scope("/nav").configure(nav::configure))
        .service(web::scope("/select").configure(select::configure))
        .service(web::scope("/step_indicator").configure(step_indicator::configure))
        .service(web::scope("/tag").configure(tag::configure))
        .service(web::scope("/tombstone").configure(tombstone::configure))
        .service(web::scope("/typography").configure(typography::configure));
}
