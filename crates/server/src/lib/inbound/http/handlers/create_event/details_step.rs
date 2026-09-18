use std::fs;

use actix_htmx::{Htmx, SwapType};
use actix_multipart::form::{bytes::Bytes, tempfile::TempFile, MultipartForm};
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use api::{event::new::EventCreateDetailsStep, UiState};
use imgproxy::{ImageUrl, SignedUrlRepo};
use log::info;
use maud::html;
use serde::Deserialize;
use serde_qs::web::QsForm;
use ui::{
    atom::form::FormValidation,
    component::EventCard,
    event::create::{
        self,
        name_step::{self},
    },
    view::{event::new::details::DetailsStep, View as _},
};

use crate::{
    domain::user::models::user::User,
    inbound::http::{user::UiStateExtractor, AppState},
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(details_step)
        .service(details_step_form)
        .service(validate_name)
        .service(validate_description)
        .service(upload_image);
}

/// User should not be able to access this step directly, so we redirect them to the first step of the flow
#[get("")]
async fn details_step(user: User, state: UiStateExtractor, htmx: Htmx) -> impl Responder {
    let details = EventCreateDetailsStep::default();
    let details_view = DetailsStep::builder().details_step(&details).build();

    let body = if htmx.is_htmx {
        details_view.render(&state)
    } else {
        details_view.full_page(&state)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}

#[derive(Deserialize, Debug)]
struct ValidateNameForm {
    name: String,
}

#[post("/validate_name")]
async fn validate_name(
    _: User,
    state: UiStateExtractor,
    image_repo: web::Data<SignedUrlRepo>,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    let card = EventCard::from_details_step(
        &state,
        &form,
        form.image_url
            .as_ref()
            .map(|url| image_repo.get(&ImageUrl::new(url)).unwrap()),
    );

    html! {
        (DetailsStep::validate_name(&state, &form.name.clone().unwrap_or_default()))
        hx-partial hx-target="#preview" {
            (card)
        }
    }
}

#[derive(Debug, MultipartForm)]
struct UploadImageForm {
    #[multipart(limit = "5MB")]
    image: TempFile,
}

#[post("/upload_image")]
async fn upload_image(
    _: User,
    state: UiStateExtractor,
    app_state: web::Data<AppState>,
    image_repo: web::Data<SignedUrlRepo>,
    htmx: Htmx,
    MultipartForm(form): MultipartForm<UploadImageForm>,
) -> impl Responder {
    info!("Received image upload request: {:?}", form.image);
    let bytes = fs::read(form.image.file.path()).unwrap();
    let url = app_state
        .event_service
        .upload_form_image(&bytes)
        .await
        .unwrap();

    let img = ImageUrl::new(&url);
    let href = image_repo.get(&img).unwrap();
    html! {
        (DetailsStep::image_success(&state, &url))
        hx-partial hx-target="#preview img" hx-swap="outerHTML"{
            img src=(href);
        }
    }
}

#[post("/validate_description")]
async fn validate_description(
    _: User,
    state: UiStateExtractor,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    DetailsStep::validate_description(&state, &form.description.clone().unwrap_or_default())
}

#[post("")]
async fn details_step_form(
    _: User,
    state: UiStateExtractor,
    htmx: Htmx,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    let details = form.into_inner();
    let details_view = DetailsStep::builder().details_step(&details).build();

    let body = if htmx.is_htmx {
        details_view.render(&state)
    } else {
        details_view.full_page(&state)
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(body.into_string())
}
