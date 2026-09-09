use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _, html};
use serde::{Deserialize, Serialize};
use ui::{
    atom::{
        card::Card,
        chip::Chip,
        nav::{Nav, NavEntry, NavEntryAlignment},
    },
    component::Icons,
};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(card);
}

#[get("/main")]
async fn card() -> Markup {
    Nav::builder()
        .entries(vec![
            NavEntry::builder().label("Home").href("/").build(),
            NavEntry::builder().label("About").href("/about").build(),
            NavEntry::builder()
                .label("Contact")
                .href("/contact")
                .alignment(NavEntryAlignment::Right)
                .build(),
        ])
        .build()
        .render()
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "nav",
        name: "Navigation",
        group: "Atoms",
        variants: vec![Variant::Metadata {
            name: "main",
            controls: vec![],
            docs: None,
            play: vec![],
        }],
    }]
}
