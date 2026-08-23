use api::{UiState, artist::Artist};
use maud::{Markup, html};

use crate::index;

pub fn full_page(state: &UiState, artist: &Artist) -> Markup {
    index::full_page(
        state,
        &t!(
            "artist.details.title",
            locale = &state.locale,
            name = &artist.name
        ),
        render(state, artist),
    )
}

pub fn render(state: &UiState, artist: &Artist) -> Markup {
    html! {
        (artist.name)
    }
}
