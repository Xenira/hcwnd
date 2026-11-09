use actix_web::{get, web, Responder};
use maud::{html, Render as _};
use ui::partial::venue::VenueInlineForm;

use crate::inbound::http::user::{RequestUser, UiStateExtractor};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_inline_form);
    // cfg.service(web::scope(VenueNewRoute::PATH).configure(new::configure));
    // cfg.service(get_artists).service(artist_detail);
}

#[get("")]
async fn get_inline_form(_: RequestUser, state: UiStateExtractor) -> impl Responder {
    VenueInlineForm::builder().state(&*state).build().render()
}
