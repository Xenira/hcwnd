use itertools::Itertools;
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::component::{icon, Icons};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Nav {
    entries: Vec<NavEntry>,
}

impl Render for Nav {
    fn render(&self) -> Markup {
        let left = self
            .entries
            .iter()
            .filter(|e| matches!(e.alignment(), NavEntryAlignment::Left))
            .collect_vec();
        let right = self
            .entries
            .iter()
            .filter(|e| matches!(e.alignment(), NavEntryAlignment::Right))
            .collect_vec();

        html! {
            nav {
                @if !left.is_empty() {
                    ul {
                        @for entry in left {
                            li { (entry) }
                        }
                    }
                }
                @if !right.is_empty() {
                    ul class="right" {
                        @for entry in right {
                            li { (entry) }
                        }
                    }
                }
            }
        }
    }
}

pub enum NavEntry {
    Simple(NavEntrySimple),
    Dropdown(NavEntryDropdown),
}

impl Render for NavEntry {
    fn render(&self) -> Markup {
        match self {
            NavEntry::Simple(entry) => entry.render(),
            NavEntry::Dropdown(entry) => entry.render(),
        }
    }
}

impl NavEntry {
    pub fn alignment(&self) -> &NavEntryAlignment {
        match self {
            NavEntry::Simple(entry) => &entry.alignment,
            NavEntry::Dropdown(entry) => &entry.alignment,
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct NavEntrySimple {
    label: String,
    href: String,
    #[builder(default)]
    icon: Option<Icons>,
    #[builder(default)]
    alignment: NavEntryAlignment,
    #[builder(default)]
    active: bool,
    #[builder(default="main".to_string())]
    target: String,
}

impl From<NavEntrySimple> for NavEntry {
    fn from(entry: NavEntrySimple) -> Self {
        NavEntry::Simple(entry)
    }
}

impl Render for NavEntrySimple {
    fn render(&self) -> Markup {
        html! {
            a
                href=(self.href)
                aria-selected=(self.active)
                hx-boost="true"
                hx-push-url="true"
                hx-target=(self.target)
            {
                @if let Some(icon) = &self.icon {
                    (crate::component::icon(icon, None))
                }

                (self.label)
            }
        }
    }
}

#[derive(Default)]
pub enum NavEntryAlignment {
    #[default]
    Left,
    Right,
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct NavEntryDropdown {
    label: String,
    entries: Vec<NavEntry>,
    #[builder(default)]
    icon: Option<Icons>,
    #[builder(default)]
    alignment: NavEntryAlignment,
    #[builder(default)]
    active: bool,
    #[builder(default = true)]
    chevron: bool,
}

impl From<NavEntryDropdown> for NavEntry {
    fn from(entry: NavEntryDropdown) -> Self {
        NavEntry::Dropdown(entry)
    }
}

impl Render for NavEntryDropdown {
    fn render(&self) -> Markup {
        html! {
            details class="dropdown" aria-selected=(self.active) {
                summary {
                    @if let Some(icon) = &self.icon {
                        (crate::component::icon(icon, None))
                    }

                    (self.label)
                    @if self.chevron {
                        (icon(&Icons::Dropdown, None))
                    }
                }
                ul {
                    @for entry in self.entries.iter() {
                        li { (entry) }
                    }
                }
            }
        }
    }
}
