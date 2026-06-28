use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum SelectedStage {
    Existing(Uuid),
    New(NewStage),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct StageDetails {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NewStage {
    pub name: String,
}
