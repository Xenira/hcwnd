use std::collections::HashSet;

use derive_builder::Builder;
use es_entity::{
    es_query, EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo, IntoEvents,
    TryFromEvents,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    domain::{
        artist::models::artist::{
            Artist as DomainArtist, ArtistGenre, ArtistId as DomainArtistId, ArtistName,
            CreateArtistRequest, SearchArtistsQuery,
        },
        event::models::event::ImageUrl,
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
        description: String,
        image_url: Option<Url>,
        genres: Vec<String>,
    },
    Approved {
        reason: Option<String>,
        user_id: UserId,
    },
    AutoApproved,
    Rejected {
        reason: Option<String>,
        user_id: UserId,
    },
    Deleted {
        reason: Option<String>,
        user_id: UserId,
    },
    Voted {
        power: i32,
        comment: Option<String>,
        user_id: UserId,
    },
    EditProposal {
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        image_url: Option<Option<Url>>,
        added_genres: Option<Vec<String>>,
        removed_genres: Option<Vec<String>>,
        source: String,
        source_url: Option<Url>,
        user_id: UserId,
    },
    EditApproved {
        id: Uuid,
        user_id: UserId,
    },
    EditAutoApproved {
        id: Uuid,
    },
    EditRejected {
        id: Uuid,
        reason: Option<String>,
        user_id: UserId,
    },
    EditVoted {
        id: Uuid,
        power: i32,
        comment: Option<String>,
        user_id: UserId,
    },
}

#[derive(Debug, Serialize, Deserialize, Default, sqlx::Type)]
#[sqlx(type_name = "event_state", rename_all = "snake_case")]
enum ArtistState {
    #[default]
    Unapproved,
    Live,
    Rejected,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize)]
struct EditProposal {
    upvotes: u32,
    downvotes: u32,
    score: i32,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<Option<Url>>,
    added_genres: Option<Vec<String>>,
    removed_genres: Option<Vec<String>>,
    source: String,
    source_url: Option<Url>,
    user_id: UserId,
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    #[builder(default)]
    pub description: String,
    #[builder(default)]
    pub image_url: Option<Url>,
    #[builder(default)]
    pub genres: Vec<String>,

    // Approval
    #[builder(default)]
    pub state: ArtistState,
    pub upvotes: u32,
    pub downvotes: u32,
    pub score: i32,

    // Edits
    pub edit_proposals: IndexMap<Uuid, EditProposal>,

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
        let mut edit_proposals = IndexMap::new();
        let mut upvotes = 0;
        let mut downvotes = 0;
        let mut score = 0;

        for event in events.iter_all() {
            match event {
                ArtistEvent::Initialized {
                    id,
                    name,
                    description,
                    image_url,
                    genres: g,
                } => {
                    builder = builder
                        .id(*id)
                        .name(name.clone())
                        .description(description.clone())
                        .image_url(image_url.clone());
                    genres = g.iter().cloned().collect();
                }
                ArtistEvent::Voted {
                    power,
                    comment: _,
                    user_id: _,
                } => {
                    score += *power;
                    if *power > 0 {
                        upvotes += 1;
                    } else {
                        downvotes += 1;
                    }
                }
                ArtistEvent::Approved { .. } | ArtistEvent::AutoApproved => {
                    builder = builder.state(ArtistState::Live);
                }
                ArtistEvent::Rejected {
                    reason: _,
                    user_id: _,
                } => {
                    builder = builder.state(ArtistState::Rejected);
                }
                ArtistEvent::Deleted {
                    reason: _,
                    user_id: _,
                } => {
                    builder = builder.state(ArtistState::Deleted);
                }
                ArtistEvent::EditProposal {
                    id,
                    name,
                    description,
                    image_url,
                    added_genres,
                    removed_genres,
                    source,
                    source_url,
                    user_id,
                } => {
                    edit_proposals.insert(
                        *id,
                        EditProposal {
                            upvotes: 0,
                            downvotes: 0,
                            score: 0,
                            name: name.clone(),
                            description: description.clone(),
                            image_url: image_url.clone(),
                            added_genres: added_genres.clone(),
                            removed_genres: removed_genres.clone(),
                            source: source.clone(),
                            source_url: source_url.clone(),
                            user_id: *user_id,
                        },
                    );
                }
                ArtistEvent::EditVoted {
                    id,
                    power,
                    comment: _,
                    user_id: _,
                } => {
                    if let Some(proposal) = edit_proposals.get_mut(id) {
                        proposal.score += *power;
                        if *power > 0 {
                            proposal.upvotes += 1;
                        } else {
                            proposal.downvotes += 1;
                        }
                    }
                }
                ArtistEvent::EditApproved { id, .. } | ArtistEvent::EditAutoApproved { id } => {
                    if let Some(proposal) = edit_proposals.shift_remove(id) {
                        if let Some(name) = &proposal.name {
                            builder = builder.name(name.clone());
                        }
                        if let Some(description) = &proposal.description {
                            builder = builder.description(description.clone());
                        }
                        if let Some(image_url) = &proposal.image_url {
                            builder = builder.image_url(image_url.clone());
                        }
                        if let Some(removed_genres) = &proposal.removed_genres {
                            for genre in removed_genres {
                                genres.remove(genre);
                            }
                        }
                        if let Some(added_genres) = &proposal.added_genres {
                            for genre in added_genres {
                                genres.insert(genre.clone());
                            }
                        }
                    }
                }
                ArtistEvent::EditRejected {
                    id,
                    reason: _,
                    user_id: _,
                } => {
                    edit_proposals.shift_remove(id);
                }
            }
        }
        builder
            .genres(genres.into_iter().collect())
            .upvotes(upvotes)
            .downvotes(downvotes)
            .score(score)
            .events(events)
            .edit_proposals(edit_proposals)
            .build()
    }
}

#[derive(Debug)]
pub struct NewArtist {
    pub id: ArtistId,
    pub name: String,
    pub description: String,
    pub image_url: Option<Url>,
    pub genres: Vec<String>,
}

impl NewArtist {
    pub(crate) fn from_domain(req: &CreateArtistRequest, _author_id: &DomainUserId) -> NewArtist {
        NewArtist {
            id: Uuid::new_v4().into(),
            name: req.name().as_ref().to_string(),
            description: String::new(),
            image_url: None,
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
                image_url: self.image_url,
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
