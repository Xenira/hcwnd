use actix_htmx::Htmx;
use actix_web::{get, post, web::ServiceConfig, HttpResponse, Responder};
use api::{event::new::EventCreateDetailsStep, UiState};
use serde_qs::web::QsForm;
use ui::{
    event::create::{
        self,
        name_step::{self},
    },
    view::{event::new::details::DetailsStep, View as _},
};

use crate::{domain::user::models::user::User, inbound::http::user::UiStateExtractor};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(details_step).service(details_step_form);
}

/// User should not be able to access this step directly, so we redirect them to the first step of the flow
#[get("")]
async fn details_step(user: User, state: UiStateExtractor, htmx: Htmx) -> impl Responder {
    let details = EventCreateDetailsStep::default();
    let details_view = DetailsStep::builder().details_step(&details).build();

    let body = if htmx.is_htmx {
        details_view.render(&state)
    } else {
        details_view.full_page(&state)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[post("")]
async fn details_step_form(
    _: User,
    state: UiStateExtractor,
    htmx: Htmx,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    let details = form.into_inner();
    let details_view = DetailsStep::builder().details_step(&details).build();

    let body = if htmx.is_htmx {
        details_view.render(&state)
    } else {
        details_view.full_page(&state)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}
