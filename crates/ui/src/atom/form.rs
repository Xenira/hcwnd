use std::fmt::Display;

use api::UiState;
use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;

use crate::component::{Icons, icon};

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
    target: String,
    #[builder(default = true)]
    boost: bool,
    #[builder(default = true)]
    push_url: bool,
}

impl Render for Form {
    fn render(&self) -> Markup {
        html! {
            form id=(self.id)
                action=(self.url)
                method=(self.method.to_string())
                hx-target=(self.target)
                hx-boost="true"
                hx-push-url="true"
            {
                @if let Some(header) = &self.header {
                    (header)
                }
                (self.content)

                div.form-actions {
                    @if let Some(back_action) = &self.back_action {
                        (back_action)
                    }

                    button type="submit" {
                        (self.submit_label)
                        @if let Some(i) = &self.submit_icon {
                            (icon(i, None))
                        }
                    }
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
