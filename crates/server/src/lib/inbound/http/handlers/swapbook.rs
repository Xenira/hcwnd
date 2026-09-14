use actix_web::{
    HttpResponse, Responder, get,
    web::{self},
};
use es_entity::prelude::serde_json;
use serde::Serialize;

pub mod preview;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(manifest)
        .service(web::scope("/preview").configure(preview::configure));
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    htmx_src: &'static str,
    css_src: &'static str,
    js_src: Option<&'static str>,
    viewports: Vec<Viewport>,
    stories: Vec<Story>,
}

#[derive(Serialize)]
struct Viewport {
    name: &'static str,
    w: &'static str,
}

#[derive(Serialize)]
pub struct Story {
    id: &'static str,
    name: &'static str,
    group: &'static str,
    variants: Vec<Variant>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Variant {
    Name(&'static str),
    Metadata {
        name: &'static str,
        controls: Vec<Control>,
        docs: Option<&'static str>,
        play: Vec<Step>,
    },
}

#[derive(Serialize)]
pub struct Control {
    name: &'static str,
    #[serde(flatten)]
    value: ControlValue,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ControlValue {
    String {
        default: Option<&'static str>,
    },
    Number {
        default: Option<f64>,
    },
    Boolean {
        default: Option<bool>,
    },
    Select {
        default: Option<&'static str>,
        options: Vec<&'static str>,
    },
}

#[derive(Serialize)]
pub struct Step {
    action: &'static str,
    target: Option<&'static str>,
    value: Option<&'static str>,
    text: Option<&'static str>,
}

#[get("/manifest.json")]
async fn manifest() -> impl Responder {
    let stories = preview::button::stories()
        .into_iter()
        .chain(preview::chip::stories().into_iter())
        .chain(preview::input::stories().into_iter())
        .chain(preview::typography::stories().into_iter())
        .chain(preview::card::stories().into_iter())
        .chain(preview::event_card::stories().into_iter())
        .chain(preview::tag::stories().into_iter())
        .chain(preview::nav::stories().into_iter())
        .chain(preview::select::stories().into_iter())
        .chain(preview::step_indicator::stories().into_iter())
        .collect();

    let manifest = Manifest {
        htmx_src: "/assets/htmx.min.js",
        css_src: "/assets/style.css",
        js_src: None,
        viewports: vec![],
        stories: stories,
    };

    HttpResponse::Ok().json(manifest)
}
