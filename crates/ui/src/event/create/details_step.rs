use api::UiState;
use chrono::NaiveDate;
use maud::{Markup, Render, html};
use serde::Deserialize;
use url::Url;

use crate::{
    event::create::{
        self,
        confirm_step::{self},
        days_step::{self, EventCreateDaysStep, EventDay},
        name_step::{self, TOTAL_STEPS},
        stage_step::{self, EventStage},
    },
    index,
};

const CURRENT_STEP: usize = 2;
pub const BASE_ROUTE: &str = "/details";

#[must_use]
pub fn full_page(state: &UiState, step: &EventCreateDetailsStep) -> Markup {
    index::full_page(
        state,
        t!(
            "event.create.details_step.title",
            locale = &state.locale,
            name = &step.name
        ),
        render(state, step),
    )
}

pub fn render(state: &UiState, step: &EventCreateDetailsStep) -> Markup {
    let next_url = format!("{}{}", create::BASE_ROUTE, days_step::BASE_ROUTE);
    let back_url = format!("{}{}", create::BASE_ROUTE, name_step::BASE_ROUTE);

    let name_step = name_step::render_hidden_inputs(step.name.as_str());
    let days_step = days_step::render_hidden_inputs(step.start_date, step.days.as_deref());
    let stages_step = stage_step::render_hidden_inputs(step.stages.as_deref());
    let confirm_step =
        confirm_step::render_hidden_inputs(step.source.as_deref(), step.source_url.as_ref());

    html! {
        progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
        hgroup {
            h2 {
                (t!("event.create.details_step.title", locale = &state.locale, name = step.name))
            }
            p {
                (t!("event.create.details_step.subtitle", locale = &state.locale))
            }
        }
        form #create_event_form
            action=(next_url)
            method="post"
            hx-target="#main"
            hx-boost="true"
            hx-push-url="true"
        {
            label {
                (t!("event.create.details_step.description.label", locale = &state.locale))
                textarea
                    name="description"
                    placeholder=(t!("event.create.details_step.description.placeholder", locale = &state.locale))
                    rows="10"
                    autofocus[step.description.as_ref().map_or(true, String::is_empty)]
                {
                    (step.description.as_deref().unwrap_or_default())
                }
                small {
                    (t!("event.create.details_step.description.hint", locale = &state.locale))
                }
            }
            label {
                (t!("event.create.details_step.website.label", locale = &state.locale))
                "Event Website"
                input
                    type="url"
                    name="website"
                    placeholder=(t!("event.create.details_step.website.placeholder", locale = &state.locale))
                    value=[step.website.as_ref()]
                    required {}
                small {
                    (t!("event.create.details_step.website.hint", locale = &state.locale))
                }
            }
            label {
                (t!("event.create.details_step.image.label", locale = &state.locale))
                input
                    type="url"
                    name="image_url"
                    placeholder=(t!("event.create.details_step.image.placeholder", locale = &state.locale))
                    value=[step.image_url.as_ref()]
                    required {}
                small {
                    (t!("event.create.details_step.image.hint", locale = &state.locale))
                }
            }
            (name_step)
            (days_step)
            (stages_step)
            (confirm_step)
            button type="submit" {
                (t!("event.create.next", locale = &state.locale))
            }
            button.secondary
                type="submit"
                formaction=(back_url)
                formnovalidate
            {
                (t!("event.create.back", locale = &state.locale))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EventCreateDetailsStep {
    pub name: String,
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

impl EventCreateDetailsStep {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            website: None,
            image_url: None,
            start_date: None,
            days: None,
            stages: None,
            source: None,
            source_url: None,
        }
    }
}

impl From<EventCreateDaysStep> for EventCreateDetailsStep {
    fn from(days_step: EventCreateDaysStep) -> Self {
        Self {
            name: days_step.name,
            description: Some(days_step.description),
            website: Some(days_step.website),
            image_url: Some(days_step.image_url),
            start_date: days_step.start_date,
            days: Some(days_step.days),
            stages: days_step.stages,
            source: days_step.source,
            source_url: days_step.source_url,
        }
    }
}

#[must_use]
pub fn render_hidden_inputs(
    description: Option<&str>,
    website: Option<&Url>,
    image_url: Option<&Url>,
) -> Markup {
    html! {
        @if let Some(description) = description {
            input type="hidden" name="description" value=(description) {}
        }
        @if let Some(website) = website {
            input type="hidden" name="website" value=(website) {}
        }
        @if let Some(image_url) = image_url {
            input type="hidden" name="image_url" value=(image_url) {}
        }
    }
}
