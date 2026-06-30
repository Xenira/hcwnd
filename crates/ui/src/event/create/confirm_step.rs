use api::{UiState, user::User};
use chrono::{Duration, NaiveDate};
use maud::{Markup, Render, html};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    event::{
        card::EventSuggestionCard,
        create::{
            self,
            days_step::{self, EventDay, default_days},
            details_step,
            name_step::{self, TOTAL_STEPS},
            stage_step::{self, EventStage},
        },
    },
    index, user,
};

const CURRENT_STEP: usize = TOTAL_STEPS;
pub const BASE_ROUTE: &str = "/confirm";

#[must_use]
pub fn full_page(state: &UiState, step: &EventCreateConfirmStep) -> Markup {
    index::full_page(
        state,
        t!(
            "event.create.confirm_step.title",
            locale = &state.locale,
            name = &step.name
        ),
        render(state, step),
    )
}

pub fn render(state: &UiState, step: &EventCreateConfirmStep) -> Markup {
    let back_url = format!("{}{}", create::BASE_ROUTE, stage_step::BASE_ROUTE);

    let name_step = name_step::render_hidden_inputs(step.name.as_str());
    let details_step = details_step::render_hidden_inputs(
        Some(&step.description),
        Some(&step.website),
        Some(&step.image_url),
    );
    let days_step = days_step::render_hidden_inputs(Some(step.start_date), Some(&step.days));
    let stages_step = stage_step::render_hidden_inputs(Some(&step.stages));

    let event_card: Markup = if let Some(user) = &state.user {
        let event_card: EventSuggestionCard = (step.clone(), user).into();
        event_card.render()
    } else {
        user::unauthenticated(&state.locale)
    };

    html! {
        progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
        hgroup {
            h2 {
                (t!("event.create.confirm_step.title", locale = &state.locale, name = step.name))
            }
            p {
                (t!("event.create.confirm_step.subtitle", locale = &state.locale))
            }
        }
        form
            hx-post=(create::BASE_ROUTE)
            hx-target="#main"
            hx-swap="innerHTML"
            hx-booost="true"
        {
            (event_card.render())

            label {
                (t!("event.create.confirm_step.source.label", locale = &state.locale))
                textarea
                    name="source"
                    placeholder=(t!("event.create.confirm_step.source.placeholder", locale = &state.locale))
                    rows="3"
                    value=[(&step.source)]
                    required
                    autofocus[step.source.as_ref().map_or(true, String::is_empty)]
                {
                    (step.source.as_deref().unwrap_or(""))
                }
                small {
                    (t!("event.create.confirm_step.source.hint", locale = &state.locale))
                }
            }
            label {
                (t!("event.create.confirm_step.source_url.label", locale = &state.locale))
                input
                    type="url"
                    name="source_url"
                    placeholder=(t!("event.create.confirm_step.source_url.placeholder", locale = &state.locale))
                    value=[(&step.source_url)]
                {}
                small {
                    (t!("event.create.confirm_step.source_url.hint", locale = &state.locale))
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
                    (t!("event.create.back", locale = &state.locale))
                }
                button.col-6 type="submit" {
                    (t!("event.create.submit", locale = &state.locale))
                }
            }
        }
    }
}

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
    #[serde(default, deserialize_with = "crate::util::empty_string_as_none")]
    pub source_url: Option<Url>,
}

impl From<(EventCreateConfirmStep, &User)> for EventSuggestionCard {
    fn from((confirm_step, user): (EventCreateConfirmStep, &User)) -> Self {
        Self {
            editable: false,
            suggested_by: user.clone(),
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
