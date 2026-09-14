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
    #[builder(default)]
    label: Option<String>,
    #[builder(setter(!strip_option,strip_bool))]
    required: bool,
    #[builder(default)]
    input_type: InputType,
    #[builder(setter(!strip_option,strip_bool))]
    hx_preserve: bool,
}

impl Render for TextField {
    fn render(&self) -> Markup {
        let input_type = match &self.input_type {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Url => "url",
            InputType::File { .. } => "file",
        };

        let accept = match &self.input_type {
            InputType::File { accept, .. } => accept,
            _ => &None,
        };
        let multiple = match &self.input_type {
            InputType::File { multiple, .. } => *multiple,
            _ => false,
        };

        let input = html! {
            input
                name=(self.name)
                type=(input_type)
                placeholder=[&self.placeholder]
                value=[&self.value]
                required[self.required]
                multiple[multiple]
                accept=[accept]
                hx-preserve[self.hx_preserve]
            {}
        };

        if let Some(label) = &self.label {
            html! {
                label {
                    (label)
                    (input)
                }
            }
        } else {
            input
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Email,
    Number,
    Url,
    File {
        accept: Option<String>,
        multiple: bool,
    },
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct ImageInput<'a> {
    name: String,
    ui_state: &'a UiState,
    kind_label: String,
    preferred_aspect_ratio_label: String,
    #[builder(default = "image/png,image/jpeg,image/webp".to_string())]
    accept: String,
    #[builder(default = "PNG, JPEG, WEBP".to_string())]
    file_type_label: String,
    #[builder(default = "5MB".to_string())]
    max_size_label: String,
    #[builder(setter(!strip_option,strip_bool))]
    multiple: bool,
    #[builder(default)]
    label: Option<String>,
}

impl Render for ImageInput<'_> {
    fn render(&self) -> Markup {
        let input = TextField::builder()
            .name(&self.name)
            .input_type(InputType::File {
                accept: Some(self.accept.clone()),
                multiple: self.multiple,
            })
            .required()
            .build();

        let input = html! {
            .file-container {
                (input)
                .prompt {
                    b {
                        (t!("input.file.title", kind = self.kind_label, locale = &self.ui_state.locale))
                    }
                    span {
                        (t!("input.file.details",
                            file_type = self.file_type_label,
                            max_size = self.max_size_label,
                            aspect_ratio = self.preferred_aspect_ratio_label,
                            locale = &self.ui_state.locale
                        ))
                    }
                }
            }
        };

        if let Some(label) = &self.label {
            html! {
                label {
                    (label)
                    (input)
                }
            }
        } else {
            input
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct TextArea {
    name: String,
    #[builder(default)]
    placeholder: Option<String>,
    #[builder(default = 10)]
    rows: usize,
    #[builder(default)]
    value: Option<String>,
    #[builder(setter(!strip_option,strip_bool))]
    autofocus: bool,
    #[builder(default)]
    label: Option<String>,
    #[builder(setter(!strip_option,strip_bool))]
    required: bool,
}

impl Render for TextArea {
    fn render(&self) -> Markup {
        let area = html! {
            textarea
                name=(self.name)
                placeholder=[&self.placeholder]
                rows=(self.rows)
                autofocus[self.autofocus]
                required[self.required]
            {
                @if let Some(value) = &self.value {
                    (value)
                }
            }
        };

        if let Some(label) = &self.label {
            html! {
                label {
                    (label)
                    (area)
                }
            }
        } else {
            area
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
