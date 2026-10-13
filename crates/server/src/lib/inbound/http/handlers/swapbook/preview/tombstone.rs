use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Query},
};
use es_entity::prelude::serde_json;
use maud::{Markup, Render as _, html};
use serde::{Deserialize, Serialize};
use ui::{
    atom::{
        input::{Search, TextField},
        select::Select,
        tombstone::Tombstone,
    },
    component::{Autocomplete, AutocompleteItem},
};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(single);
}

#[get("/atom")]
async fn single() -> Markup {
    html! {
        "Text Short"
        (Tombstone::TextShort)
        "Text Medium"
        (Tombstone::TextMedium)
        "Text Long"
        (Tombstone::TextLong)
        "Text Full Width"
        (Tombstone::TextFullWidth)
        "Text Paragraph"
        (Tombstone::TextParagraph)
        "Avatar Small"
        (Tombstone::AvatarSmall)
        "Avatar Medium"
        (Tombstone::AvatarLarge)
        "Icon"
        (Tombstone::Icon)
        "Chip"
        (Tombstone::Chip)
        "Button"
        (Tombstone::Button)
        "Image Square"
        (Tombstone::ImageSquare)
        "Image Banner"
        (Tombstone::ImageBanner)
    }
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "tombstone",
        name: "Tombstones",
        group: "Atoms",
        variants: vec![Variant::Metadata {
            name: "atom",
            controls: vec![],
            docs: None,
            play: vec![],
        }],
    }]
}
