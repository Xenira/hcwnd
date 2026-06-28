use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use crate::domain::artist::models::artist::ArtistId;

#[derive(Clone, Debug)]
pub struct Act {
    id: ActId,
    name: ActName,
    description: Option<ActDescription>,
    act_img: Option<ActImg>,
    artists: Vec<ArtistId>,
}

impl Act {
    pub fn new(
        id: ActId,
        name: ActName,
        description: Option<ActDescription>,
        act_img: Option<ActImg>,
        artists: Vec<ArtistId>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            act_img,
            artists,
        }
    }

    #[must_use]
    pub fn id(&self) -> &ActId {
        &self.id
    }

    #[must_use]
    pub fn name(&self) -> &ActName {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> Option<&ActDescription> {
        self.description.as_ref()
    }

    #[must_use]
    pub fn act_img(&self) -> Option<&ActImg> {
        self.act_img.as_ref()
    }

    #[must_use]
    pub fn artists(&self) -> &[ArtistId] {
        &self.artists
    }
}

#[nutype(derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef))]
pub struct ActId(Uuid);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ActName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 1000),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ActDescription(String);

#[nutype(
    validate(predicate = |v| !v.has_authority() && v.has_host() && v.scheme() == "https"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ActImg(Url);

#[derive(Clone, Debug)]
pub struct CreateActRequest {
    name: ActName,
    description: Option<ActDescription>,
    act_img: Option<ActImg>,
    artists: Vec<ArtistId>,
}

impl CreateActRequest {
    pub fn new(
        name: ActName,
        description: Option<ActDescription>,
        act_img: Option<ActImg>,
        artists: Vec<ArtistId>,
    ) -> Self {
        Self {
            name,
            description,
            act_img,
            artists,
        }
    }

    #[must_use]
    pub fn name(&self) -> &ActName {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> Option<&ActDescription> {
        self.description.as_ref()
    }

    #[must_use]
    pub fn act_img(&self) -> Option<&ActImg> {
        self.act_img.as_ref()
    }

    #[must_use]
    pub fn artists(&self) -> &[ArtistId] {
        &self.artists
    }
}

#[derive(Error, Debug)]
pub enum CreateActError {
    #[error("An act with the same name already exists")]
    DuplicateName,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ListActsError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetActError {
    #[error("Act not found")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
