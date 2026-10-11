use api::{festival::Festival, UiState};
use maud::{html, Markup};

use crate::{index, view::festival::View};

#[must_use]
pub fn full_page(state: &UiState, event: &Festival) -> Markup {
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
pub fn render(state: &UiState, event: &Festival) -> Markup {
    super::render(state, event, View::Detail, detail_view(state, event))
}

fn detail_view(state: &UiState, event: &Festival) -> Markup {
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
