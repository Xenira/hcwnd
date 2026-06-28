use chrono::{Duration, Local, NaiveDate, NaiveTime};
use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::event::create::{
    self,
    confirm_step::{self, EventCreateConfirmStep},
    days_step::{self, EventCreateDaysStep, EventDay},
    name_step::{self, TOTAL_STEPS},
    stage_step::{self, EventStage},
};

const CURRENT_STEP: usize = 2;
pub const BASE_ROUTE: &str = "/details";

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

impl Render for EventCreateDetailsStep {
    fn render(&self) -> Markup {
        let next_url = format!("{}{}", create::BASE_ROUTE, days_step::BASE_ROUTE);
        let back_url = format!("{}{}", create::BASE_ROUTE, name_step::BASE_ROUTE);

        let name_step = name_step::render_hidden_inputs(self.name.as_str());
        let days_step = days_step::render_hidden_inputs(self.start_date, self.days.as_deref());
        let stages_step = stage_step::render_hidden_inputs(self.stages.as_deref());
        let confirm_step =
            confirm_step::render_hidden_inputs(self.source.as_deref(), self.source_url.as_ref());

        html! {
            progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
            hgroup {
                h2 { "“" (self.name) "” - Details" }
                p {
                    "Great! Now let's add some details about the event."
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
                    "Description"
                    textarea
                        name="description"
                        placeholder="Enter Event Description"
                        rows="10"
                        autofocus[self.description.as_ref().map_or(true, String::is_empty)]
                    {
                        (self.description.as_deref().unwrap_or_default())
                    }
                    small {
                        "If available, this should be the official description of the event, as provided on the event's website or social media pages. If not available, you can write a brief description of the event yourself."
                    }
                }
                label {
                    "Event Website"
                    input
                        type="url"
                        name="website"
                        placeholder="https://example.com"
                        value=[self.website.as_ref()]
                        required {}
                    small {
                        "This should be a link to the event's official website. If possible, provide a link to the english version of the website."
                    }
                }
                label {
                    "Event Image"
                    input
                        type="url"
                        name="image_url"
                        placeholder="https://example.com/image.jpg"
                        value=[self.image_url.as_ref()]
                        required {}
                    small {
                        "This should be a link to an image representing the event (e.g. poster, logo, etc.)"
                    }
                }
                (name_step)
                (days_step)
                (stages_step)
                (confirm_step)
                button type="submit" {
                    "Next"
                }
                button.secondary
                    type="submit"
                    formaction=(back_url)
                    formnovalidate
                {
                    "Back"
                }
            }
        }
    }
}

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
