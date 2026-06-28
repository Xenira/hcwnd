use chrono::{Duration, Local, NaiveTime};
use maud::{html, Markup, Render};

use crate::{
    component::dialog,
    event::create::{
        confirm_step::EventCreateConfirmStep, days_step::EventCreateDaysStep,
        details_step::EventCreateDetailsStep, name_step::EventCreateNameStep,
        stage_step::EventCreateStageStep,
    },
    user::User,
};

pub mod confirm_step;
pub mod days_step;
pub mod details_step;
pub mod name_step;
pub mod stage_step;

pub const CREATE_EVENT_DIALOG_ID: &str = "create_event_form";
pub const BASE_ROUTE: &str = "/create-event";

#[derive(Debug)]
pub enum EventCreate {
    NameStep(EventCreateNameStep),
    DetailsStep(EventCreateDetailsStep),
    DaysStep(EventCreateDaysStep),
    StagesStep(EventCreateStageStep),
    ConfirmStep(EventCreateConfirmStep, User),
}

impl Render for EventCreate {
    fn render(&self) -> Markup {
        match self {
            EventCreate::NameStep(step) => step.render(),
            EventCreate::DetailsStep(step) => step.render(),
            EventCreate::DaysStep(step) => step.render(),
            EventCreate::StagesStep(step) => step.render(),
            EventCreate::ConfirmStep(step, user) => step.render(user.clone()),
        }
    }
}
