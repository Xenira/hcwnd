use actix_htmx::Htmx;
use actix_web::{
    get, post,
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
    domain::{
        artist::ports::ArtistService,
        event::ports::EventService,
        user::{models::user::User, ports::UserService},
    },
    inbound::http::{handlers::index_markup, AppState},
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(redirect_to_name_step)
        .service(confirm_step_form);
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
async fn confirm_step_form(
    signer: web::Data<SignedUrlRepo>,
    user: User,
    form: QsForm<EventCreateConfirmStep>,
    htmx: Htmx,
) -> impl Responder {
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
    let user = user.into();

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
