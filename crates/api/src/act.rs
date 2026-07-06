use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Act {
    pub id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum SelectedAct {
    Existing(Uuid),
    New(NewAct),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ActListEntry {
    pub id: Uuid,
    pub name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ActDetails {
    pub id: Uuid,
    pub event_id: Uuid,
    pub stage_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NewAct {
    pub stage_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetStage {
    pub stage_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetTimes {
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}
