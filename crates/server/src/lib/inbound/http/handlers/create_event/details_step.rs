use actix_htmx::Htmx;
use actix_web::{HttpResponse, Responder, get, post, web::ServiceConfig};
use serde_qs::web::QsForm;
use ui::event::create::{
    self,
    details_step::EventCreateDetailsStep,
    name_step::{self},
};

use crate::domain::user::models::user::User;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(redirect_to_name_step)
        .service(details_step_form);
}

/// User should not be able to access this step directly, so we redirect them to the first step of the flow
#[get("")]
async fn redirect_to_name_step() -> impl Responder {
    HttpResponse::Found()
        .append_header((
            "Location",
            format!("{}{}", create::BASE_ROUTE, name_step::BASE_ROUTE),
        ))
        .finish()
}

#[post("")]
async fn details_step_form(
    user: User,
    htmx: Htmx,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    let state = api::UiState::from(&user);
    let body = if htmx.is_htmx {
        ui::event::create::details_step::render(&state, &form.into_inner())
    } else {
        ui::event::create::details_step::full_page(&state, &form.into_inner())
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}
