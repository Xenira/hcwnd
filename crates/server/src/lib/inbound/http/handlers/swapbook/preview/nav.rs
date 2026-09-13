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
        nav::{Nav, NavEntry, NavEntryAlignment, NavEntrySimple},
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
            NavEntrySimple::builder()
                .label("Home")
                .href("/")
                .build()
                .into(),
            NavEntrySimple::builder()
                .label("About")
                .href("/about")
                .build()
                .into(),
            NavEntrySimple::builder()
                .label("Contact")
                .href("/contact")
                .alignment(NavEntryAlignment::Right)
                .build()
                .into(),
        ])
        .build();
    let tab = Nav::builder()
        .entries(vec![
            NavEntrySimple::builder()
                .label("Details")
                .href("/details")
                .active(true)
                .build()
                .into(),
            NavEntrySimple::builder()
                .label("Timetable")
                .href("/timetable")
                .build()
                .into(),
            NavEntrySimple::builder()
                .label("Lineup")
                .href("/lineup")
                .build()
                .into(),
            NavEntrySimple::builder()
                .label("Pending Edits (8)")
                .href("/suggestions")
                .alignment(NavEntryAlignment::Right)
                .build()
                .into(),
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
