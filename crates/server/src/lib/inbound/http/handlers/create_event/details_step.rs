use actix_htmx::{Htmx, SwapType};
use actix_multipart::form::{MultipartForm, bytes::Bytes, tempfile::TempFile};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, ServiceConfig},
};
use api::{UiState, event::new::EventCreateDetailsStep};
use imgproxy::{ImageUrl, SignedUrlRepo};
use maud::html;
use serde::Deserialize;
use serde_qs::web::QsForm;
use ui::{
    atom::form::FormValidation,
    event::create::{
        self,
        name_step::{self},
    },
    view::{View as _, event::new::details::DetailsStep},
};

use crate::{
    domain::user::models::user::User,
    inbound::http::{AppState, user::UiStateExtractor},
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(details_step)
        .service(details_step_form)
        .service(validate_name)
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
    form: QsForm<ValidateNameForm>,
) -> impl Responder {
    DetailsStep::validate_name(&state, &form.name)
}

#[derive(Debug, MultipartForm)]
struct UploadImageForm {
    #[multipart(limit = "5MB")]
    image: Bytes,
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
    let url = app_state
        .event_service
        .upload_form_image(&form.image.data)
        .await
        .unwrap();

    let img = ImageUrl::new(&url);
    let href = image_repo.get(&img).unwrap();
    htmx.retarget("#preview img");
    htmx.reswap(SwapType::OuterHtml);
    html! { img src=(href) {} }
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
