use chrono::{Duration, Local, NaiveDate, NaiveTime};
use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::event::create::{
    self,
    confirm_step::{self, EventCreateConfirmStep},
    details_step::{self, EventCreateDetailsStep},
    name_step::{self, TOTAL_STEPS},
    stage_step::{self, EventCreateStageStep, EventStage},
};

const CURRENT_STEP: usize = 3;
const MAX_DAYS: usize = 7;

pub const BASE_ROUTE: &str = "/days";
pub const ADD_DAY_ROUTE: &str = "/add-day";
pub const REMOVE_DAY_ROUTE: &str = "/remove-day";

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

impl Render for EventCreateDaysStep {
    fn render(&self) -> Markup {
        let next_url = format!("{}{}", create::BASE_ROUTE, stage_step::BASE_ROUTE);
        let back_url = format!("{}{}", create::BASE_ROUTE, details_step::BASE_ROUTE);

        let name_step = name_step::render_hidden_inputs(self.name.as_str());
        let details_step = details_step::render_hidden_inputs(
            Some(&self.description),
            Some(&self.website),
            Some(&self.image_url),
        );
        let stages_step = stage_step::render_hidden_inputs(self.stages.as_deref());
        let confirm_step =
            confirm_step::render_hidden_inputs(self.source.as_deref(), self.source_url.as_ref());

        html! {
            progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
            hgroup {
                h2 { "“" (self.name) "” - Day(s) and Time" }
                p {
                    "When does the event take place?"
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
                    "Date"
                    input type="date" name="start_date" value=[self.start_date] required {}
                }
                div #event-dates {
                    @for day in &self.days {
                        (day.render())
                    }
                }
                (day_buttons(self.days.len(), self.days.len() >= 10) )
                (name_step)
                (details_step)
                (stages_step)
                (confirm_step)
                button.secondary
                    type="submit"
                    formaction=(back_url)
                    formnovalidate
                {
                    "Back"
                }
                button type="submit" {
                    "Next"
                }
            }
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

impl Render for EventDay {
    fn render(&self) -> Markup {
        let day_id = format!("day-{}", self.day);
        let start_time_str = self
            .start_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_default();
        let end_time_str = self
            .end_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_default();
        let start_time_name = format!("days[{}][start_time]", self.day);
        let end_time_name = format!("days[{}][end_time]", self.day);
        html! {
            div id=(day_id) {
                h4 { "Day " (self.day + 1) }
                div.row {
                    input type="hidden" name=(format!("days[{}][day]", self.day)) value=(self.day) {}
                    div.col-12.col-md-6 {
                        label {
                            "Start Time"
                            input type="time" name=(start_time_name) value=(start_time_str) {}
                        }
                    }
                    div.col-12.col-md-6 {
                        label {
                            "End Time"
                            input type="time" name=(end_time_name) value=(end_time_str) {}
                        }
                    }
                }
            }
        }
    }
}

#[must_use]
pub fn default_days() -> Vec<EventDay> {
    vec![EventDay::default()]
}

#[must_use]
pub fn day_buttons(day_length: usize, oob: bool) -> Markup {
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
                    "Remove a day"
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
                    "Add another day"
                }
            }
        }
    }
}
