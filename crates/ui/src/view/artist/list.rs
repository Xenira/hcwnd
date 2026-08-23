use api::{UiState, artist::Artist};
use maud::{Markup, html};

use crate::{index, partial::artist::artist_card, view::artist::create};

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
                    a role="button"
                        href=(create::BASE_ROUTE)
                        hx-target="#main"
                        hx-push-url="true"
                        hx-boost="true"
                    {
                        (t!("artist.list.empty.create", locale = &state.locale))
                    }
                }
            }

            @for artist in artists {
                (artist_card(state, artist))
            }
        }
    }
}
