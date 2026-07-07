use api::{event::Event, UiState};
use maud::{html, Markup};

use crate::{index, view::event::View};

#[must_use]
pub fn full_page(state: &UiState, event: &Event) -> Markup {
    let title = t!(
        "event.detail.timetable.title",
        locale = &state.locale,
        name = &event.name
    );

    super::full_page(state, &title, render(state, event))
}

#[must_use]
pub fn render(state: &UiState, event: &Event) -> Markup {
    html! {
        (super::nav_bar(state, event.id, View::Timetable))
        img src=(event.image_url) alt=(event.name);
        section.hero.hero-primary {
            div.container {
                h1 { (event.name) }
            }
        }
        section {
            div.container {
                header {
                    h2 { "Overview" }
                }
                p { (event.description) }
            }
        }
    }
}
