use api::UiState;
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::component::Icons;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct TextField {
    name: String,
    #[builder(default)]
    placeholder: Option<String>,
    #[builder(default)]
    value: Option<String>,
}

impl Render for TextField {
    fn render(&self) -> Markup {
        html! {
            input
                type="text"
                name=(self.name)
                placeholder=[&self.placeholder]
                value=[&self.value]
            {}
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Search {
    name: String,
    #[builder(default)]
    placeholder: Option<String>,
    #[builder(default)]
    value: Option<String>,
}

impl Render for Search {
    fn render(&self) -> Markup {
        html! {
            input
                type="search"
                name=(self.name)
                placeholder=[&self.placeholder]
                value=[&self.value]
            {}
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct ImageUpload {
    name: String,
    image_name: String,
    #[builder(default)]
    preferred_aspect_ratio: Option<(u32, u32)>,
}

impl ImageUpload {
    pub fn render(&self, state: &UiState) -> Markup {
        html! {
            input
                type="file"
                name=(self.name)
                accept="image/jpeg,image/png,image/webp";
        }
    }
}
