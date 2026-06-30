use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use crate::domain::{
    event::models::{
        act::{Act, ListActsError},
        day::*,
        stage::{ListStagesError, Stage},
    },
    proposal::{ProposalSource, ProposalSourceUrl},
};

#[derive(Clone, Debug)]
pub struct Event {
    id: EventId,
    name: EventName,
    description: EventDescription,
    website_url: WebsiteUrl,
    image_url: ImageUrl,
    stages: EventStages,
    acts: EventActs,
    start_date: NaiveDate,
    days: EventDays,
}

impl Event {
    pub fn new(
        id: EventId,
        name: EventName,
        description: EventDescription,
        website_url: WebsiteUrl,
        image_url: ImageUrl,
        stages: EventStages,
        acts: EventActs,
        start_date: NaiveDate,
        days: EventDays,
    ) -> Self {
        Self {
            id,
            name,
            description,
            website_url,
            image_url,
            stages,
            acts,
            start_date,
            days,
        }
    }

    pub fn id(&self) -> &EventId {
        &self.id
    }

    pub fn name(&self) -> &EventName {
        &self.name
    }

    pub fn description(&self) -> &EventDescription {
        &self.description
    }

    pub fn website_url(&self) -> &WebsiteUrl {
        &self.website_url
    }

    pub fn image_url(&self) -> &ImageUrl {
        &self.image_url
    }

    pub fn stages(&self) -> &EventStages {
        &self.stages
    }

    pub fn acts(&self) -> &EventActs {
        &self.acts
    }

    pub fn days(&self) -> &EventDays {
        &self.days
    }

    #[must_use]
    pub fn start_date(&self) -> NaiveDateTime {
        self.days().first().start_datetime(&self.start_date)
    }

    #[must_use]
    pub fn end_date(&self) -> NaiveDateTime {
        self.days().first().end_datetime(&self.start_date)
    }
}

#[derive(Clone, Debug)]
pub struct EventListItem {
    id: EventId,
    name: EventName,
    description: EventDescription,
    image_url: ImageUrl,
    start_date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_date: NaiveDate,
    end_time: Option<NaiveTime>,
}

impl EventListItem {
    pub fn new(
        id: EventId,
        name: EventName,
        description: EventDescription,
        image_url: ImageUrl,
        start_date: NaiveDate,
        start_time: Option<NaiveTime>,
        end_date: NaiveDate,
        end_time: Option<NaiveTime>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            image_url,
            start_date,
            start_time,
            end_date,
            end_time,
        }
    }

    #[must_use]
    pub fn id(&self) -> &EventId {
        &self.id
    }

    #[must_use]
    pub fn name(&self) -> &EventName {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> &EventDescription {
        &self.description
    }

    #[must_use]
    pub fn image_url(&self) -> &ImageUrl {
        &self.image_url
    }

    #[must_use]
    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    #[must_use]
    pub fn start_time(&self) -> Option<NaiveTime> {
        self.start_time
    }

    #[must_use]
    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    #[must_use]
    pub fn end_time(&self) -> Option<NaiveTime> {
        self.end_time
    }
}

#[nutype(derive(Display, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef))]
pub struct EventId(Uuid);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 5, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct EventName(String);

#[nutype(
    sanitize(trim),
    validate(len_char_max = 1000),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct EventDescription(String);

#[nutype(
    validate(predicate = |v| !v.is_empty()),
    derive(Debug, Clone, AsRef)
)]
pub struct EventStages(Vec<Stage>);

#[nutype(derive(Debug, Clone, AsRef))]
pub struct EventActs(Vec<Act>);

#[nutype(
    validate(predicate = |v| !v.is_empty()),
    derive(Debug, Clone, AsRef)
)]
pub struct EventDays(Vec<Day>);

impl EventDays {
    pub fn first(&self) -> &Day {
        match self.as_ref().first() {
            Some(day) => day,
            None => unreachable!("Validation should ensure that this is never empty"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateEventRequest {
    name: EventName,
    description: EventDescription,
    start_date: NaiveDate,
    days: EventDaysCreateRequests,
    website_url: WebsiteUrl,
    image_url: ImageUrl,
    source: ProposalSource,
    source_url: Option<ProposalSourceUrl>,
}

impl CreateEventRequest {
    pub fn new(
        name: EventName,
        description: EventDescription,
        start_date: NaiveDate,
        days: EventDaysCreateRequests,
        website_url: WebsiteUrl,
        image_url: ImageUrl,
        source: ProposalSource,
        source_url: Option<ProposalSourceUrl>,
    ) -> Self {
        Self {
            name,
            description,
            start_date,
            days,
            website_url,
            image_url,
            source,
            source_url,
        }
    }

    pub fn name(&self) -> &EventName {
        &self.name
    }

    pub fn description(&self) -> &EventDescription {
        &self.description
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn days(&self) -> &EventDaysCreateRequests {
        &self.days
    }

    pub fn website_url(&self) -> &WebsiteUrl {
        &self.website_url
    }

    pub fn image_url(&self) -> &ImageUrl {
        &self.image_url
    }

    pub fn source(&self) -> &ProposalSource {
        &self.source
    }

    pub fn source_url(&self) -> Option<&ProposalSourceUrl> {
        self.source_url.as_ref()
    }
}

#[nutype(
    validate(predicate = |v| v.has_host() && v.scheme() == "https"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct WebsiteUrl(Url);

#[nutype(
    validate(predicate = |v| v.has_host() && v.scheme() == "https"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ImageUrl(Url);

#[nutype(
    validate(predicate = |v| !v.is_empty() && v.len() <= 10),
    derive(Debug, Clone, AsRef)
)]
pub struct EventDaysCreateRequests(Vec<CreateDayRequest>);

impl EventDaysCreateRequests {
    pub fn first(&self) -> &CreateDayRequest {
        match self.as_ref().first() {
            Some(day) => day,
            None => unreachable!("Validation should ensure that this is never empty"),
        }
    }
}

#[derive(Error, Debug)]
pub enum CreateEventError {
    #[error("Event with the same name already exists")]
    EventAlreadyExists,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetEventError {
    #[error("Event not found")]
    EventNotFound,
    #[error(transparent)]
    ListDays(#[from] ListDaysError),
    #[error(transparent)]
    ListStages(#[from] ListStagesError),
    #[error(transparent)]
    ListActs(#[from] ListActsError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ListEventsError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
