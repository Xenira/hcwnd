use api::{UiState, artist::ArtistCreateForm};
use maud::{Markup, html};

use crate::index;

pub const BASE_ROUTE: &str = "/create-artist";

#[must_use]
pub fn full_page(state: &UiState, step: &ArtistCreateForm) -> Markup {
    index::full_page(
        state,
        t!("artist.create.title", locale = &state.locale),
        render(state, step),
    )
}

pub fn render(state: &UiState, step: &ArtistCreateForm) -> Markup {
    html! {
        h2 { (t!("artist.create.title", locale = &state.locale)) }
        form
            #artist_event_form
            method="post"
            hx-target="#main"
            hx-boost="true"
            hx-push-url="true"
        {
            label {
                (t!("artist.create.name.label", locale = &state.locale))
                input
                    type="text"
                    name="name"
                    placeholder=(t!("artist.create.name.placeholder", locale = &state.locale))
                    minlength="3"
                    maxlength="100"
                    value=[&step.name]
                    autofocus
                    required {}
            }

            label {
                (t!("artist.create.image_url.label", locale = &state.locale))
                input
                    type="url"
                    name="image_url"
                    placeholder=(t!("artist.create.image_url.placeholder", locale = &state.locale))
                    value=[&step.image_url]
                    required {}
                small {
                    (t!("artist.create.image_url.hint", locale = &state.locale))
                }
            }

            label {
                (t!("artist.create.website_url.label", locale = &state.locale))
                input
                    type="url"
                    name="website_url"
                    placeholder=(t!("artist.create.website_url.placeholder", locale = &state.locale))
                    value=[&step.website_url]
                    required {}
                small {
                    (t!("artist.create.website_url.hint", locale = &state.locale))
                }
            }

            button.btn.btn-primary type="submit" {
                (t!("artist.create.submit", locale = &state.locale))
            }
        }
    }
}

#[must_use]
pub fn render_hidden_inputs(name: &str) -> Markup {
    html! {
        input type="hidden" name="name" value=(name) {}
    }
}
