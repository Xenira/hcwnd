use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _, html};
use serde::{Deserialize, Serialize};
use ui::{
    atom::input::{Search, TextField},
    component::{Icons, icon},
};

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
    let icon = icon(&Icons::Like, None);
    html! {
        form {
            (TextField::builder()
                .name("text")
                .label("Required Text Field")
                .placeholder(query.placeholder.clone())
                .value_opt(query.value.clone())
                .required(true)
                .build())
            section {
                header { (icon) "Optional Text Field" }
                (TextField::builder()
                    .name("text_optional")
                    .label("Optional Text Field")
                    .placeholder(query.placeholder.clone())
                    .value_opt(query.value.clone())
                    .required(false)
                    .build())
            }
            (Search::builder()
                .name("search")
                .placeholder("Search...")
                .value_opt(query.value.clone())
                .build())
        }
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
