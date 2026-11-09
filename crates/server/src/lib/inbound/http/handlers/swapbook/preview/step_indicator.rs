use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use es_entity::prelude::serde_json;
use itertools::Itertools;
use maud::{html, Markup, Render as _};
use serde::{Deserialize, Serialize};
use ui::{
    component::{Icons, StepIndicator},
    primitive::{chip::Chip, tag::Tag},
};

use crate::inbound::http::handlers::swapbook::{Control, ControlValue, Story, Variant};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(tags);
}

#[derive(Deserialize)]
struct StepIndicatorParams {
    current_step: usize,
    steps: String,
}

#[get("/step_indicator")]
async fn tags(query: Query<StepIndicatorParams>) -> Markup {
    let steps = query
        .steps
        .split(',')
        .map(|s| s.trim().to_string())
        .collect_vec();

    let indicator = StepIndicator::builder()
        .steps(steps)
        .current_step(query.current_step)
        .build();

    html! {
        form {
            header {
                (indicator)
            }
        }
    }
}

pub fn stories() -> Vec<Story> {
    vec![Story {
        id: "step_indicator",
        name: "Step Indicator",
        group: "Forms",
        variants: vec![Variant::Metadata {
            name: "step_indicator",
            controls: vec![
                Control {
                    name: "steps",
                    value: ControlValue::String {
                        default: Some("Create Event, ..., Profit"),
                    },
                },
                Control {
                    name: "current_step",
                    value: ControlValue::Number { default: Some(0.0) },
                },
            ],
            docs: None,
            play: vec![],
        }],
    }]
}
