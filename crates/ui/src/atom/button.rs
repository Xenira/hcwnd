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
    button_type: ButtonType,
}

impl Render for Button {
    fn render(&self) -> Markup {
        html! {
            button
                type="button"
                class=(self.button_type.as_str())
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
    button_type: ButtonType,
}

impl Render for LinkButton {
    fn render(&self) -> Markup {
        html! {
            a
                role="button"
                href=(&self.href)
                class=(self.button_type.as_str())
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
    Primary,
    Secondary,
    Flat,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Primary => "primary",
            ButtonType::Secondary => "secondary",
            ButtonType::Flat => "flat",
        }
    }
}
