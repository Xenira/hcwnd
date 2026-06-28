use chrono::{Duration, Local, NaiveDate, NaiveTime};
use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    event::{
        card::{EventCard, EventSuggestionCard},
        create::{
            self,
            days_step::{self, default_days, EventDay},
            details_step,
            name_step::{self, TOTAL_STEPS},
            stage_step::{self, EventCreateStageStep, EventStage},
        },
    },
    user::User,
};

const CURRENT_STEP: usize = TOTAL_STEPS;
pub const BASE_ROUTE: &str = "/confirm";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventCreateConfirmStep {
    pub name: String,
    pub description: String,
    pub website: Url,
    pub image_url: Url,
    #[serde(default)]
    pub signed_image_url: String,
    pub start_date: NaiveDate,
    #[serde(default = "default_days")]
    pub days: Vec<EventDay>,
    pub stages: Vec<EventStage>,
    pub source: Option<String>,
    pub source_url: Option<Url>,
}

impl From<(EventCreateConfirmStep, User)> for EventSuggestionCard {
    fn from((confirm_step, user): (EventCreateConfirmStep, User)) -> Self {
        Self {
            editable: false,
            suggested_by: user,
            title: confirm_step.name,
            description: confirm_step.description,
            image_url: confirm_step.signed_image_url,
            start_date: confirm_step.start_date,
            start_time: confirm_step.days.first().and_then(|d| d.start_time),
            end_date: confirm_step.start_date + Duration::days(confirm_step.days.len() as i64 - 1),
            end_time: confirm_step.days.last().and_then(|d| d.end_time),
            upvotes: 0,
            downvotes: 0,
        }
    }
}

impl EventCreateConfirmStep {
    pub fn render(&self, user: User) -> Markup {
        let back_url = format!("{}{}", create::BASE_ROUTE, stage_step::BASE_ROUTE);

        let name_step = name_step::render_hidden_inputs(self.name.as_str());
        let details_step = details_step::render_hidden_inputs(
            Some(&self.description),
            Some(&self.website),
            Some(&self.image_url),
        );
        let days_step = days_step::render_hidden_inputs(Some(self.start_date), Some(&self.days));
        let stages_step = stage_step::render_hidden_inputs(Some(&self.stages));

        let event_card: EventSuggestionCard = (self.clone(), user).into();

        html! {
            progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
            hgroup {
                h2 { (self.name) " - Confirmation" }
                p {
                    "Almost there! Check that the information you provided is correct, and let us know where you found the information about this event. This will help us ensure that the events listed on our platform are accurate and reliable."
                }
            }
            form
                hx-post="/create-event/confirm"
                hx-target="#main"
                hx-swap="innerHTML"
                hx-booost="true"
            {
                (event_card.render())

                label {
                    "Source"
                    textarea
                        name="source"
                        placeholder="Enter source of information (e.g. event website, social media, etc.)"
                        rows="3"
                        value=[(&self.source)]
                        required
                        autofocus[self.source.as_ref().map_or(true, String::is_empty)]
                    {}
                    small {
                        "Where did you find the information about this event? This is important for verification purposes."
                    }
                }
                label {
                    "Source URL"
                    input type="url" name="source_url" placeholder="https://example.com" value=[(&self.source_url)] {}
                    small {
                        "If the source of information is different from the event website, please provide a link to the source (e.g. a social media post, news article, etc.)"
                    }
                }

                (name_step)
                (details_step)
                (days_step)
                (stages_step)

                div.row {
                    button.col-6.secondary
                        type="submit"
                        formaction=(back_url)
                        formnovalidate
                    {
                        "Back"
                    }
                    button.col-6 type="submit" {
                        "Create Event"
                    }
                }
            }
        }
    }
}

pub(crate) fn render_hidden_inputs(source: Option<&str>, source_url: Option<&Url>) -> Markup {
    html! {
        @if let Some(source) = source {
            input type="hidden" name="source" value=(source) {}
        }
        @if let Some(source_url) = source_url {
            input type="hidden" name="source_url" value=(source_url) {}
        }
    }
}
