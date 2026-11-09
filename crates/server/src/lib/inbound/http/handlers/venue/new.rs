use actix_web::web;
use api::routes::{RouteSegment, VenueNewInlineRoute};

pub mod inline;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(VenueNewInlineRoute::PATH).configure(inline::configure));
    // cfg.service(web::scope(VenueNewRoute::PATH).configure(new::configure));
    // cfg.service(get_artists).service(artist_detail);
}
