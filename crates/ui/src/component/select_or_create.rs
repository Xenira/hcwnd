use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;

use crate::{
    atom::button::{Button, ButtonClass, ButtonType},
    component::Autocomplete,
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct SelectOrCreate {
    name: String,
    placeholder: String,
    search_url: String,
    create_form: Markup,
    #[builder(default)]
    multiple: bool,
}

impl Render for SelectOrCreate {
    fn render(&self) -> Markup {
        let autocomplete = Autocomplete::builder()
            .name(self.name.clone())
            .placeholder(self.placeholder.clone())
            .search_url(self.search_url.clone())
            .build();
        let new_button = Button::builder()
            .label("Create new")
            .class(ButtonClass::Link)
            .build();
        html! {
            (autocomplete)
            (new_button)
        }
    }
}
