use actix_web::web;
use api::routes::venue::new as Routes;

pub mod inline;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(Routes::inline::SEG).configure(inline::configure));
    // cfg.service(web::scope(VenueNewRoute::PATH).configure(new::configure));
    // cfg.service(get_artists).service(artist_detail);
}
