use actix_htmx::Htmx;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use imgproxy::{
    ImageUrl, ProcessingOption, ResizeMode, ResizingOptions, ResizingOptionsBuilder, SignedUrlRepo,
};
use log::{debug, info};
use serde_qs::web::QsForm;
use ui::{
    event::create::{
        self,
        confirm_step::EventCreateConfirmStep,
        days_step::{day_buttons, EventCreateDaysStep, EventDay},
        name_step,
        stage_step::EventCreateStageStep,
        EventCreate,
    },
    index::{IndexRoute, UiComponent as _},
};
use url::Url;

use crate::{
    domain::{artist::ports::ArtistService, event::ports::EventService, user::ports::UserService},
    inbound::http::{handlers::index_markup, user::UserExtractor, AppState},
};

pub fn configure<ES, AS, US>(cfg: &mut ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.service(redirect_to_name_step)
        .route("", web::post().to(confirm_step_form::<ES, AS, US>));
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

async fn confirm_step_form<ES, AS, US>(
    signer: web::Data<SignedUrlRepo>,
    user: UserExtractor<ES, AS, US>,
    form: QsForm<EventCreateConfirmStep>,
    htmx: Htmx,
) -> impl Responder
where
    ES: EventService,
    AS: ArtistService,
    US: UserService,
{
    let mut form = form.into_inner();
    let image_url = ImageUrl::new(form.image_url.clone()).with_option(ProcessingOption::Resize(
        ResizingOptionsBuilder::default()
            .width(1158)
            .height(650)
            .mode(ResizeMode::Auto)
            .build()
            .unwrap_or_default(),
    ));

    let image_url = &signer.get(&image_url).expect("Failed to sign image URL");
    let user = user.user.into();

    form.signed_image_url = image_url.to_string();

    let body = if htmx.is_htmx {
        form.render(user).into_string()
    } else {
        index_markup(
            "Stages - Create Event",
            IndexRoute::CreateEvent(EventCreate::ConfirmStep(form, user)),
            None,
        )
        .render_html()
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}
