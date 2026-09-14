use maud::Markup;

use crate::component::StepIndicator;

pub mod details;

pub(crate) fn step_indicator(state: &api::UiState, current_step: usize) -> StepIndicator {
    StepIndicator::builder()
        .steps(vec![
            t!("event.create.details_step.name", locale = &state.locale),
            t!("event.create.schedule_step.name", locale = &state.locale),
            t!("event.create.stages_step.name", locale = &state.locale),
            t!("event.create.confirm_step.name", locale = &state.locale),
        ])
        .current_step(current_step)
        .build()
}
