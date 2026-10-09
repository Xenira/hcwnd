use std::{collections::HashMap, ops::Deref};

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use eventwise::{event::StreamId, Aggregate};
use nutype::nutype;
use thiserror::Error;
use typed_builder::TypedBuilder;
use url::Url;
use uuid::Uuid;

use crate::{
    festival::{
        events::*,
        models::{
            act::{Act, ListActsError},
            day::*,
            stage::{ListStagesError, Stage},
        },
    },
    proposal::{ProposalSource, ProposalSourceUrl},
    suggestion::events::*,
    suggestion::SuggestionState,
    user::models::user::UserId,
};

#[derive(Debug, Error)]
pub enum EventError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, Aggregate, TypedBuilder)]
#[aggregate(id_type = FestivalId, error_type = EventError)]
#[aggregate(bootstrap(FestivalSuggested))]
#[aggregate(events(
    SuggestionVoted,
    SuggestionVoteRemoved,
    SuggestionAccepted,
    SuggestionRejected,
    ChangeSuggested,
))]
pub struct Festival {
    id: FestivalId,
    name: EventName,
    description: EventDescription,
    website_url: WebsiteUrl,
    image_url: ImageUrl,
    stages: FestivalStages,
    acts: FestivalActs,
    start_date: NaiveDate,
    days: FestivalDays,
    state: SuggestionState,
}

impl Festival {
    pub fn id(&self) -> &FestivalId {
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

    pub fn stages(&self) -> &FestivalStages {
        &self.stages
    }

    pub fn acts(&self) -> &FestivalActs {
        &self.acts
    }

    pub fn days(&self) -> &FestivalDays {
        &self.days
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDate {
        self.start_date + Duration::days(self.days.as_ref().len().try_into().unwrap_or(1))
    }

    pub(crate) fn set_name(&mut self, name: EventName) {
        self.name = name;
    }

    pub(crate) fn set_description(&mut self, description: EventDescription) {
        self.description = description;
    }

    pub(crate) fn set_website_url(&mut self, website_url: WebsiteUrl) {
        self.website_url = website_url;
    }

    pub(crate) fn set_image_url(&mut self, image_url: ImageUrl) {
        self.image_url = image_url;
    }

    pub(crate) fn set_start_date(&mut self, start_date: NaiveDate) {
        self.start_date = start_date;
    }

    pub(crate) fn set_days(&mut self, vec: FestivalDays) {
        self.days = vec;
    }

    pub(crate) fn state_mut(&mut self) -> &mut SuggestionState {
        &mut self.state
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FestivalState {
    Suggested {
        votes: HashMap<UserId, i32>,
        user_id: UserId,
    },
    Active,
    Rejected,
}

#[derive(Clone, Debug)]
pub struct EventListItem {
    id: FestivalId,
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
        id: FestivalId,
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
    pub fn id(&self) -> &FestivalId {
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
pub struct FestivalId(Uuid);

impl StreamId for FestivalId {
    fn key(&self) -> Uuid {
        self.clone().into_inner()
    }
}

impl From<Uuid> for FestivalId {
    fn from(id: Uuid) -> Self {
        FestivalId::new(id)
    }
}

#[nutype(
    sanitize(trim),
    validate(len_char_min = 5, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef),
    new_unchecked
)]
pub struct EventName(String);

#[nutype(
    sanitize(trim),
    validate(len_char_max = 1000),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef),
    new_unchecked
)]
pub struct EventDescription(String);

#[nutype(derive(Debug, Clone, AsRef))]
pub struct FestivalStages(Vec<Stage>);

#[nutype(derive(Debug, Clone, AsRef, Deref))]
pub struct FestivalActs(Vec<Act>);

#[nutype(
    validate(predicate = |v| !v.is_empty()),
    derive(Debug, Clone, AsRef, Deref),
    new_unchecked
)]
pub struct FestivalDays(Vec<(Option<NaiveTime>, Option<NaiveTime>)>);

impl FestivalDays {
    pub fn first(&self) -> &(Option<NaiveTime>, Option<NaiveTime>) {
        match self.as_ref().first() {
            Some(day) => day,
            None => unreachable!("Validation should ensure that this is never empty"),
        }
    }

    pub fn last(&self) -> &(Option<NaiveTime>, Option<NaiveTime>) {
        match self.as_ref().last() {
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
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef),
    new_unchecked
)]
pub struct WebsiteUrl(Url);

#[nutype(
    validate(predicate = |v| v.has_host() && v.scheme() == "https"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef),
    new_unchecked
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
