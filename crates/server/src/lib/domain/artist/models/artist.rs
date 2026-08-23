use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Artist {
    id: ArtistId,
    name: ArtistName,
    image_url: Option<Url>,
    website_url: Option<Url>,
    genres: Vec<ArtistGenre>,
}

impl Artist {
    pub fn new(
        id: ArtistId,
        name: ArtistName,
        image_url: Option<Url>,
        website_url: Option<Url>,
        genres: Vec<ArtistGenre>,
    ) -> Self {
        Self {
            id,
            name,
            image_url,
            website_url,
            genres,
        }
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

    #[must_use]
    pub fn image_url(&self) -> Option<&Url> {
        self.image_url.as_ref()
    }

    #[must_use]
    pub fn website_url(&self) -> Option<&Url> {
        self.website_url.as_ref()
    }
}

impl From<Artist> for api::artist::Artist {
    fn from(artist: Artist) -> Self {
        Self {
            id: artist.id().clone().into_inner(),
            name: artist.name().as_ref().to_string(),
            image_url: artist.image_url().map(|url| url.as_str().to_string()),
        }
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
    image_url: Option<Url>,
    website_url: Option<Url>,
    genres: Vec<ArtistGenre>,
}

impl CreateArtistRequest {
    pub fn new(
        name: ArtistName,
        image_url: Option<Url>,
        website_url: Option<Url>,
        genres: Vec<ArtistGenre>,
    ) -> Self {
        Self {
            name,
            image_url,
            website_url,
            genres,
        }
    }

    #[must_use]
    pub fn name(&self) -> &ArtistName {
        &self.name
    }

    #[must_use]
    pub fn image_url(&self) -> Option<&Url> {
        self.image_url.as_ref()
    }

    #[must_use]
    pub fn website_url(&self) -> Option<&Url> {
        self.website_url.as_ref()
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
