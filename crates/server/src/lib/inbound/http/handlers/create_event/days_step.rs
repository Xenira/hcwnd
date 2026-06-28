use actix_htmx::Htmx;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use log::{debug, info};
use serde_qs::web::QsForm;
use ui::{
    event::create::{
        self,
        days_step::{day_buttons, EventCreateDaysStep, EventDay},
        name_step, EventCreate,
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
        .route("", web::post().to(days_step_form::<ES, AS, US>))
        .route(
            ui::event::create::days_step::ADD_DAY_ROUTE,
            web::post().to(add_event_day::<ES, AS, US>),
        )
        .route(
            ui::event::create::days_step::REMOVE_DAY_ROUTE,
            web::post().to(remove_event_day::<ES, AS, US>),
        );
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

async fn days_step_form<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
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
            IndexRoute::CreateEvent(EventCreate::DaysStep(form.into_inner())),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn add_event_day<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
    debug!("Adding event day, current days: {:?}", form.days);
    let day = EventDay {
        day: form.days.len(),
        start_time: None,
        end_time: None,
    };
    let body = if htmx.is_htmx {
        day.render_html() + &day_buttons(form.days.len() + 1, true).render_html()
    } else {
        let mut form = form.into_inner();
        form.days.push(day);
        index_markup(
            "Create Event",
            IndexRoute::CreateEvent(EventCreate::DaysStep(form)),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}
async fn remove_event_day<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
    let body = if htmx.is_htmx {
        day_buttons(form.days.len() - 1, true).render_html()
    } else {
        let mut form = form.into_inner();
        form.days.pop();
        index_markup(
            "Create Event",
            IndexRoute::CreateEvent(EventCreate::DaysStep(form)),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}
