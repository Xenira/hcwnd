use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct EventDay {
    pub n: u16,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NewEventDay {
    pub n: u16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}
