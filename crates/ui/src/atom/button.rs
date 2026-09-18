use std::fmt::Display;

use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::component::Icons;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Button {
    label: String,
    #[builder(default)]
    icon: Option<Icons>,
    #[builder(default)]
    #[allow(clippy::struct_field_names)]
    button_class: ButtonClass,
    #[builder(default)]
    button_type: ButtonType,
}

impl Render for Button {
    fn render(&self) -> Markup {
        html! {
            button
                type=(self.button_type)
                class=(self.button_class.as_str())
            {
                (button_content(&self.label, self.icon.as_ref()))
            }
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct LinkButton {
    label: String,
    href: String,
    #[builder(default="main".to_string())]
    target: String,
    #[builder(default = true)]
    boost: bool,
    #[builder(default = true)]
    push_url: bool,
    #[builder(default)]
    icon: Option<Icons>,
    #[builder(default)]
    #[allow(clippy::struct_field_names)]
    button_class: ButtonClass,
}

impl Render for LinkButton {
    fn render(&self) -> Markup {
        html! {
            a
                role="button"
                href=(&self.href)
                class=(self.button_class.as_str())
                hx-boost=(self.boost)
                hx-target=(self.target)
                hx-push-url=(self.push_url)
            {
                (button_content(&self.label, self.icon.as_ref()))
            }
        }
    }
}

fn button_content(label: &str, icon: Option<&Icons>) -> Markup {
    let icon_markup = if let Some(i) = icon {
        crate::component::icon(i, None)
    } else {
        html! {}
    };

    html! {
        (icon_markup)
        @if !label.is_empty() {
            span {
                (label)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Button => write!(f, "button"),
            ButtonType::Submit => write!(f, "submit"),
            ButtonType::Reset => write!(f, "reset"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonClass {
    #[default]
    Primary,
    Secondary,
    Flat,
}

impl ButtonClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonClass::Primary => "primary",
            ButtonClass::Secondary => "secondary",
            ButtonClass::Flat => "flat",
        }
    }
}
