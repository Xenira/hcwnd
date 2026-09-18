use std::{collections::HashMap, fmt::Display};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{day::EventDay, serde_utils::SwitchValue};

#[derive(Debug, Deserialize)]
pub struct EventCreateDetailsStep {
    // Details Step
    pub name: Option<String>,
    pub event_type: EventType,
    pub description: Option<String>,
    pub venue: Option<String>,
    pub image_url: Option<Url>,
    // Schedule Step
    pub start_date: Option<NaiveDate>,
    pub days: Option<Vec<EventDay>>,
    // Stages Step
    pub stages: Option<Vec<EventStage>>,
    pub website: Option<Url>,
    #[serde(default, deserialize_with = "crate::serde_utils::empty_string_as_none")]
    // Review Step
    pub source: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_utils::empty_string_as_none")]
    pub source_url: Option<Url>,
}

impl EventCreateDetailsStep {
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            event_type: EventType::Indoor,
            description: None,
            venue: None,
            website: None,
            image_url: None,
            start_date: None,
            days: None,
            stages: None,
            source: None,
            source_url: None,
        }
    }
}

impl Default for EventCreateDetailsStep {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
pub enum EventType {
    Indoor,
    Outdoor,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Indoor => write!(f, "Indoor"),
            EventType::Outdoor => write!(f, "Outdoor"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventStage {
    pub name: String,
    pub days: HashMap<usize, SwitchValue>,
}

// impl From<EventCreateDaysStep> for EventCreateDetailsStep {
//     fn from(days_step: EventCreateDaysStep) -> Self {
//         Self {
//             name: days_step.name,
//             description: Some(days_step.description),
//             website: Some(days_step.website),
//             image_url: Some(days_step.image_url),
//             start_date: days_step.start_date,
//             days: Some(days_step.days),
//             stages: days_step.stages,
//             source: days_step.source,
//             source_url: days_step.source_url,
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct EventCreateScheduleStep {
    // Details Step
    pub name: String,
    pub event_type: EventType,
    pub description: String,
    pub venue: Uuid,
    pub image_url: Url,
    // Schedule Step
    pub start_date: Option<NaiveDate>,
    pub days: Option<Vec<EventDay>>,
    // Stages Step
    pub stages: Option<Vec<EventStage>>,
    pub website: Option<Url>,
    #[serde(default, deserialize_with = "crate::serde_utils::empty_string_as_none")]
    // Review Step
    pub source: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_utils::empty_string_as_none")]
    pub source_url: Option<Url>,
}
