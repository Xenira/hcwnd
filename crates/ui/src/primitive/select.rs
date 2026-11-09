use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::primitive::form::FormElement;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Select {
    name: String,
    options: Vec<(String, String)>,
    #[builder(default)]
    placeholder: Option<String>,
    #[builder(default)]
    value: Option<String>,
    #[builder(default)]
    label: Option<String>,
    #[builder(setter(!strip_option,strip_bool))]
    required: bool,
    #[builder(default)]
    form_id: Option<String>,
}

impl FormElement for Select {
    fn set_form(&mut self, id: &str) {
        self.form_id = Some(id.to_string());
    }
}

impl Render for Select {
    fn render(&self) -> Markup {
        let select = html! {
            select
                name=(self.name)
                form=[&self.form_id]
                required[self.required]
            {
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
        };

        if let Some(label) = &self.label {
            html! {
                label {
                    (label)
                    (select)
                }
            }
        } else {
            select
        }
    }
}
