use crate::prelude::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up_form);
}

#[get("/")]
async fn sign_up_form() -> impl Responder {
    "foo"
}
