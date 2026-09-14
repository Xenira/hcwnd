use api::{
    event::new::{EventCreateDetailsStep, EventType},
    UiState,
};
use chrono::{Local, NaiveDate};
use maud::{html, Markup, Render};
use serde::Deserialize;
use typed_builder::TypedBuilder;
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
    view::{event::new::step_indicator, View},
};

const CURRENT_STEP: usize = 0;
pub const BASE_ROUTE: &str = "/details";

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct DetailsStep<'a> {
    details_step: &'a EventCreateDetailsStep,
    // #[builder(default)]
    // schedule_step: Option<&'a EventCreateScheduleStep>,
}

impl<'a> View for DetailsStep<'a> {
    fn render(&self, state: &UiState) -> Markup {
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
            .value_opt(self.details_step.name.clone())
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
            .value(self.details_step.event_type.to_string())
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
            .value_opt(self.details_step.description.clone())
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
            aside {
                h2 {
                    (t!("event.create.details_step.preview.title", locale = &state.locale))
                }
                (EventCard::builder()
                    .id(Uuid::nil())
                    .name(self.details_step.name.as_deref().unwrap_or_default())
                    .location(t!("event.create.details_step.preview.location", locale = &state.locale).to_string())
                    .start_time(Local::now())
                    .end_time(Local::now())
                    .image(self.details_step.image_url.as_ref().map(Url::to_string).unwrap_or_default())
                    .flair(t!("event.create.details_step.preview.flair", locale = &state.locale).to_string())
                    .non_interactive()
                    .build())
            }
        }
    }

    fn title(&self, state: &UiState) -> std::borrow::Cow<'_, str> {
        t!("event.create.details_step.title", locale = &state.locale)
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
