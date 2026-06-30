use actix_htmx::Htmx;
use actix_web::{
    get, post,
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
    domain::{
        artist::ports::ArtistService,
        event::ports::EventService,
        user::{models::user::User, ports::UserService},
    },
    inbound::http::handlers::index_markup,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(redirect_to_name_step)
        .service(days_step_form)
        .route(
            ui::event::create::days_step::ADD_DAY_ROUTE,
            web::post().to(add_event_day),
        )
        .route(
            ui::event::create::days_step::REMOVE_DAY_ROUTE,
            web::post().to(remove_event_day),
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

#[post("")]
async fn days_step_form(_: User, form: QsForm<EventCreateDaysStep>, htmx: Htmx) -> impl Responder {
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

async fn add_event_day(_: User, form: QsForm<EventCreateDaysStep>, htmx: Htmx) -> impl Responder {
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

async fn remove_event_day(
    _: User,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
) -> impl Responder {
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
