use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    act::SelectedAct,
    day::NewEventDay,
    stage::{SelectedStage, StageDetails},
    PagedResult,
};

pub type PagedEventList = PagedResult<EventListEntry, (Uuid, NaiveDate)>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct EventListEntry {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub website_url: String,
    pub image_url: String,
    pub start_date: NaiveDate,
    pub organizer_id: Option<Uuid>,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub website_url: String,
    pub image_url: String,
    pub start_date: NaiveDate,
    pub days: Vec<NewEventDay>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct EventDetails {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub website_url: String,
    pub image_url: String,
    pub start_date: NaiveDate,
    pub organizer_id: Option<Uuid>,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AddActToTimetable {
    pub act: SelectedAct,
    pub stage: SelectedStage,
    pub times: Option<ActTimes>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ActTimes {
    pub day: u16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}
