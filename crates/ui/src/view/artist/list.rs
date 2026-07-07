use api::{artist::Artist, UiState};
use maud::{html, Markup};

use crate::index;

pub fn full_page(state: &UiState, artists: &[Artist]) -> Markup {
    index::full_page(
        state,
        &t!("artist.list.title", locale = &state.locale),
        render(state, artists),
    )
}

pub fn render(state: &UiState, artists: &[Artist]) -> Markup {
    html! {
        form {
            fieldset role="search" {
                input type="search" name="query" placeholder=(t!("artist.list.search.placeholder", locale = &state.locale));
                input type="submit" value=(t!("artist.list.search.submit", locale = &state.locale));
            }
            @if artists.is_empty() {
                p {
                    h2 { (t!("artist.list.empty.alert", locale = &state.locale)) }
                    a role="button" href="/create-artist/name" hx-target="#main" hx-push-url="true" hx-boost="true" { (t!("artist.list.empty.create", locale = &state.locale)) }
                }
            }
        }
    }
}
