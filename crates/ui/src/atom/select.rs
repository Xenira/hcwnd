use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Select {
    name: String,
    options: Vec<(String, String)>,
    #[builder(default)]
    placeholder: Option<String>,
    #[builder(default)]
    value: Option<String>,
}

impl Render for Select {
    fn render(&self) -> Markup {
        html! {
            select name=(self.name) {
                @if let Some(placeholder) = &self.placeholder {
                    option value="" { (placeholder) }
                }
                @for (value, label) in &self.options {
                    option
                        value=(value)
                        selected=[&self.value.as_ref().map(|v| v == value)]
                    {
                        (label)
                    }
                }
            }
        }
    }
}
