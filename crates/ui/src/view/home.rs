use std::{borrow::Cow, fmt::Display};

use api::{UiState, event::Event};
use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;

use crate::view::View;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Index {
    #[builder(default)]
    events: Vec<Event>,
}

impl View for Index {
    fn render(&self, state: &UiState) -> maud::Markup {
        html! {
            div class="home" {
                h1 { (t!("app.title.home", locale = &state.locale)) }
                p { (t!("app.description.home", locale = &state.locale)) }
                form method="get" action="/search" {
                    input type="text" name="q" placeholder=(t!("app.search.placeholder", locale = &state.locale)) {}
                    button type="submit" { (t!("app.search.button", locale = &state.locale)) }
                }
            }
        }
    }

    fn title(&self, state: &UiState) -> Cow<'_, str> {
        t!("app.title.home", locale = &state.locale)
    }
}
