use actix_web::web::{self, ServiceConfig};
use api::routes::{Route, RouteSegment as _, VenueNewRoute};

pub mod new;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(VenueNewRoute::PATH).configure(new::configure));
    // cfg.service(get_artists).service(artist_detail);
}
