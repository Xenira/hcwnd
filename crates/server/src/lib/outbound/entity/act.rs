use api::act::ActDetails;
use chrono::NaiveDateTime;
use derive_builder::Builder;
use es_entity::{
    es_query, idempotency_guard, EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo,
    Idempotent, IntoEvents, TryFromEvents,
};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::domain::{
    artist::models::artist::ArtistId as DomainArtistId,
    event::models::{
        act::{
            Act as DomainAct, ActDescription, ActId as DomainActId, ActImg, ActName,
            CreateActRequest,
        },
        event::EventId as DomainEventId,
    },
    user::models::user::UserId as DomainUserId,
};
use crate::outbound::entity::{artist::ArtistId, event::EventId, stage::StageId, user::UserId};

es_entity::entity_id! { ActId }

#[derive(EsEvent, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "ActId")]
pub enum ActEvent {
    Initialized {
        id: ActId,
        event_id: EventId,
        stage_id: Option<StageId>,
        name: String,
        description: Option<String>,
        image_url: Option<String>,
        start_time: Option<chrono::NaiveDateTime>,
        end_time: Option<chrono::NaiveDateTime>,
        artist_ids: Vec<ArtistId>,
        user_id: UserId,
    },
    SetStage {
        stage_id: Option<StageId>,
        user_id: UserId,
    },
    SetTime {
        time: Option<(NaiveDateTime, NaiveDateTime)>,
        user_id: UserId,
    },
    SetName {
        name: String,
        user_id: UserId,
    },
    SetDescription {
        description: Option<String>,
        user_id: UserId,
    },
    AddArtist {
        artist_id: ArtistId,
        user_id: UserId,
    },
    RemoveArtist {
        artist_id: ArtistId,
        user_id: UserId,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct Act {
    pub id: ActId,
    pub event_id: EventId,
    #[builder(default)]
    pub stage_id: Option<StageId>,
    pub name: String,
    #[builder(default)]
    pub image_url: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub start_time: Option<chrono::NaiveDateTime>,
    #[builder(default)]
    pub end_time: Option<chrono::NaiveDateTime>,
    #[builder(default)]
    pub artist_ids: Vec<ArtistId>,

    events: EntityEvents<ActEvent>,
}

impl Act {
    pub fn set_time(
        &mut self,
        time: Option<(NaiveDateTime, NaiveDateTime)>,
        user_id: UserId,
    ) -> Idempotent<&mut Self> {
        idempotency_guard!(
            self.events.iter_all().rev(),
            already_applied: ActEvent::SetTime { time: existing_time, .. } if existing_time == &time,
            resets_on: ActEvent::SetTime { .. }
        );
        self.events.push(ActEvent::SetTime { time, user_id });

        Idempotent::Executed(self)
    }

    pub fn set_stage(
        &mut self,
        stage_id: Option<StageId>,
        user_id: UserId,
    ) -> Idempotent<&mut Self> {
        idempotency_guard!(
            self.events.iter_all().rev(),
            already_applied: ActEvent::SetStage { stage_id: existing_stage, .. } if existing_stage == &stage_id,
            resets_on: ActEvent::SetStage { .. }
        );
        self.events.push(ActEvent::SetStage { stage_id, user_id });

        Idempotent::Executed(self)
    }
}

impl TryFrom<Act> for DomainAct {
    type Error = anyhow::Error;

    fn try_from(value: Act) -> anyhow::Result<Self> {
        let id = DomainActId::new(value.id.into());
        let name = ActName::try_new(value.name)?;
        let description = value.description.map(ActDescription::try_new).transpose()?;
        let act_img = value
            .image_url
            .map(|url| anyhow::Ok(ActImg::try_new(Url::parse(&url)?)?))
            .transpose()?;
        let artists = value
            .artist_ids
            .into_iter()
            .map(|id| DomainArtistId::new(id.into()))
            .collect_vec();

        Ok(DomainAct::new(id, name, description, act_img, artists))
    }
}

// Any EsEntity must implement `TryFromEvents`.
// This trait is what hydrates entities after loading the events from the database
impl TryFromEvents<ActEvent> for Act {
    fn try_from_events(events: EntityEvents<ActEvent>) -> Result<Self, EntityHydrationError> {
        let mut builder = ActBuilder::default();
        let mut artist_ids = IndexSet::new();

        for event in events.iter_all() {
            match event {
                ActEvent::Initialized {
                    id,
                    event_id,
                    stage_id,
                    name,
                    description,
                    image_url,
                    start_time,
                    end_time,
                    artist_ids: artists,
                    user_id: _,
                } => {
                    builder = builder
                        .id(*id)
                        .event_id(*event_id)
                        .stage_id(*stage_id)
                        .name(name.clone())
                        .description(description.clone())
                        .image_url(image_url.clone())
                        .start_time(*start_time)
                        .end_time(*end_time);
                    artist_ids = artists.iter().cloned().collect();
                }
                ActEvent::SetStage {
                    stage_id,
                    user_id: _,
                } => {
                    builder = builder.stage_id(*stage_id);
                }
                ActEvent::SetTime { time, user_id: _ } => {
                    if let Some((start_time, end_time)) = time {
                        builder = builder
                            .start_time(Some(*start_time))
                            .end_time(Some(*end_time));
                    } else {
                        builder = builder.start_time(None).end_time(None);
                    }
                }
                ActEvent::SetName { name, user_id: _ } => {
                    builder = builder.name(name.clone());
                }
                ActEvent::SetDescription {
                    description,
                    user_id: _,
                } => {
                    builder = builder.description(description.clone());
                }
                ActEvent::AddArtist {
                    artist_id,
                    user_id: _,
                } => {
                    artist_ids.insert(*artist_id);
                }
                ActEvent::RemoveArtist {
                    artist_id,
                    user_id: _,
                } => {
                    artist_ids.shift_remove(artist_id);
                }
            }
        }
        builder = builder.artist_ids(artist_ids.into_iter().collect());
        builder.events(events).build()
    }
}

#[derive(Debug)]
pub struct NewAct {
    pub id: ActId,
    pub event_id: EventId,
    pub stage_id: Option<StageId>,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub artist_ids: Vec<ArtistId>,
    pub user_id: UserId,
}

impl NewAct {
    pub(crate) fn from_domain(
        event_id: &DomainEventId,
        act: &CreateActRequest,
        author_id: &DomainUserId,
    ) -> NewAct {
        NewAct {
            id: Uuid::new_v4().into(),
            event_id: (*event_id.as_ref()).into(),
            stage_id: None,
            name: act.name().as_ref().to_string(),
            description: act.description().map(|d| d.as_ref().to_string()),
            image_url: act.act_img().map(|img| img.as_ref().to_string()),
            start_time: None,
            end_time: None,
            artist_ids: act
                .artists()
                .iter()
                .map(|id| (*id.as_ref()).into())
                .collect(),
            user_id: (*author_id.as_ref()).into(),
        }
    }
}

impl IntoEvents<ActEvent> for NewAct {
    fn into_events(self) -> EntityEvents<ActEvent> {
        EntityEvents::init(
            self.id,
            [ActEvent::Initialized {
                id: self.id,
                event_id: self.event_id,
                stage_id: self.stage_id,
                name: self.name,
                description: self.description,
                image_url: self.image_url,
                start_time: self.start_time,
                end_time: self.end_time,
                artist_ids: self.artist_ids,
                user_id: self.user_id,
            }],
        )
    }
}

#[derive(EsRepo, Debug, Clone)]
#[es_repo(
    entity = "Act",
    // Configure the columns that need populating in the index table
    columns(
        // The 'name' column
        name(ty = "String"),
        event_id(ty = "EventId"),
        stage_id(ty = "Option<StageId>"),
        start_time(ty = "Option<chrono::NaiveDateTime>"),
        end_time(ty = "Option<chrono::NaiveDateTime>"),
    )
)]
pub struct ActRepo {
    // Mandatory field so that the Repository can begin transactions
    pub pool: sqlx::PgPool,
}

impl ActRepo {
    pub async fn acts_for_event(&self, event_id: EventId) -> Result<Vec<Act>, ActQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM acts WHERE event_id = $1 ORDER BY name ASC",
            Some(event_id)
        )
        .fetch_n(&self.pool, usize::MAX)
        .await
        .map(|(r, _)| r)
    }

    pub async fn unassigned_acts_for_event(
        &self,
        event_id: EventId,
        num: Option<usize>,
    ) -> Result<Vec<Act>, ActQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM acts WHERE event_id = $1 AND stage_id IS NULL ORDER BY name ASC",
            Some(event_id)
        )
        .fetch_n(&self.pool, num.unwrap_or(usize::MAX))
        .await
        .map(|(r, _)| r)
    }

    pub async fn search_unassigned_acts_for_event(
        &self,
        event_id: EventId,
        query: &str,
        num: Option<usize>,
    ) -> Result<Vec<Act>, ActQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM acts WHERE event_id = $1 AND stage_id IS NULL AND name ILIKE '%' || $2 || '%' ORDER BY name ASC",
            Some(event_id),
            Some(query)
        )
        .fetch_n(&self.pool, num.unwrap_or(usize::MAX))
        .await
        .map(|(r, _)| r)
    }
}
