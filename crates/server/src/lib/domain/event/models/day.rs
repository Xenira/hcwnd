use std::hash::Hash;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use nutype::nutype;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Day {
    id: DayId,
    day_number: u8,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
}

impl Day {
    pub fn new(
        id: DayId,
        day_number: u8,
        start_time: Option<NaiveTime>,
        end_time: Option<NaiveTime>,
    ) -> Self {
        Self {
            id,
            day_number,
            start_time,
            end_time,
        }
    }

    #[must_use]
    pub fn id(&self) -> &DayId {
        &self.id
    }

    #[must_use]
    pub fn day_number(&self) -> u8 {
        self.day_number
    }

    #[must_use]
    pub fn start_time(&self) -> Option<NaiveTime> {
        self.start_time
    }

    #[must_use]
    pub fn end_time(&self) -> Option<NaiveTime> {
        self.end_time
    }

    #[must_use]
    pub fn start_datetime(&self, event_start_date: &NaiveDate) -> NaiveDateTime {
        (*event_start_date + Duration::days(self.day_number as i64))
            .and_time(self.start_time.unwrap_or_default())
    }

    #[must_use]
    pub fn end_datetime(&self, event_start_date: &NaiveDate) -> NaiveDateTime {
        let start_datetime = self.start_datetime(event_start_date);
        Self::transform_date_from(start_datetime, self.end_time.unwrap_or_default())
    }

    #[must_use]
    pub fn transform_date_from(base: NaiveDateTime, time: NaiveTime) -> NaiveDateTime {
        let naive = base.date().and_time(time);
        if naive < base {
            let technically_next_day = base.date() + Duration::days(1);

            technically_next_day.and_time(time)
        } else {
            naive
        }
    }
}

#[nutype(derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref))]
pub struct DayId(Uuid);

#[derive(Clone, Debug)]
pub struct CreateDayRequest {
    day_number: u8,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
}

impl CreateDayRequest {
    pub fn new(day_number: u8, start_time: Option<NaiveTime>, end_time: Option<NaiveTime>) -> Self {
        Self {
            day_number,
            start_time,
            end_time,
        }
    }

    #[must_use]
    pub fn day_number(&self) -> u8 {
        self.day_number
    }

    pub fn start_time(&self) -> Option<NaiveTime> {
        self.start_time
    }

    pub fn end_time(&self) -> Option<NaiveTime> {
        self.end_time
    }
}

#[derive(Error, Debug)]
pub enum CreateDayError {
    #[error("The time range overlaps with an existing day")]
    OverlappingDay,
    #[error(transparent)]
    Range(#[from] DayRangeError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ListDaysError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetDayError {
    #[error("Day not found")]
    DayNotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum DayRangeError {
    #[error("Start time must be before end time")]
    StartTimeAfterEndTime,
    #[error("Day is too long (max 24 hours)")]
    DayTooLong,
}
