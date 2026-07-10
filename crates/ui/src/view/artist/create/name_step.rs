use api::UiState;
use maud::{html, Markup};

use crate::{
    index,
    view::artist::create::{self, TOTAL_STEPS},
};

const CURRENT_STEP: usize = 1;

#[must_use]
pub fn full_page(state: &UiState, step: &ArtistCreateNameStep) -> Markup {
    index::full_page(
        state,
        t!("artist.create.name_step.title", locale = &state.locale),
        render(state, step),
    )
}

pub fn render(state: &UiState, step: &ArtistCreateNameStep) -> Markup {
    let next_url = format!("{}{}", create::BASE_ROUTE, details_step::BASE_ROUTE);
    let details_step = details_step::render_hidden_inputs(
        step.description.as_deref(),
        step.website.as_ref(),
        step.image_url.as_ref(),
    );
    let confirm_step =
        confirm_step::render_hidden_inputs(step.source.as_deref(), step.source_url.as_ref());

    html! {
        progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
        hgroup {
            h2 { (t!("artist.create.name_step.title", locale = &state.locale)) }
            p {
                (t!("artist.create.name_step.subtitle", locale = &state.locale))
            }
        }
        form
            #artist_event_form
            action=(next_url)
            method="post"
            hx-target="#main"
            hx-boost="true"
            hx-push-url="true"
        {
            label {
                (t!("artist.create.name_step.name.label", locale = &state.locale))
                input
                    type="text"
                    name="name"
                    placeholder=(t!("artist.create.name_step.name.placeholder", locale = &state.locale))
                    minlength="3"
                    maxlength="100"
                    value=[&step.name]
                    autofocus[step.name.as_ref().map_or(true, String::is_empty)]
                    required {}
            }
            div #search-results {
                // This is where search results will be displayed
            }

            (details_step)
            (confirm_step)

            button type="submit" {
                (t!("artist.create.next", locale = &state.locale))
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
