use api::UiState;
use chrono::{NaiveDate, NaiveTime};
use maud::{Markup, Render, html};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    event::create::{
        self,
        confirm_step::{self},
        details_step::{self},
        name_step::{self, TOTAL_STEPS},
        stage_step::{self, EventCreateStageStep, EventStage},
    },
    index,
};

const CURRENT_STEP: usize = 3;
const MAX_DAYS: usize = 7;

pub const BASE_ROUTE: &str = "/days";
pub const ADD_DAY_ROUTE: &str = "/add-day";
pub const REMOVE_DAY_ROUTE: &str = "/remove-day";

#[must_use]
pub fn full_page(state: &UiState, step: &EventCreateDaysStep) -> Markup {
    index::full_page(
        state,
        t!(
            "event.create.days_step.title",
            locale = &state.locale,
            name = &step.name
        ),
        render(state, step),
    )
}

#[must_use]
pub fn render(state: &UiState, step: &EventCreateDaysStep) -> Markup {
    let next_url = format!("{}{}", create::BASE_ROUTE, stage_step::BASE_ROUTE);
    let back_url = format!("{}{}", create::BASE_ROUTE, details_step::BASE_ROUTE);

    let name_step = name_step::render_hidden_inputs(step.name.as_str());
    let details_step = details_step::render_hidden_inputs(
        Some(&step.description),
        Some(&step.website),
        Some(&step.image_url),
    );
    let stages_step = stage_step::render_hidden_inputs(step.stages.as_deref());
    let confirm_step =
        confirm_step::render_hidden_inputs(step.source.as_deref(), step.source_url.as_ref());

    html! {
        progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
        hgroup {
            h2 {
                (t!("event.create.days_step.title", locale = &state.locale, name = step.name))
            }
            p {
                (t!("event.create.days_step.subtitle", locale = &state.locale))
            }
        }
        form #create_event_form
            action=(next_url)
            method="post"
            hx-boost="true"
            hx-target="#main"
            hx-push-url="true"
        {
            label {
                (t!("event.create.days_step.date.label", locale = &state.locale))
                input
                    type="date"
                    name="start_date"
                    value=[step.start_date]
                    required
                {}
            }
            div #event-dates {
                @for day in &step.days {
                    (render_day(&state.locale, day, None))
                }
            }
            (day_buttons(&state.locale, step.days.len(), false))
            (name_step)
            (details_step)
            (stages_step)
            (confirm_step)
            button.secondary
                type="submit"
                formaction=(back_url)
                formnovalidate
            {
                (t!("event.create.back", locale = &state.locale))
            }
            button type="submit" {
                (t!("event.create.next", locale = &state.locale))
            }
        }
    }
}

#[must_use]
pub fn render_day(locale: &str, day: &EventDay, buttons: Option<usize>) -> Markup {
    let day_id = format!("day-{}", day.day);
    let start_time_str = day
        .start_time
        .map(|t| t.format("%H:%M").to_string())
        .unwrap_or_default();
    let end_time_str = day
        .end_time
        .map(|t| t.format("%H:%M").to_string())
        .unwrap_or_default();

    let start_time_name = format!("days[{}][start_time]", day.day);
    let end_time_name = format!("days[{}][end_time]", day.day);

    let buttons = buttons
        .map(|n| day_buttons(locale, n, true))
        .unwrap_or_default();

    html! {
        div id=(day_id) {
            h4 { (t!("event.create.days_step.day_n", locale = locale, n = (day.day + 1))) }
            div.row {
                input type="hidden" name=(format!("days[{}][day]", day.day)) value=(day.day) {}
                div.col-12.col-md-6 {
                    label {
                        (t!("event.create.days_step.start_time.label", locale = locale))
                        input
                            type="time"
                            name=(start_time_name)
                            value=(start_time_str)
                        {}
                    }
                }
                div.col-12.col-md-6 {
                    label {
                        (t!("event.create.days_step.end_time.label", locale = locale))
                        input
                            type="time"
                            name=(end_time_name)
                            value=(end_time_str)
                        {}
                    }
                }
            }
        }
        (buttons)
    }
}

#[derive(Debug, Deserialize)]
pub struct EventCreateDaysStep {
    pub name: String,
    pub description: String,
    pub website: Url,
    pub image_url: Url,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(default = "default_days")]
    pub days: Vec<EventDay>,
    pub stages: Option<Vec<EventStage>>,
    pub source: Option<String>,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub source_url: Option<Url>,
}

impl From<EventCreateStageStep> for EventCreateDaysStep {
    fn from(stages_step: EventCreateStageStep) -> Self {
        Self {
            name: stages_step.name,
            description: stages_step.description,
            website: stages_step.website,
            image_url: stages_step.image_url,
            start_date: Some(stages_step.start_date),
            days: stages_step.days,
            stages: Some(stages_step.stages),
            source: stages_step.source,
            source_url: stages_step.source_url,
        }
    }
}

#[must_use]
pub fn render_hidden_inputs(start_date: Option<NaiveDate>, days: Option<&[EventDay]>) -> Markup {
    html! {
        @if let Some(start_date) = start_date {
            input type="hidden" name="start_date" value=(start_date) {}
        }
        @if let Some(days) = days {
            @for day in days {
                input type="hidden" name=(format!("days[{}][day]", day.day)) value=(day.day) {}
                input type="hidden" name=(format!("days[{}][start_time]", day.day)) value=(day.start_time.map(|t| t.format("%H:%M").to_string()).unwrap_or_default()) {}
                input type="hidden" name=(format!("days[{}][end_time]", day.day)) value=(day.end_time.map(|t| t.format("%H:%M").to_string()).unwrap_or_default()) {}
            }
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDay {
    pub day: usize,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub start_time: Option<NaiveTime>,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub end_time: Option<NaiveTime>,
}

#[must_use]
pub fn default_days() -> Vec<EventDay> {
    vec![EventDay::default()]
}

#[must_use]
pub fn day_buttons(locale: &str, day_length: usize, oob: bool) -> Markup {
    let base_url = format!("{}{BASE_ROUTE}", create::BASE_ROUTE);
    let new_url = format!("{base_url}{ADD_DAY_ROUTE}");
    let remove_url = format!("{base_url}{REMOVE_DAY_ROUTE}");
    let delete_target = format!("#day-{}", day_length - 1);
    let oob = oob.then_some(true);
    html! {
        #day-buttons.flex hx-swap-oob=[oob] {
            @if day_length > 1 {
                button.error
                    type="submit"
                    formaction=(remove_url)
                    hx-post=(remove_url)
                    hx-boost="true"
                    hx-target=(delete_target)
                    hx-swap="outerHTML"
                {
                    (t!("event.create.days_step.remove_day", locale = locale))
                }
            }
            @if day_length < MAX_DAYS - 1 {
                button.accent
                    type="submit"
                    formaction=(new_url)
                    hx-post=(new_url)
                    hx-boost="true"
                    hx-target="#event-dates"
                    hx-swap="beforeend"
                {
                    (t!("event.create.days_step.add_day", locale = locale))
                }
            }
        }
    }
}
