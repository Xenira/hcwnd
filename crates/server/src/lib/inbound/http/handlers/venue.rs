use actix_web::web::{self, ServiceConfig};
use api::routes::venue as Routes;

pub mod new;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(Routes::new::SEG).configure(new::configure));
}
