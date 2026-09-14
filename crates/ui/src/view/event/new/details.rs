use api::{
    UiState,
    event::new::{EventCreateDetailsStep, EventType},
};
use chrono::NaiveDate;
use maud::{Markup, Render, html};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::{
    atom::{
        form::{Form, Method},
        input::{ImageInput, TextArea, TextField},
        select::Select,
    },
    component::EventCard,
    event::create::{
        self,
        confirm_step::{self},
        days_step::{self, EventCreateDaysStep, EventDay},
        name_step::{self, TOTAL_STEPS},
        stage_step::{self, EventStage},
    },
    index,
    view::event::new::step_indicator,
};

const CURRENT_STEP: usize = 0;
pub const BASE_ROUTE: &str = "/details";

#[must_use]
pub fn full_page(state: &UiState, step: &EventCreateDetailsStep) -> Markup {
    index::full_page(
        state,
        t!("event.create.details_step.title", locale = &state.locale),
        render(state, step),
    )
}

pub fn render(state: &UiState, step: &EventCreateDetailsStep) -> Markup {
    let next_url = format!("{}{}", create::BASE_ROUTE, days_step::BASE_ROUTE);

    // let name_step = name_step::render_hidden_inputs(step.name.as_str());
    // let days_step = days_step::render_hidden_inputs(step.start_date, step.days.as_deref());
    // let stages_step = stage_step::render_hidden_inputs(step.stages.as_deref());
    // let confirm_step =
    //     confirm_step::render_hidden_inputs(step.source.as_deref(), step.source_url.as_ref());

    let form_header = html! {
        header {
            (step_indicator(state, CURRENT_STEP))
            h1 {
                (t!("event.create.details_step.title", locale = &state.locale))
            }
        }
    };

    let event_name = TextField::builder()
        .name("name")
        .label(t!(
            "event.create.details_step.name.label",
            locale = &state.locale
        ))
        .placeholder(t!(
            "event.create.details_step.name.placeholder",
            locale = &state.locale
        ))
        .value_opt(step.name.clone())
        .required()
        .build();
    let event_type = Select::builder()
        .name("type")
        .label(t!(
            "event.create.details_step.type.label",
            locale = &state.locale
        ))
        .options(vec![
            (
                EventType::Indoor.to_string(),
                t!(
                    "event.create.details_step.type.indoor",
                    locale = &state.locale
                )
                .to_string(),
            ),
            (
                EventType::Outdoor.to_string(),
                t!(
                    "event.create.details_step.type.outdoor",
                    locale = &state.locale
                )
                .to_string(),
            ),
        ])
        .value(step.event_type.to_string())
        .required()
        .build();
    let description = TextArea::builder()
        .name("description")
        .label(t!(
            "event.create.details_step.description.label",
            locale = &state.locale
        ))
        .placeholder(t!(
            "event.create.details_step.description.placeholder",
            locale = &state.locale
        ))
        .value_opt(step.description.clone())
        .required()
        .build();
    let image = ImageInput::builder()
        .name("image")
        .ui_state(state)
        .kind_label(t!(
            "event.create.details_step.image.label",
            locale = &state.locale
        ))
        .preferred_aspect_ratio_label("16:9")
        .build();

    let form_content = html! {
        (event_name)
        (event_type)
        (description)
        (image)

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
    };
    // TODO: Add hidden inputs

    let form = Form::builder()
        .id("create_event_form")
        .url(next_url)
        .content(form_content)
        .header(form_header)
        .submit_label(t!("event.create.next", locale = &state.locale))
        .build();

    // let preview = EventCard::builder()
    //     .id(Uuid::nil())
    //     .name(step.name.unwrap_or_default())
    //     .location()

    html! {
        (form)
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
