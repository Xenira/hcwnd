use std::{fs, ops::Deref};

use actix_htmx::{Htmx, SwapType};
use actix_multipart::form::{bytes::Bytes, tempfile::TempFile, MultipartForm};
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use api::routes::festival::new::details_step as Route;
use api::{festival::new::EventCreateDetailsStep, UiState};
use domain::user::models::user::User;
use imgproxy::{ImageUrl, SignedUrlRepo};
use log::info;
use maud::{html, Render as _};
use serde::Deserialize;
use serde_qs::web::QsForm;
use ui::{
    component::EventCard,
    event::create::{
        self,
        name_step::{self},
    },
    partial::venue::VenueInlineForm,
    primitive::form::FormValidation,
    view::{festival::new::details::DetailsStep, View as _},
};

use crate::inbound::http::{
    user::{RequestUser, UiStateExtractor},
    AppState,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(Route::form::SEG, web::get().to(details_step))
        .route(Route::validate_name::SEG, web::post().to(validate_name))
        .route(
            Route::validate_description::SEG,
            web::post().to(validate_description),
        )
        .route(Route::upload_image::SEG, web::post().to(upload_image));
    //     .service(details_step_form)
    // .service(validate_name)
    // .service(validate_description)
    // .service(upload_image);
}

async fn details_step(_: RequestUser, state: UiStateExtractor, htmx: Htmx) -> impl Responder {
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

async fn validate_name(
    _: RequestUser,
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

#[get("/venue/new")]
async fn new_venue(_: RequestUser, state: UiStateExtractor) -> impl Responder {
    VenueInlineForm::builder()
        .state(state.deref())
        .build()
        .render()
}

#[derive(Debug, MultipartForm)]
struct UploadImageForm {
    #[multipart(limit = "5MB")]
    image: TempFile,
}

async fn upload_image(
    _: RequestUser,
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

async fn validate_description(
    _: RequestUser,
    state: UiStateExtractor,
    form: QsForm<EventCreateDetailsStep>,
) -> impl Responder {
    DetailsStep::validate_description(&state, &form.description.clone().unwrap_or_default())
}

// #[post("")]
// async fn details_step_form(
//     _: RequestUser,
//     state: UiStateExtractor,
//     htmx: Htmx,
//     form: QsForm<EventCreateDetailsStep>,
// ) -> impl Responder {
//     let details = form.into_inner();
//     let details_view = DetailsStep::builder().details_step(&details).build();
//
//     let body = if htmx.is_htmx {
//         details_view.render(&state)
//     } else {
//         details_view.full_page(&state)
//     };
//
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(body.into_string())
// }
