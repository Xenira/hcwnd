use std::ops::Not as _;

use api::event::Event;
use chrono::{DateTime, Local};
use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::{
    atom::{card::Card, tag::Tag},
    component::{Icons, icon},
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct StepIndicator {
    steps: Vec<String>,
    #[builder(default)]
    current_step: usize,
}

impl Render for StepIndicator {
    fn render(&self) -> Markup {
        html! {
            ol {
                @for (i, step) in self.steps.iter().enumerate() {
                    li class=[(i == self.current_step).then_some("active").or_else(|| (i < self.current_step).then_some("completed"))] {
                        div {
                            span {
                                @if i < self.current_step {
                                    (icon(&Icons::FormStepComplete, None))
                                } @else {
                                    (i + 1)
                                }
                            }
                            (step)
                        }
                        (icon(&Icons::FormStepArrow, None))
                    }
                }
            }
        }
    }
}
