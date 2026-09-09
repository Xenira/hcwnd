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
            search {
                input
                    type="search"
                    name=(self.name)
                    placeholder=[&self.placeholder]
                    value=[&self.value]
                {}
            }
        }
    }
}
