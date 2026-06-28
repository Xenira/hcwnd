use std::collections::HashMap;

use api::stage::StageDetails;
use derive_builder::Builder;
use es_entity::{
    es_query, EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo, IntoEvents,
    TryFromEvents,
};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::query;
use ui::event::timetable::StageSearchResult;
use uuid::Uuid;

use crate::domain::event::models::stage::{
    Stage as DomainStage, StageId as DomainStageId, StageName,
};
use crate::outbound::entity::{event::EventId, user::UserId};

es_entity::entity_id! { StageId }

#[derive(EsEvent, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "StageId")]
pub enum StageEvent {
    Initialized {
        id: StageId,
        event_id: EventId,
        name: String,
        user_id: UserId,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct Stage {
    pub id: StageId,
    pub event_id: EventId,
    pub name: String,

    events: EntityEvents<StageEvent>,
}

impl TryFrom<Stage> for DomainStage {
    type Error = anyhow::Error;

    fn try_from(value: Stage) -> anyhow::Result<Self> {
        info!(
            "Transforming Stage {}, {}, {} into DomainStage",
            value.id, value.event_id, value.name
        );
        let id = DomainStageId::new(value.id.into());
        let name = StageName::try_new(value.name)?;
        let description = None; // Description is not stored in the current implementation, so we set it to None

        Ok(DomainStage::new(id, name, description))
    }
}

// Any EsEntity must implement `TryFromEvents`.
// This trait is what hydrates entities after loading the events from the database
impl TryFromEvents<StageEvent> for Stage {
    fn try_from_events(events: EntityEvents<StageEvent>) -> Result<Self, EntityHydrationError> {
        let mut builder = StageBuilder::default();

        for event in events.iter_all() {
            match event {
                StageEvent::Initialized {
                    id,
                    event_id,
                    name,
                    user_id: _,
                } => builder = builder.id(*id).event_id(*event_id).name(name.clone()),
            }
        }
        builder.events(events).build()
    }
}

#[derive(Debug, Builder)]
pub struct NewStage {
    #[builder(setter(into), default = "Uuid::new_v4().into()")]
    pub id: StageId,
    #[builder(setter(into))]
    pub event_id: EventId,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub user_id: UserId,
}

impl IntoEvents<StageEvent> for NewStage {
    fn into_events(self) -> EntityEvents<StageEvent> {
        EntityEvents::init(
            self.id,
            [StageEvent::Initialized {
                id: self.id,
                event_id: self.event_id,
                name: self.name,
                user_id: self.user_id,
            }],
        )
    }
}

#[derive(EsRepo, Debug, Clone)]
#[es_repo(
    entity = "Stage",
    // Configure the columns that need populating in the index table
    columns(
        // The 'name' column
        name(ty = "String"),
        event_id(ty = "EventId", list_by),
    )
)]
pub struct StageRepo {
    // Mandatory field so that the Repository can begin transactions
    pub pool: sqlx::PgPool,
}

impl StageRepo {
    pub async fn stages_for_event(&self, event_id: EventId) -> Result<Vec<Stage>, StageQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!("SELECT * FROM stages WHERE event_id = $1", Some(event_id))
            .fetch_n(&self.pool, usize::MAX)
            .await
            .map(|(r, _)| r)
    }

    pub async fn search_for_event(
        &self,
        event_id: EventId,
        query: &str,
    ) -> Result<Vec<Stage>, StageQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM stages WHERE event_id = $1 AND name ILIKE '%' || $2 || '%' ORDER BY name ASC",
            Some(event_id),
            Some(query)
        )
        .fetch_n(&self.pool, usize::MAX)
        .await
        .map(|(r, _)| r)
    }

    pub async fn get_stage_names(
        &self,
        event_id: EventId,
    ) -> anyhow::Result<HashMap<StageId, String>> {
        let event_id: Uuid = event_id.into();
        Ok(
            query!("SELECT id, name FROM stages WHERE event_id = $1", event_id)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|record| (record.id.into(), record.name))
                .collect::<HashMap<_, _>>(),
        )
    }
}
