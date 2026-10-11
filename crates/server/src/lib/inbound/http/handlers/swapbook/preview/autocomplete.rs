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
        input::{Search, TextField},
        select::Select,
    },
    component::{Autocomplete, AutocompleteItem},
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
    let option = AutocompleteItem::builder()
        .title("Option 1")
        .subtitle("Subtitle 1")
        .selected_url("/option1")
        .build();
    Autocomplete::builder()
        .name("autocomplete")
        .placeholder(query.placeholder.clone())
        .default_items(vec![option])
        .search_url("/search")
        .build()
        .render()
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "autocomplete",
        name: "Autocomplete",
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
