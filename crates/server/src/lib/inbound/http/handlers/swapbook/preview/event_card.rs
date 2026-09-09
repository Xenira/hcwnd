use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use chrono::Local;
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _, html};
use serde::{Deserialize, Serialize};
use ui::{
    atom::{card::Card, chip::Chip},
    component::{EventCard, Icons},
};
use uuid::Uuid;

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(card);
}

#[derive(Deserialize)]
struct EventCardParams {
    name: String,
    location: String,
    genres: String,
    image_url: String,
}

#[get("/card")]
async fn card(query: Query<EventCardParams>) -> Markup {
    let genres: Vec<String> = query
        .genres
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let one_day = EventCard::builder()
        .id(Uuid::nil())
        .name(&query.name)
        .location(&query.location)
        .start_time(Local::now())
        .end_time(Local::now() + chrono::Duration::hours(23))
        .image(&query.image_url)
        .genres(genres.clone())
        .build();

    let two_day = EventCard::builder()
        .id(Uuid::nil())
        .name(query.name.clone())
        .location(query.location.clone())
        .start_time(Local::now())
        .end_time(Local::now() + chrono::Duration::days(1))
        .image(query.image_url.clone())
        .genres(genres)
        .build();

    html! {
        (one_day)
        (two_day)
    }
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "event_card",
        name: "Event Card",
        group: "Components",
        variants: vec![Variant::Metadata {
            name: "card",
            controls: vec![
                Control {
                    name: "name",
                    value: ControlValue::String {
                        default: Some("Reverze"),
                    },
                },
                Control {
                    name: "location",
                    value: ControlValue::String {
                        default: Some("AFAS Dome, Antwerp Belgium"),
                    },
                },
                Control {
                    name: "genres",
                    value: ControlValue::String {
                        default: Some("Hardstyle, Hardcore, Hardtechno"),
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
