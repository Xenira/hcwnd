use api::{event::Event, UiState};
use maud::{html, Markup};

use crate::{index, view::event::View};

#[must_use]
pub fn full_page(state: &UiState, event: &Event) -> Markup {
    let title = t!(
        "event.detail.details.title",
        locale = &state.locale,
        name = &event.name
    );

    super::full_page(
        state,
        &title,
        event,
        View::Detail,
        detail_view(state, event),
    )
}

#[must_use]
pub fn render(state: &UiState, event: &Event) -> Markup {
    super::render(state, event, View::Detail, detail_view(state, event))
}

fn detail_view(state: &UiState, event: &Event) -> Markup {
    html! {
        h2 {
            (t!("event.detail.details.about", locale = &state.locale))
        }
        p {
            (event.description)
        }
        aside {
        }
    }
}
