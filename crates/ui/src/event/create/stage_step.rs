// use std::collections::HashMap;
//
// use api::{
//     day::EventDay,
//     event::new::{EventCreateConfirmStep, EventCreateStageStep, EventStage},
//     UiState,
// };
// use chrono::NaiveDate;
// use itertools::Itertools;
// use maud::{html, Markup, Render};
// use serde::{Deserialize, Serialize};
// use url::Url;
//
// use crate::{
//     event::create::{
//         self,
//         confirm_step::{self},
//         days_step::{self, default_days},
//         details_step::{self},
//         name_step::{self, TOTAL_STEPS},
//     },
//     index,
//     util::SwitchValue,
// };
//
// const CURRENT_STEP: usize = 3;
// pub const BASE_ROUTE: &str = "/stages";
// pub const ADD_STAGE_ROUTE: &str = "/add-stage";
// pub const REMOVE_STAGE_ROUTE: &str = "/remove-stage";
//
// #[must_use]
// pub fn full_page(state: &UiState, step: &EventCreateStageStep) -> Markup {
//     index::full_page(
//         state,
//         t!(
//             "event.create.stage_step.title",
//             locale = &state.locale,
//             name = &step.name
//         ),
//         render(state, step),
//     )
// }
//
// #[must_use]
// pub fn render(state: &UiState, step: &EventCreateStageStep) -> Markup {
//     let next_url = format!("{}{}", create::BASE_ROUTE, confirm_step::BASE_ROUTE);
//     let back_url = format!("{}{}", create::BASE_ROUTE, days_step::BASE_ROUTE);
//
//     let base_url = format!("{}{BASE_ROUTE}", create::BASE_ROUTE);
//     let new_url = format!("{base_url}{ADD_STAGE_ROUTE}");
//
//     let name_step = name_step::render_hidden_inputs(step.name.as_str());
//     let details_step = details_step::render_hidden_inputs(
//         Some(&step.description),
//         Some(&step.website),
//         Some(&step.image_url),
//     );
//     let days_step = days_step::render_hidden_inputs(Some(step.start_date), Some(&step.days));
//     let confirm_step =
//         confirm_step::render_hidden_inputs(step.source.as_deref(), step.source_url.as_ref());
//
//     html! {
//         progress.progress-success value=(CURRENT_STEP) max=(TOTAL_STEPS) {}
//         hgroup {
//             h2 {
//                 (t!("event.create.stage_step.title", locale = &state.locale, name = step.name))
//             }
//             p {
//                 (t!("event.create.stage_step.subtitle", locale = &state.locale))
//             }
//         }
//         form #create_event_form
//             action=(next_url)
//             method="post"
//             hx-target="#main"
//             hx-boost="true"
//             hx-push-url="true"
//         {
//             input type="hidden" name="name" value=(step.name) {}
//             div #event-stages {
//                 @for (i, stage) in step.stages.iter().enumerate() {
//                     div id=(format!("stage-{i}")) {
//                         (render_stage(&state.locale, stage, i))
//                     }
//                 }
//             }
//             button.btn-accent
//                 type="submit"
//                 formaction=(new_url)
//                 formnovalidate
//                 hx-boost="true"
//                 hx-post=(new_url)
//                 hx-target="#event-stages"
//                 hx-swap="beforeend"
//             {
//                 (t!("event.create.stage_step.add_stage", locale = &state.locale))
//             }
//
//             (name_step)
//             (details_step)
//             (days_step)
//             (confirm_step)
//
//             button.secondary
//                 type="submit"
//                 formaction=(back_url)
//                 formnovalidate
//             {
//                 (t!("event.create.back", locale = &state.locale))
//             }
//             button type="submit" {
//                 (t!("event.create.next", locale = &state.locale))
//             }
//         }
//     }
// }
//
// #[must_use]
// pub fn render_stage(locale: &str, stage: &EventStage, i: usize) -> Markup {
//     let days = stage
//         .days
//         .iter()
//         .sorted_by_cached_key(|(day, _)| *day)
//         .map(|(day, active)| {
//             html! {
//                 div.col-12.col-md-4 {
//                     label {
//                         input
//                             type="checkbox"
//                             role="switch"
//                             name=(format!("stages[{i}][days][{day}]"))
//                             checked=(*active)
//                         {}
//                         (t!("event.create.stage_step.day_switch", locale = locale, n = day))
//                     }
//                 }
//             }
//         })
//         .collect_vec();
//
//     html! {
//         article {
//             input
//                 type="text"
//                 name=(format!("stages[{i}][name]"))
//                 value=(stage.name)
//                 autofocus[stage.name.is_empty()]
//                 required
//             {}
//             button.btn-danger
//                 type="submit"
//                 formaction=(format!("{}{BASE_ROUTE}{REMOVE_STAGE_ROUTE}/{i}", create::BASE_ROUTE))
//                 formnovalidate
//                 hx-boost="true"
//                 hx-post=(format!("{}{BASE_ROUTE}{REMOVE_STAGE_ROUTE}/{i}", create::BASE_ROUTE))
//                 hx-target="closest article"
//                 hx-swap="outerHTML"
//             {
//                 (t!("event.create.stage_step.remove_stage", locale = locale))
//             }
//
//             @for day in days {
//                 (day)
//             }
//         }
//     }
// }
//
// #[must_use]
// pub fn render_hidden_inputs(stages: Option<&[EventStage]>) -> Markup {
//     let Some(stages) = stages else {
//         return html! {};
//     };
//     html! {
//         @for (i, stage) in stages.iter().enumerate() {
//             input type="hidden" name=(format!("stages[{i}][name]")) value=(stage.name) {}
//             @for (day, active) in &stage.days {
//                 input type="hidden" name=(format!("stages[{i}][days][{day}]")) value=(active) {}
//             }
//         }
//     }
// }
