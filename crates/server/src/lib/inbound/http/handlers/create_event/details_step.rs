use actix_htmx::Htmx;
use actix_web::{
    get,
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
    domain::{artist::ports::ArtistService, event::ports::EventService, user::ports::UserService},
    inbound::http::{handlers::index_markup, user::UserExtractor},
};

pub fn configure<ES, AS, US>(cfg: &mut ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.service(redirect_to_name_step)
        .route("", web::post().to(details_step_form::<ES, AS, US>));
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

async fn details_step_form<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    htmx: Htmx,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
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
