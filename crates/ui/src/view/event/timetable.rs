use api::{UiState, event::Event};
use maud::{Markup, html};

use crate::{index, view::event::View};

#[must_use]
pub fn full_page(state: &UiState, event: &Event) -> Markup {
    let title = t!(
        "event.detail.timetable.title",
        locale = &state.locale,
        name = &event.name
    );

    super::full_page(
        state,
        &title,
        event.id,
        View::Timetable,
        timetable_view(state, event),
    )
}

#[must_use]
pub fn render(state: &UiState, event: &Event) -> Markup {
    super::render(
        state,
        event.id,
        View::Timetable,
        timetable_view(state, event),
    )
}

fn timetable_view(state: &UiState, event: &Event) -> Markup {
    html! {
        img src=(event.image_url) alt=(event.name);
        section.hero.hero-primary {
            div.container {
                h1 { (event.name) }
            }
        }
        section {
            div.container {
                header {
                    h2 { (t!("event.detail.timetable.title", locale = &state.locale)) }
                }
                p { (event.description) }
            }
        }
    }
}
