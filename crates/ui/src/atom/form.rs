use std::fmt::Display;

use api::UiState;
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::{
    atom::button::{Button, ButtonType},
    component::{icon, Icons},
    htmx::HxEncoding,
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Form {
    id: String,
    url: String,
    content: Markup,
    submit_label: String,
    #[builder(default)]
    submit_icon: Option<Icons>,
    #[builder(default)]
    back_action: Option<BackAction>,
    #[builder(default)]
    method: Method,
    #[builder(default)]
    header: Option<Markup>,
    #[builder(default="main".to_string())]
    hx_target: String,
    #[builder(default = true)]
    hx_boost: bool,
    #[builder(default = true)]
    hx_push_url: bool,
    #[builder(default)]
    hx_encoding: Option<HxEncoding>,
}

impl Render for Form {
    fn render(&self) -> Markup {
        let submit_button = Button::builder()
            .label(&self.submit_label)
            .button_type(ButtonType::Submit)
            .icon_opt(self.submit_icon.clone())
            .build();

        html! {
            form id=(self.id)
                action=(self.url)
                method=(self.method.to_string())
                hx-target=(self.hx_target)
                hx-boost=(self.hx_boost)
                hx-push-url=(self.hx_push_url)
                hx-encoding=[self.hx_encoding.as_ref().map(ToString::to_string)]
            {
                @if let Some(header) = &self.header {
                    (header)
                }
                (self.content)

                footer {
                    @if let Some(back_action) = &self.back_action {
                        (back_action)
                    } @else {
                        span {}
                    }

                    (submit_button)
                }
            }
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct BackAction {
    url: String,
    label: String,
    #[builder(default)]
    icon: Option<Icons>,
}

impl Render for BackAction {
    fn render(&self) -> Markup {
        html! {
            button.secondary
                type="submit"
                formaction=(self.url)
                formnovalidate
            {
                @if let Some(i) = &self.icon {
                    (icon(i, None))
                }
                (self.label)
            }
        }
    }
}

#[derive(Default)]
pub enum Method {
    Get,
    #[default]
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "get"),
            Method::Post => write!(f, "post"),
        }
    }
}

#[derive(Debug)]
pub enum FormValidation {
    Invalid(String),
    Valid(Option<String>),
    Required(String),
    Warning(String),
    Info(String),
}

impl Render for FormValidation {
    fn render(&self) -> Markup {
        match self {
            FormValidation::Invalid(msg) => html! {
                span.error {
                    (icon(&Icons::ValidationError, None))
                    (msg)
                }
            },
            FormValidation::Valid(msg) => html! {
                span.success {
                    (icon(&Icons::ValidationSuccess, None))
                    @if let Some(msg) = msg  {
                        (msg)
                    }
                }
            },
            FormValidation::Required(msg) => html! {
                span.error {
                    (icon(&Icons::ValidationRequired, None))
                    (msg)
                }
            },
            FormValidation::Warning(msg) => html! {
                span.warning {
                    (icon(&Icons::ValidationWarning, None))
                    (msg)
                }
            },
            FormValidation::Info(msg) => html! {
                span.info {
                    (icon(&Icons::ValidationInfo, None))
                    (msg)
                }
            },
        }
    }
}
