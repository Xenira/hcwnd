use std::collections::HashMap;

use chrono::{Duration, Local, NaiveDate, NaiveTime};
use itertools::Itertools;
use maud::{Markup, Render, html};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    event::create::{
        self,
        confirm_step::{self, EventCreateConfirmStep},
        days_step::{self, EventCreateDaysStep, EventDay, default_days},
        details_step::{self, EventCreateDetailsStep},
        name_step::{self, TOTAL_STEPS},
    },
    util::SwitchValue,
};

const CURRENT_STEP: usize = 3;
pub const BASE_ROUTE: &str = "/stages";
pub const ADD_STAGE_ROUTE: &str = "/add-stage";
pub const REMOVE_STAGE_ROUTE: &str = "/remove-stage";

#[derive(Debug, Deserialize)]
pub struct EventCreateStageStep {
    pub name: String,
    pub description: String,
    pub website: Url,
    pub image_url: Url,
    pub start_date: NaiveDate,
    #[serde(default = "default_days")]
    pub days: Vec<EventDay>,
    #[serde(default)]
    pub stages: Vec<EventStage>,
    pub source: Option<String>,
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub source_url: Option<Url>,
}

impl EventCreateStageStep {
    pub fn populate_stages(&mut self) {
        if self.stages.is_empty() {
            self.stages.push(EventStage {
                name: "Main Stage".to_string(),
                days: HashMap::with_capacity(self.days.len()),
            });
        }

        for stage in &mut self.stages {
            stage
                .days
                .retain(|i, _| self.days.iter().any(|d| d.day == *i));
        }

        for stage in &mut self.stages {
            for day in &self.days {
                stage.days.entry(day.day).or_insert(SwitchValue(true));
            }
        }
    }
}

impl From<EventCreateConfirmStep> for EventCreateStageStep {
    fn from(confirm_step: EventCreateConfirmStep) -> Self {
        Self {
            name: confirm_step.name,
            description: confirm_step.description,
            website: confirm_step.website,
            image_url: confirm_step.image_url,
            start_date: confirm_step.start_date,
            days: confirm_step.days,
            stages: confirm_step.stages,
            source: confirm_step.source,
            source_url: confirm_step.source_url,
        }
    }
}

impl Render for EventCreateStageStep {
    fn render(&self) -> Markup {
        let next_url = format!("{}{}", create::BASE_ROUTE, confirm_step::BASE_ROUTE);
        let back_url = format!("{}{}", create::BASE_ROUTE, days_step::BASE_ROUTE);

        let base_url = format!("{}{BASE_ROUTE}", create::BASE_ROUTE);
        let new_url = format!("{base_url}{ADD_STAGE_ROUTE}");

        let name_step = name_step::render_hidden_inputs(self.name.as_str());
        let details_step = details_step::render_hidden_inputs(
            Some(&self.description),
            Some(&self.website),
            Some(&self.image_url),
        );
        let days_step = days_step::render_hidden_inputs(Some(self.start_date), Some(&self.days));
        let confirm_step =
            confirm_step::render_hidden_inputs(self.source.as_deref(), self.source_url.as_ref());

        html! {
            progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
            hgroup {
                h2 {
                    (t!("event.create.stage_step.title", name = self.name))
                }
                p {
                    (t!("event.create.stage_step.subtitle"))
                }
            }
            form #create_event_form
                action=(next_url)
                method="post"
                hx-target="#main"
                hx-boost="true"
                hx-push-url="true"
            {
                input type="hidden" name="name" value=(self.name) {}
                div #event-stages {
                    @for (i, stage) in self.stages.iter().enumerate() {
                        div id=(format!("stage-{i}")) {
                            (stage.render(i))
                        }
                    }
                }
                button.btn-accent
                    type="submit"
                    formaction=(new_url)
                    formnovalidate
                    hx-boost="true"
                    hx-post=(new_url)
                    hx-target="#event-stages"
                    hx-swap="beforeend"
                {
                    (t!("event.create.stage_step.add_stage"))
                }

                (name_step)
                (details_step)
                (days_step)
                (confirm_step)

                button.secondary
                    type="submit"
                    formaction=(back_url)
                    formnovalidate
                {
                    (t!("event.create.back"))
                }
                button type="submit" {
                    (t!("event.create.next"))
                }
            }
        }
    }
}

pub fn render_hidden_inputs(stages: Option<&[EventStage]>) -> Markup {
    let Some(stages) = stages else {
        return html! {};
    };
    html! {
        @for (i, stage) in stages.iter().enumerate() {
            input type="hidden" name=(format!("stages[{i}][name]")) value=(stage.name) {}
            @for (day, active) in &stage.days {
                input type="hidden" name=(format!("stages[{i}][days][{day}]")) value=(active) {}
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventStage {
    pub name: String,
    pub days: HashMap<usize, SwitchValue>,
}

impl EventStage {
    pub fn render(&self, i: usize) -> Markup {
        let days = self
            .days
            .iter()
            .sorted_by_cached_key(|(day, _)| *day)
            .map(|(day, active)| {
                html! {
                    div.col-12.col-md-4 {
                        label {
                            input
                                type="checkbox"
                                role="switch"
                                name=(format!("stages[{i}][days][{day}]"))
                                checked=(*active)
                            {}
                            "Day " (day)
                        }
                    }
                }
            })
            .collect_vec();

        html! {
            article {
                input
                    type="text"
                    name=(format!("stages[{i}][name]"))
                    value=(self.name)
                    autofocus[self.name.is_empty()]
                    required
                {}
                button.btn-danger
                    type="submit"
                    formaction=(format!("{}{BASE_ROUTE}{REMOVE_STAGE_ROUTE}/{i}", create::BASE_ROUTE))
                    formnovalidate
                    hx-boost="true"
                    hx-post=(format!("{}{BASE_ROUTE}{REMOVE_STAGE_ROUTE}/{i}", create::BASE_ROUTE))
                    hx-target="closest article"
                    hx-swap="outerHTML"
                {
                    "Remove stage"
                }

                @for day in days {
                    (day)
                }
            }
        }
    }
}
