use api::{act::Act, event::Event, UiState};
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    act::create::{ActCreate, CREATE_ACT_DIALOG_ID},
    index,
};

pub const ACTS_LIST_ID: &str = "acts_list";

#[must_use]
pub fn full_page(state: &UiState, event: &Event, stage_filter: Option<Uuid>) -> Markup {
    crate::view::event::full_page(
        state,
        &t!(
            "event.detail.lineup.title",
            locale = &state.locale,
            name = &event.name
        ),
        render(state, event, stage_filter),
    )
}

#[must_use]
pub fn render(state: &UiState, event: &Event, stage_filter: Option<Uuid>) -> Markup {
    let menu = crate::view::event::nav_bar(state, event.id, crate::view::event::View::Lineup);
    let acts = if let Some(stage_id) = stage_filter {
        &event
            .acts
            .iter()
            .filter(|act| act.id == stage_id)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        &event.acts
    };

    html! {
        (menu)
        section {
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
                        autofocus
                        required {}
                }

                label {
                    (t!("artist.create.image_url.label", locale = &state.locale))
                    input
                        type="url"
                        name="image_url"
                        placeholder=(t!("artist.create.image_url.placeholder", locale = &state.locale))
                    {}
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
                    {}
                    small {
                        (t!("artist.create.website_url.hint", locale = &state.locale))
                    }
                }

                button.btn.btn-primary type="submit" {
                    (t!("artist.create.submit", locale = &state.locale))
                }
            }
            form {
                div role="search" {
                    select name="stage" {
                        option value=""
                        {
                            (t!("event.detail.lineup.all_stages", locale = &state.locale))
                        }
                        @for stage in &event.stages {
                            option value=(stage.id) selected[Some(stage.id) == stage_filter] { (&stage.name) }
                        }
                    }
                    input
                        type="search"
                        name="search"
                        placeholder=(t!("event.detail.lineup.search_acts.placeholder", locale = &state.locale));
                }
            }
            button command="show-modal" commandfor=(CREATE_ACT_DIALOG_ID) {
                (t!("event.detail.lineup.add_act", locale = &state.locale))
            }
            (render_act_list(state, acts))
        }
    }
}

#[must_use]
pub fn render_act_list(state: &UiState, acts: &[Act]) -> Markup {
    html! {
        div id=(ACTS_LIST_ID) {
            @for act in acts {
                (render_act(state, act))
            }
        }
    }
}

#[must_use]
pub fn render_act(state: &UiState, act: &Act) -> Markup {
    html! {
        (&act.name)
    }
}
