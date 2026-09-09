use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _, html};
use serde::{Deserialize, Serialize};
use ui::{
    atom::{chip::Chip, tag::Tag},
    component::Icons,
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
