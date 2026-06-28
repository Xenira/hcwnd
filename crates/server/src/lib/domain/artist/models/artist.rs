use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Artist {
    id: ArtistId,
    name: ArtistName,
    genres: Vec<ArtistGenre>,
}

impl Artist {
    pub fn new(id: ArtistId, name: ArtistName, genres: Vec<ArtistGenre>) -> Self {
        Self { id, name, genres }
    }

    #[must_use]
    pub fn id(&self) -> &ArtistId {
        &self.id
    }

    #[must_use]
    pub fn name(&self) -> &ArtistName {
        &self.name
    }

    #[must_use]
    pub fn genres(&self) -> &[ArtistGenre] {
        &self.genres
    }
}

#[nutype(derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef))]
pub struct ArtistId(Uuid);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ArtistName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 50),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct ArtistGenre(String);

#[derive(Clone, Debug)]
pub struct CreateArtistRequest {
    name: ArtistName,
    genres: Vec<ArtistGenre>,
}

impl CreateArtistRequest {
    pub fn new(name: ArtistName, genres: Vec<ArtistGenre>) -> Self {
        Self { name, genres }
    }

    #[must_use]
    pub fn name(&self) -> &ArtistName {
        &self.name
    }

    #[must_use]
    pub fn genres(&self) -> &[ArtistGenre] {
        &self.genres
    }
}

#[derive(Error, Debug)]
pub enum CreateArtistError {
    #[error("An act with the same name already exists")]
    DuplicateName,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
pub struct SearchArtistsQuery {
    name: Option<String>,
}

impl SearchArtistsQuery {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
    }

    #[must_use]
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

#[derive(Error, Debug)]
pub enum SearchArtistsError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetArtistError {
    #[error("Act not found")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
