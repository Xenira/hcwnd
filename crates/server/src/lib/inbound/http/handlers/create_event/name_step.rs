use actix_htmx::Htmx;
use actix_web::{
    guard::{self, Any, Get, Post},
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
    domain::{artist::ports::ArtistService, event::ports::EventService, user::ports::UserService},
    inbound::http::{handlers::index_markup, user::UserExtractor},
};

pub fn configure<ES, AS, US>(cfg: &mut ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.route(
        "",
        web::route()
            .guard(Any(Get()).or(Post()))
            .to(name_step_form::<ES, AS, US>),
    )
    .route("/search", web::get().to(search));
}

async fn name_step_form<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: Option<QsForm<EventCreateNameStep>>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
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

async fn search() -> impl Responder {
    "Not implemented yet"
}

async fn submit<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateNameStepData>,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
    "Not implemented yet"
}
