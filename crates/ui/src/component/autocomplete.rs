use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::atom::input::{InputType, TextField};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Autocomplete {
    name: String,
    placeholder: String,
    search_url: String,
    #[builder(default)]
    default_items: Vec<AutocompleteItem>,
}

impl Render for Autocomplete {
    fn render(&self) -> Markup {
        let search_input = TextField::builder()
            .name(self.name.clone())
            .placeholder(self.placeholder.clone())
            .validation_endpoint(self.search_url.clone())
            .input_type(InputType::Search)
            .build();
        html! {
            .autocomplete {
                (search_input)
                ul {
                    @for item in &self.default_items {
                        (item)
                    }
                }
            }
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct AutocompleteItem {
    selected_url: String,
    title: String,
    subtitle: Option<String>,
    #[builder(default)]
    image_url: Option<String>,
}

impl Render for AutocompleteItem {
    fn render(&self) -> Markup {
        html! {
            li
                hx-get=(self.selected_url.clone())
                hx-trigger="click"
                hx-target="closest .autocomplete"
                hx-swap="outerHTML"
                tabindex="0"
            {
                @if let Some(image_url) = &self.image_url {
                    img
                        src=(image_url)
                        alt=(self.title)
                    {}
                }
                hgroup {
                    h5 { (self.title) }
                    @if let Some(subtitle) = &self.subtitle {
                        small { (subtitle) }
                    }
                }
            }
        }
    }
}
