use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{html, Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::{atom::chip::Chip, component::Icons};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(headings).service(hgroup);
}

#[get("/headings")]
async fn headings() -> Markup {
    html! {
        h1 { "Heading 1" }
        h2 { "Heading 2" }
        h3 { "Heading 3" }
        h4 { "Heading 4" }
        h5 { "Heading 5" }
        h6 { "Heading 6" }
    }
}

#[get("/hgroup")]
async fn hgroup() -> Markup {
    html! {
    hgroup {
        h3 { "Heading 3" }
        p { "This is a paragraph under the heading group." }
    }
    }
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "typography",
        name: "Typography",
        group: "Atoms",
        variants: vec![
            Variant::Metadata {
                name: "headings",
                controls: vec![],
                docs: None,
                play: vec![],
            },
            Variant::Metadata {
                name: "hgroup",
                controls: vec![],
                docs: None,
                play: vec![],
            },
        ],
    }]
}
