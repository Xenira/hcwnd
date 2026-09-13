use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{html, Markup, Render as _};
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
    let main = Nav::builder()
        .entries(vec![
            NavEntry::builder().label("Home").href("/").build(),
            NavEntry::builder().label("About").href("/about").build(),
            NavEntry::builder()
                .label("Contact")
                .href("/contact")
                .alignment(NavEntryAlignment::Right)
                .build(),
        ])
        .build();
    let tab = Nav::builder()
        .entries(vec![
            NavEntry::builder()
                .label("Details")
                .href("/details")
                .active(true)
                .build(),
            NavEntry::builder()
                .label("Timetable")
                .href("/timetable")
                .build(),
            NavEntry::builder().label("Lineup").href("/lineup").build(),
            NavEntry::builder()
                .label("Pending Edits (8)")
                .href("/suggestions")
                .alignment(NavEntryAlignment::Right)
                .build(),
        ])
        .build();

    html! {
        (main)
        div role="tablist" {
            (tab)
        }
    }
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
