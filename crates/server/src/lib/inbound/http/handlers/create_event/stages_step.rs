use actix_htmx::Htmx;
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, ServiceConfig},
};
use log::debug;
use serde_qs::web::QsForm;
use ui::{
    event::create::{
        self, name_step,
        stage_step::{EventCreateStageStep, EventStage},
    },
    util::SwitchValue,
};

use crate::domain::user::models::user::User;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(redirect_to_name_step)
        .service(stages_step_form)
        .route(
            ui::event::create::stage_step::ADD_STAGE_ROUTE,
            web::post().to(add_event_stage),
        )
        .route(
            &format!(
                "{}/{{index}}",
                ui::event::create::stage_step::REMOVE_STAGE_ROUTE
            ),
            web::post().to(remove_event_stage),
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
async fn stages_step_form(
    user: User,
    form: QsForm<EventCreateStageStep>,
    htmx: Htmx,
) -> impl Responder {
    let state = api::UiState::from(&user);

    let mut form = form.into_inner();
    form.populate_stages();

    let body = if htmx.is_htmx {
        ui::event::create::stage_step::render(&state, &form)
    } else {
        ui::event::create::stage_step::full_page(&state, &form)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

async fn add_event_stage(
    user: User,
    form: QsForm<EventCreateStageStep>,
    htmx: Htmx,
) -> impl Responder {
    debug!("Adding stage to event {form:?}");
    let state = api::UiState::from(&user);

    let stage = EventStage {
        name: String::new(),
        days: form
            .days
            .iter()
            .enumerate()
            .map(|(i, _)| (i, SwitchValue(true)))
            .collect(),
    };
    let body = if htmx.is_htmx {
        ui::event::create::stage_step::render_stage(&state.locale, &stage, form.stages.len())
    } else {
        let mut form = form.into_inner();
        form.stages.push(stage);
        ui::event::create::stage_step::full_page(&state, &form)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

async fn remove_event_stage(
    user: User,
    form: QsForm<EventCreateStageStep>,
    index: web::Path<usize>,
    htmx: Htmx,
) -> impl Responder {
    let state = api::UiState::from(&user);

    if form.stages.len() <= 1 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Cannot remove the last stage");
    }

    if htmx.is_htmx {
        HttpResponse::Ok().finish()
    } else {
        let mut form = form.into_inner();
        form.stages.remove(*index);

        let body = ui::event::create::stage_step::full_page(&state, &form);

        HttpResponse::Ok()
            .content_type("text/html")
            .body(body.into_string())
    }
}
