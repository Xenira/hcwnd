use std::collections::HashSet;

use derive_builder::Builder;
use es_entity::{
    es_query, EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo, IntoEvents,
    TryFromEvents,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{
        artist::models::artist::{
            Artist as DomainArtist, ArtistGenre, ArtistId as DomainArtistId, ArtistName,
            CreateArtistRequest, SearchArtistsQuery,
        },
        user::models::user::UserId as DomainUserId,
    },
    outbound::entity::user::UserId,
};

es_entity::entity_id! { ArtistId }

#[derive(EsEvent, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "ArtistId")]
pub enum ArtistEvent {
    Initialized {
        id: ArtistId,
        name: String,
        description: Option<String>,
        genres: Vec<String>,
    },
    SetName {
        name: String,
        user_id: UserId,
    },
    SetDescription {
        description: Option<String>,
        user_id: UserId,
    },
    AddGenre {
        genre: String,
        user_id: UserId,
    },
    RemoveGenre {
        genre: String,
        user_id: UserId,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub genres: Vec<String>,

    events: EntityEvents<ArtistEvent>,
}

impl TryFrom<Artist> for DomainArtist {
    type Error = anyhow::Error;

    fn try_from(value: Artist) -> Result<Self, Self::Error> {
        let id = DomainArtistId::new(value.id.into());
        let name = ArtistName::try_new(value.name.clone())?;
        let genres = value
            .genres
            .into_iter()
            .map(ArtistGenre::try_new)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DomainArtist::new(id, name, genres))
    }
}

// Any EsEntity must implement `TryFromEvents`.
// This trait is what hydrates entities after loading the events from the database
impl TryFromEvents<ArtistEvent> for Artist {
    fn try_from_events(events: EntityEvents<ArtistEvent>) -> Result<Self, EntityHydrationError> {
        let mut builder = ArtistBuilder::default();
        let mut genres = HashSet::new();

        for event in events.iter_all() {
            match event {
                ArtistEvent::Initialized {
                    id,
                    name,
                    description,
                    genres: g,
                } => {
                    builder = builder
                        .id(*id)
                        .name(name.clone())
                        .description(description.clone());
                    genres = g.iter().cloned().collect();
                }
                ArtistEvent::SetName { name, .. } => {
                    builder = builder.name(name.clone());
                }
                ArtistEvent::SetDescription { description, .. } => {
                    builder = builder.description(description.clone());
                }
                ArtistEvent::AddGenre { genre, .. } => {
                    genres.insert(genre.clone());
                }
                ArtistEvent::RemoveGenre { genre, .. } => {
                    genres.remove(genre);
                }
            }
        }
        builder = builder.genres(genres.into_iter().collect());
        builder.events(events).build()
    }
}

#[derive(Debug)]
pub struct NewArtist {
    pub id: ArtistId,
    pub name: String,
    pub description: Option<String>,
    pub genres: Vec<String>,
}

impl NewArtist {
    pub(crate) fn from_domain(req: &CreateArtistRequest, author_id: &DomainUserId) -> NewArtist {
        NewArtist {
            id: Uuid::new_v4().into(),
            name: req.name().as_ref().to_string(),
            description: None,
            genres: req
                .genres()
                .iter()
                .map(|g| g.as_ref().to_string())
                .collect(),
        }
    }
}

impl IntoEvents<ArtistEvent> for NewArtist {
    fn into_events(self) -> EntityEvents<ArtistEvent> {
        EntityEvents::init(
            self.id,
            [ArtistEvent::Initialized {
                id: self.id,
                name: self.name,
                description: self.description,
                genres: self.genres,
            }],
        )
    }
}

#[derive(EsRepo, Debug, Clone)]
#[es_repo(
    entity = "Artist",
    // Configure the columns that need populating in the index table
    columns(
        // The 'name' column
        name(ty = "String"),
    )
)]
pub struct ArtistRepo {
    // Mandatory field so that the Repository can begin transactions
    pub pool: sqlx::PgPool,
}

impl ArtistRepo {
    pub async fn search(&self, query: &SearchArtistsQuery) -> anyhow::Result<Vec<Artist>> {
        Ok(
            es_query!("SELECT * FROM artists WHERE LOWER(name) = $1", query.name())
                .fetch_n(&self.pool, usize::MAX)
                .await?
                .0,
        )
    }
}
