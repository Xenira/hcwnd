use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use maud::{html, Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::atom::button::{Button, ButtonClass, LinkButton};

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
    let primary = Button::builder().label(query.label.clone()).build();
    let secondary = Button::builder()
        .label(query.label.clone())
        .button_class(ButtonClass::Secondary)
        .build();
    let flat = Button::builder()
        .label(query.label.clone())
        .button_class(ButtonClass::Flat)
        .build();

    let link_primary = LinkButton::builder()
        .label(query.label.clone())
        .button_class(ButtonClass::Primary)
        .href("https://example.com")
        .build();
    let link_secondary = LinkButton::builder()
        .label(query.label.clone())
        .button_class(ButtonClass::Secondary)
        .href("https://example.com")
        .build();
    let link_flat = LinkButton::builder()
        .label(query.label.clone())
        .button_class(ButtonClass::Flat)
        .href("https://example.com")
        .build();

    html! {
        (primary)
        (secondary)
        (flat)
        hr;
        (link_primary)
        (link_secondary)
        (link_flat)
    }
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
