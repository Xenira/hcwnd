use actix_htmx::Htmx;
use actix_web::{HttpResponse, Responder, get, routes, web::ServiceConfig};
use serde_qs::web::QsForm;
use ui::event::create::name_step::{self, EventCreateNameStep};

use crate::domain::user::models::user::User;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(name_step_form).service(search);
}

#[routes]
#[get("")]
#[post("")]
async fn name_step_form(
    user: User,
    form: Option<QsForm<EventCreateNameStep>>,
    htmx: Htmx,
) -> impl Responder {
    let state = api::UiState::from(&user);
    let step = form.map(|f| f.into_inner()).unwrap_or_default();
    let body = if htmx.is_htmx {
        name_step::render(&state, &step)
    } else {
        name_step::full_page(&state, &step)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[get("/search")]
async fn search() -> impl Responder {
    "Not implemented yet"
}
