use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{html, Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::atom::input::{Search, TextField};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(text);
}

#[derive(Deserialize)]
struct TextParams {
    placeholder: String,
    value: Option<String>,
}

#[get("/text")]
async fn text(query: Query<TextParams>) -> Markup {
    html! {
        (TextField::builder()
            .name("text")
            .placeholder(query.placeholder.clone())
            .value_opt(query.value.clone())
            .build())
        (Search::builder()
            .name("search")
            .placeholder("Search...")
            .value_opt(query.value.clone())
            .build())
    }
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "input",
        name: "Inputs",
        group: "Atoms",
        variants: vec![Variant::Metadata {
            name: "text",
            controls: vec![
                Control {
                    name: "placeholder",
                    value: ControlValue::String {
                        default: Some("Enter text here"),
                    },
                },
                Control {
                    name: "value",
                    value: ControlValue::String { default: Some("") },
                },
            ],
            docs: None,
            play: vec![],
        }],
    }]
}
