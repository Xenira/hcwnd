use actix_htmx::Htmx;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use serde_qs::web::QsForm;
use ui::{
    event::create::{
        self,
        details_step::EventCreateDetailsStep,
        name_step::{self, EventCreateNameStep, EventCreateNameStepData},
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
    _: User,
    htmx: Htmx,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    let body = if htmx.is_htmx {
        form.render_html()
    } else {
        index_markup(
            "Create Event",
            IndexRoute::CreateEvent(EventCreate::DetailsStep(form.into_inner())),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}
