use std::collections::HashMap;

use chrono::NaiveDate;
use eventwise::{aggregate::Apply as _, aggregate::Create as _, event::StreamId, Aggregate};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    event::commands::*,
    event::events::*,
    event::models::festival::{
        EventDays, EventDescription, EventName, FestivalId, ImageUrl, WebsiteUrl,
    },
    user::models::user::UserId,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FestivalSuggestionId(Uuid);
impl FestivalSuggestionId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}
impl StreamId for FestivalSuggestionId {
    fn key(&self) -> Uuid {
        self.0
    }
}
impl From<Uuid> for FestivalSuggestionId {
    fn from(id: Uuid) -> Self {
        FestivalSuggestionId::new(id)
    }
}

#[derive(Debug, Error)]
pub enum FestivalSuggestionError {
    #[error("Invalid event name")]
    InvalidEventName,
}

#[derive(Clone, Debug, Aggregate)]
#[aggregate(id_type = FestivalSuggestionId, error_type = FestivalSuggestionError)]
#[aggregate(bootstrap(FestivalSuggested))]
#[aggregate(events())]
pub struct FestivalSuggestion {
    id: FestivalId,
    name: EventName,
    description: EventDescription,
    website_url: WebsiteUrl,
    image_url: ImageUrl,
    start_date: NaiveDate,
    days: u8,
    votes: HashMap<UserId, i32>,
    user_id: UserId,
    state: SuggestionState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SuggestionState {
    Pending,
    Accepted,
    Rejected,
}

impl FestivalSuggestion {
    pub fn new(
        id: FestivalId,
        name: EventName,
        description: EventDescription,
        website_url: WebsiteUrl,
        image_url: ImageUrl,
        start_date: NaiveDate,
        days: u8,
        user_id: UserId,
    ) -> Self {
        Self {
            id,
            name,
            description,
            website_url,
            image_url,
            start_date,
            days,
            votes: HashMap::new(),
            user_id,
            state: SuggestionState::Pending,
        }
    }

    pub fn add_vote(&mut self, user_id: UserId, power: i32) {
        self.votes.insert(user_id, power);
    }

    pub fn remove_vote(&mut self, user_id: UserId) {
        self.votes.remove(&user_id);
    }

    pub fn accept(&mut self) {
        self.state = SuggestionState::Accepted;
    }

    pub fn reject(&mut self) {
        self.state = SuggestionState::Rejected;
    }

    pub fn score(&self) -> i32 {
        self.votes.values().sum()
    }
}
