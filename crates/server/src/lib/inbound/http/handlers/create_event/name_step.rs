use actix_htmx::Htmx;
use actix_web::{
    get,
    guard::{self, Any, Get, Post},
    post, routes,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use serde_qs::web::QsForm;
use ui::{
    event::create::{
        name_step::{EventCreateNameStep, EventCreateNameStepData},
        EventCreate,
    },
    index::{IndexRoute, UiComponent as _},
};

use crate::{
    domain::{
        artist::ports::ArtistService,
        event::ports::EventService,
        user::{models::user::User, ports::UserService},
    },
    inbound::http::handlers::index_markup,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(name_step_form).service(search);
}

#[routes]
#[get("")]
#[post("")]
async fn name_step_form(
    _: User,
    form: Option<QsForm<EventCreateNameStep>>,
    htmx: Htmx,
) -> impl Responder {
    let step = form.map(|f| f.into_inner()).unwrap_or_default();
    let body = if htmx.is_htmx {
        step.render_html()
    } else {
        index_markup(
            "Create Event",
            IndexRoute::CreateEvent(EventCreate::NameStep(step)),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/search")]
async fn search() -> impl Responder {
    "Not implemented yet"
}
