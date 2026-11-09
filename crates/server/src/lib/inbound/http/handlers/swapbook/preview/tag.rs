use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{html, Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::{
    component::Icons,
    primitive::{chip::Chip, tag::Tag},
};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(tags);
}

#[derive(Deserialize)]
struct TagParams {
    label: String,
}

#[get("/tags")]
async fn tags(query: Query<TagParams>) -> Markup {
    html! {
        (Tag::builder().label(query.label.clone()).build())

        @for i in 0..12 {
            (Tag::builder().label(format!("{i}: {}", query.label.clone())).index(i).build())
        }
    }
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "tag",
        name: "Tags",
        group: "Atoms",
        variants: vec![Variant::Metadata {
            name: "tags",
            controls: vec![Control {
                name: "label",
                value: ControlValue::String {
                    default: Some("Tag, you're it!"),
                },
            }],
            docs: None,
            play: vec![],
        }],
    }]
}
