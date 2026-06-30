use api::UiState;
use chrono::NaiveDate;
use maud::{Markup, Render, html};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    event::create::{
        self, confirm_step,
        days_step::{self, EventDay},
        details_step,
        stage_step::{self, EventStage},
    },
    index,
};

const CURRENT_STEP: usize = 1;
pub const TOTAL_STEPS: usize = 5;

pub const BASE_ROUTE: &str = "/name";

#[must_use]
pub fn full_page(state: &UiState, step: &EventCreateNameStep) -> Markup {
    index::full_page(
        state,
        t!("event.create.name_step.title", locale = &state.locale),
        render(state, step),
    )
}

pub fn render(state: &UiState, step: &EventCreateNameStep) -> Markup {
    let next_url = format!("{}{}", create::BASE_ROUTE, details_step::BASE_ROUTE);
    let details_step = details_step::render_hidden_inputs(
        step.description.as_deref(),
        step.website.as_ref(),
        step.image_url.as_ref(),
    );
    let days_step = days_step::render_hidden_inputs(step.start_date, step.days.as_deref());
    let stages_step = stage_step::render_hidden_inputs(step.stages.as_deref());
    let confirm_step =
        confirm_step::render_hidden_inputs(step.source.as_deref(), step.source_url.as_ref());

    html! {
        progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
        hgroup {
            h2 { (t!("event.create.name_step.title", locale = &state.locale)) }
            p {
                (t!("event.create.name_step.subtitle", locale = &state.locale))
            }
        }
        form
            #create_event_form
            action=(next_url)
            method="post"
            hx-target="#main"
            hx-boost="true"
            hx-push-url="true"
        {
            label {
                (t!("event.create.name_step.name.label", locale = &state.locale))
                input
                    type="text"
                    name="name"
                    placeholder=(t!("event.create.name_step.name.placeholder", locale = &state.locale))
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
            (days_step)
            (stages_step)
            (confirm_step)

            button type="submit" {
                (t!("event.create.next", locale = &state.locale))
            }
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct EventCreateNameStep {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<Url>,
    pub image_url: Option<Url>,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub start_date: Option<NaiveDate>,
    pub days: Option<Vec<EventDay>>,
    pub stages: Option<Vec<EventStage>>,
    pub source: Option<String>,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub source_url: Option<Url>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventCreateNameStepData {
    pub name: String,
}

#[must_use]
pub fn render_hidden_inputs(name: &str) -> Markup {
    html! {
        input type="hidden" name="name" value=(name) {}
    }
}

#[derive(Debug)]
pub enum EventSearchResult {
    Active {
        name: String,
        id: usize,
    },
    InCreation {
        id: Uuid,
        name: String,
        upvotes: usize,
        downvotes: usize,
    },
}

#[derive(Debug)]
pub struct InCreationEventSearchResult {
    pub id: Uuid,
    pub name: String,
    pub image_url: String,
    pub upvotes: usize,
    pub downvotes: usize,
    pub user_vote: Option<bool>,
}

impl Render for InCreationEventSearchResult {
    fn render(&self) -> Markup {
        html! {
            article {
                aside {
                    img src=(self.image_url) alt=(format!("Image for event {}", self.name)) {}
                }
                h3 { (self.name) }
                aside {

                }
            }
        }
    }
}
