use actix_htmx::Htmx;
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, ServiceConfig},
};
use log::debug;
use serde_qs::web::QsForm;
use ui::event::create::{
    self,
    days_step::{EventCreateDaysStep, EventDay, day_buttons},
    name_step,
};

use crate::domain::user::models::user::User;

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
async fn days_step_form(
    user: User,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
) -> impl Responder {
    let state = api::UiState::from(&user);

    let body = if htmx.is_htmx {
        ui::event::create::days_step::render(&state, &form.into_inner())
    } else {
        ui::event::create::days_step::full_page(&state, &form.into_inner())
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

async fn add_event_day(
    user: User,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
) -> impl Responder {
    debug!("Adding event day, current days: {:?}", form.days);
    let state = api::UiState::from(&user);

    let day = EventDay {
        day: form.days.len(),
        start_time: None,
        end_time: None,
    };

    let body = if htmx.is_htmx {
        ui::event::create::days_step::render_day(&state.locale, &day, Some(form.days.len() + 1))
    } else {
        let mut form = form.into_inner();
        form.days.push(day);
        ui::event::create::days_step::full_page(&state, &form)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

async fn remove_event_day(
    user: User,
    form: QsForm<EventCreateDaysStep>,
    htmx: Htmx,
) -> impl Responder {
    let state = api::UiState::from(&user);

    let body = if htmx.is_htmx {
        day_buttons(&state.locale, form.days.len() - 1, true)
    } else {
        let mut form = form.into_inner();
        form.days.pop();
        ui::event::create::days_step::full_page(&state, &form)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}
