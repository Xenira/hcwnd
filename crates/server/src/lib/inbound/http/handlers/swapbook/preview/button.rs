use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _};
use serde::{Deserialize, Serialize};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(primary);
}

#[derive(Deserialize)]
struct ButtonParams {
    label: String,
}

#[get("/primary")]
async fn primary(query: Query<ButtonParams>) -> Markup {
    let button = ui::atom::button::Button {
        label: query.label.clone(),
    };

    button.render()
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "button",
        name: "Button",
        group: "Atoms",
        variants: vec![Variant::Metadata {
            name: "primary",
            controls: vec![Control {
                name: "label",
                value: ControlValue::String {
                    default: Some("Click Me"),
                },
            }],
            docs: None,
            play: vec![],
        }],
    }]
}
