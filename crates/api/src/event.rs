use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{act::SelectedAct, day::NewEventDay, stage::SelectedStage, user::User, PagedResult};

pub type PagedEventList = PagedResult<EventListEntry, (Uuid, NaiveDate)>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub website_url: Url,
    /// String as this is just the path to the image, not a full URL
    pub image_url: String,
    pub start_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_date: NaiveDate,
    pub end_time: Option<NaiveTime>,
    pub state: EventState,
    // pub days: Vec<EventDay>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventState {
    Suggested { upvotes: u32, downvotes: u32 },
    Online,
    Verified { organizer: User },
}

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
