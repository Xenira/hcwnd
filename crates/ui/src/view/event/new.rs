use maud::Markup;

use crate::component::StepIndicator;

pub mod details;

pub(crate) fn step_indicator(state: &api::UiState, current_step: usize) -> StepIndicator {
    StepIndicator::builder()
        .steps(vec![
            t!(
                "event.create.details_step.step_name",
                locale = &state.locale
            )
            .to_string(),
            t!(
                "event.create.schedule_step.step_name",
                locale = &state.locale
            )
            .to_string(),
            t!("event.create.stage_step.step_name", locale = &state.locale).to_string(),
            t!(
                "event.create.confirm_step.step_name",
                locale = &state.locale
            )
            .to_string(),
        ])
        .current_step(current_step)
        .build()
}
