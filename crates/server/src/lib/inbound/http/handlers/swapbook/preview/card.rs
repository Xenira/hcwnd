use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{html, Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::{
    atom::{card::Card, chip::Chip},
    component::Icons,
};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(card);
}

#[derive(Deserialize)]
struct CardParams {
    heading: Option<String>,
    body: Option<String>,
    footer: Option<String>,
    image_url: Option<String>,
}

#[get("/card")]
async fn card(query: Query<CardParams>) -> Markup {
    Card::builder()
        .header_opt(query.heading.as_ref().map(|h| {
            html! {
                h3 { (h) }
            }
        }))
        .body_opt(query.body.as_ref().map(|b| {
            html! {
                p { (b) }
            }
        }))
        .footer_opt(query.footer.as_ref().map(|f| {
            html! {
                p { (f) }
            }
        }))
        .image_opt(query.image_url.clone())
        .build()
        .render()
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "card",
        name: "Card",
        group: "Atoms",
        variants: vec![Variant::Metadata {
            name: "card",
            controls: vec![
                Control {
                    name: "heading",
                    value: ControlValue::String {
                        default: Some("Cards against humanity"),
                    },
                },
                Control {
                    name: "body",
                    value: ControlValue::String {
                        default: Some(
                            "Cards against humanity is a party game for horrible people.",
                        ),
                    },
                },
                Control {
                    name: "footer",
                    value: ControlValue::String {
                        default: Some("Get it now!"),
                    },
                },
                Control {
                    name: "image_url",
                    value: ControlValue::String {
                        default: Some("https://picsum.photos/400/200"),
                    },
                },
            ],
            docs: None,
            play: vec![],
        }],
    }]
}
