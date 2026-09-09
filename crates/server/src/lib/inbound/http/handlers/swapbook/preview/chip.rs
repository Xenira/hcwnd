use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::{atom::chip::Chip, component::Icons};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(primary).service(icon);
}

#[derive(Deserialize)]
struct ChipParams {
    label: String,
    icon: Option<String>,
}

#[get("/simple")]
async fn primary(query: Query<ChipParams>) -> Markup {
    let chip = Chip::builder().label(query.label.clone()).build();

    chip.render()
}

#[get("/icon")]
async fn icon(query: Query<ChipParams>) -> Markup {
    let icon = match query.icon.as_deref() {
        Some("like") => Icons::Like,
        Some("date") => Icons::Date,
        _ => panic!("Unknown icon"),
    };
    let chip = Chip::builder()
        .label(query.label.clone())
        .icon(icon)
        .build();

    chip.render()
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "chip",
        name: "Chips",
        group: "Atoms",
        variants: vec![
            Variant::Metadata {
                name: "simple",
                controls: vec![Control {
                    name: "label",
                    value: ControlValue::String {
                        default: Some("Nomnomnom Chips"),
                    },
                }],
                docs: None,
                play: vec![],
            },
            Variant::Metadata {
                name: "icon",
                controls: vec![
                    Control {
                        name: "label",
                        value: ControlValue::String {
                            default: Some("Nomnomnom Chips"),
                        },
                    },
                    Control {
                        name: "icon",
                        value: ControlValue::Select {
                            default: Some("like"),
                            options: vec!["like", "date"],
                        },
                    },
                ],
                docs: None,
                play: vec![],
            },
        ],
    }]
}
