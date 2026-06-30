use actix_htmx::Htmx;
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, ServiceConfig},
};
use imgproxy::{ImageUrl, ProcessingOption, ResizeMode, ResizingOptionsBuilder, SignedUrlRepo};
use serde_qs::web::QsForm;
use ui::event::create::{
    self,
    confirm_step::{self, EventCreateConfirmStep},
    name_step,
};

use crate::domain::user::models::user::User;

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
    let state = api::UiState::from(&user);
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

    form.signed_image_url = image_url.to_string();

    let body = if htmx.is_htmx {
        confirm_step::render(&state, &form)
    } else {
        confirm_step::full_page(&state, &form)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}
