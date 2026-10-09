use crate::suggestion::{events::*, SuggestionState};
use crate::venue::events::*;
use eventwise::{event::StreamId, Aggregate};
use nutype::nutype;
use thiserror::Error;
use typed_builder::TypedBuilder;
use uuid::Uuid;

pub type ValidationToken = (Uuid, chrono::DateTime<chrono::Utc>);

#[derive(Error, Debug)]
pub enum VenueError {
    #[error("Venue not found")]
    NotFound,
    #[error("User with the same email already exists")]
    UserAlreadyExists,
    #[error("Invalid Name")]
    InvalidName,
    #[error("Invalid Address")]
    InvalidAddress,
    #[error("Invalid City")]
    InvalidCity,
    #[error("Invalid Country")]
    InvalidCountry,
    #[error("Invalid Postal Code")]
    InvalidPostalCode,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl StreamId for VenueId {
    fn key(&self) -> Uuid {
        self.clone().into_inner()
    }
}
impl From<Uuid> for VenueId {
    fn from(id: Uuid) -> Self {
        VenueId::new(id)
    }
}

#[derive(Aggregate, TypedBuilder, Clone, Debug)]
#[aggregate(error_type = VenueError, id_type = VenueId)]
#[aggregate(bootstrap(VenueSuggested))]
#[aggregate(events(
    SuggestionVoted,
    SuggestionVoteRemoved,
    SuggestionAccepted,
    SuggestionRejected
))]
pub struct Venue {
    id: VenueId,
    name: VenueName,
    address: VenueAddress,
    city: VenueCity,
    country: VenueCountry,
    postal_code: VenuePostalCode,
    #[builder(default)]
    capacity: Option<i32>,
    state: SuggestionState,
}

impl Venue {
    pub fn id(&self) -> &VenueId {
        &self.id
    }

    pub fn name(&self) -> &VenueName {
        &self.name
    }

    pub fn address(&self) -> &VenueAddress {
        &self.address
    }

    pub fn city(&self) -> &VenueCity {
        &self.city
    }

    pub fn country(&self) -> &VenueCountry {
        &self.country
    }

    pub fn postal_code(&self) -> &VenuePostalCode {
        &self.postal_code
    }

    pub fn capacity(&self) -> Option<i32> {
        self.capacity
    }

    pub fn state_mut(&mut self) -> &mut SuggestionState {
        &mut self.state
    }
}

pub struct VenueProjection {
    pub id: VenueId,
    pub name: VenueName,
    pub address: VenueAddress,
    pub city: VenueCity,
    pub country: VenueCountry,
    pub postal_code: VenuePostalCode,
    pub capacity: Option<i32>,
}

#[nutype(derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref))]
pub struct VenueId(Uuid);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 3, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct VenueName(String);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 3, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct VenueAddress(String);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 1, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct VenueCity(String);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 3, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct VenueCountry(String);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 3, len_char_max = 32),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct VenuePostalCode(String);
