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
    index::full_page(
        state,
        t!(
            "event.detail.lineup.title",
            locale = &state.locale,
            name = &event.name
        ),
        render(state, event, stage_filter),
    )
}

#[must_use]
pub fn render(state: &UiState, event: &Event, stage_filter: Option<Uuid>) -> Markup {
    let menu = super::nav_bar(state, event.id, super::View::Lineup);
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
        #event {
            (menu)
            section {
                form.flex.row {
                    input.grow type="search" name="search" placeholder=(t!("event.detail.lineup.search_acts.placeholder", locale = &state.locale))
                    select name="stage" value=[stage_filter] {
                        option value="" { (t!("event.detail.lineup.all_stages", locale = &state.locale)) }
                        @for stage in &event.stages {
                            option value=(stage.id) { (&stage.name) }
                        }
                    }
                }
                button command="show-modal" commandfor=(CREATE_ACT_DIALOG_ID) {
                    (t!("event.detail.lineup.add_act", locale = &state.locale))
                }
                (render_act_list(state, acts))
            }
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
