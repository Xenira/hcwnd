use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _, html};
use serde::{Deserialize, Serialize};
use ui::atom::{
    input::{Search, TextField},
    select::Select,
};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(single);
}

#[derive(Deserialize)]
struct SelectParams {
    placeholder: String,
}

#[get("/single")]
async fn single(query: Query<SelectParams>) -> Markup {
    Select::builder()
        .name("select")
        .placeholder(query.placeholder.clone())
        .options(vec![
            ("Option 1".to_string(), "option1".to_string()),
            ("Option 2".to_string(), "option2".to_string()),
            ("Option 3".to_string(), "option3".to_string()),
        ])
        .build()
        .render()
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "select",
        name: "Select",
        group: "Forms",
        variants: vec![Variant::Metadata {
            name: "single",
            controls: vec![Control {
                name: "placeholder",
                value: ControlValue::String {
                    default: Some("Enter text here"),
                },
            }],
            docs: None,
            play: vec![],
        }],
    }]
}
