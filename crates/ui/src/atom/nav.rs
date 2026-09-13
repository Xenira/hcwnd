use itertools::Itertools;
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

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
            .filter(|e| matches!(e.alignment, NavEntryAlignment::Left))
            .collect_vec();
        let right = self
            .entries
            .iter()
            .filter(|e| matches!(e.alignment, NavEntryAlignment::Right))
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

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct NavEntry {
    label: String,
    href: String,
    #[builder(default)]
    alignment: NavEntryAlignment,
    #[builder(default)]
    active: bool,
    #[builder(default="main".to_string())]
    target: String,
}

impl Render for NavEntry {
    fn render(&self) -> Markup {
        html! {
            a
                href=(self.href)
                aria-selected=(self.active)
                hx-boost="true"
                hx-push-url="true"
                hx-target=(self.target)
            {
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
