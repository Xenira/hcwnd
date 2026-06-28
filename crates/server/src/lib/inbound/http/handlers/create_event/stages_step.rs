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
        name_step,
        stage_step::{EventCreateStageStep, EventStage},
        EventCreate,
    },
    index::{IndexRoute, UiComponent as _},
    util::SwitchValue,
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
        .route("", web::post().to(stages_step_form::<ES, AS, US>))
        .route(
            ui::event::create::stage_step::ADD_STAGE_ROUTE,
            web::post().to(add_event_stage::<ES, AS, US>),
        )
        .route(
            &format!(
                "{}/{{index}}",
                ui::event::create::stage_step::REMOVE_STAGE_ROUTE
            ),
            web::post().to(remove_event_stage::<ES, AS, US>),
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

async fn stages_step_form<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateStageStep>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
    let mut form = form.into_inner();
    form.populate_stages();

    let body = if htmx.is_htmx {
        form.render_html()
    } else {
        index_markup(
            "Stages - Create Event",
            IndexRoute::CreateEvent(EventCreate::StagesStep(form)),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn add_event_stage<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateStageStep>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
    debug!("Adding stage to event {form:?}");
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
        stage.render(form.stages.len()).into_string()
    } else {
        let mut form = form.into_inner();
        form.stages.push(stage);
        index_markup(
            "Create Event",
            IndexRoute::CreateEvent(EventCreate::StagesStep(form)),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}
async fn remove_event_stage<ES, AS, US>(
    _: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateStageStep>,
    index: web::Path<usize>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
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
        let body = index_markup(
            "Create Event",
            IndexRoute::CreateEvent(EventCreate::StagesStep(form)),
            None,
        )
        .render_html();

        HttpResponse::Ok().content_type("text/html").body(body)
    }
}
