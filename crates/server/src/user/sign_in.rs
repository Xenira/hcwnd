use crate::prelude::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in_form);
}

#[get("/")]
async fn sign_in_form() -> impl Responder {
    "foo"
}
