use api::UiState;
use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::{
    atom::{
        card::Card,
        image::Image,
        tombstone::{Tombstone, Tombstoneable},
    },
    component::Fetchable,
};

pub struct TombstoneCard;

impl Render for TombstoneCard {
    fn render(&self) -> Markup {
        let body = html! {
            (Tombstone::ImageSquare)
            (Tombstone::Heading)
            (Tombstone::TextShort)
        };
        Card::builder().body(body).build().render()
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct InlineCard {
    name: String,
    city: String,
    country: String,
    #[builder(default)]
    image_url: Option<String>,
}

impl Render for InlineCard {
    fn render(&self) -> Markup {
        let image = self
            .image_url
            .as_ref()
            .map(|url| {
                Image::builder()
                    .src(url.clone())
                    .alt(self.name.clone())
                    .format(crate::atom::image::ImageFormat::Square)
                    .build()
                    .render()
            })
            .unwrap_or_default();

        let body = html! {
            (image)
            h5 {
                (self.name)
            }
            p {
                (self.city) ", " (self.country)
            }
        };

        Card::builder().body(body).build().render()
    }
}

impl Tombstoneable for InlineCard {
    fn tombstone() -> impl Render {
        TombstoneCard
    }
}

impl Fetchable<Uuid> for InlineCard {
    fn fetch_url(id: &Uuid) -> String {
        format!("/venue/{id}/inline_card")
    }
}
