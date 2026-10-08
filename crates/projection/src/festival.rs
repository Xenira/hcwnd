use std::ops::Deref;

use anyhow::Context as _;
use async_trait::async_trait;
use chrono::Utc;
use domain::event::{
    models::festival::{Festival, FestivalEvent, FestivalId, ListEventsError},
    ports::FestivalQuery,
};
use eventwise::{TxProjection, aggregate::Aggregate, event::Envelope, store::ProjectionError};
use log::debug;
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresFestivalProjection {
    pool: sqlx::PgPool,
}

impl PostgresFestivalProjection {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TxProjection<Festival> for PostgresFestivalProjection {
    fn kinds(&self) -> &'static [&'static str] {
        todo!()
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'static, Postgres>,
        events: &[Envelope<FestivalEvent, FestivalId>],
    ) -> Result<(), ProjectionError> {
        for envelope in events {
            match &envelope.event {
                FestivalEvent::FestivalSuggested(created) => {
                    sqlx::query!(
                        "INSERT INTO festivals (id, name, start_date, created_at) VALUES ($1, $2, $3, $4)",
                        created.id,
                        created.name,
                        created.start_date,
                        Utc::now()
                    )
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| {
                        ProjectionError::new("Failed to insert user", Some(Box::new(e)))
                    })?;
                }
                _ => {
                    // Handle other events if necessary
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl FestivalQuery for PostgresFestivalProjection {
    async fn upcoming_festivals(&self) -> Result<Vec<FestivalId>, ListEventsError> {
        let result = sqlx::query!(
            "SELECT id FROM festivals WHERE start_date > NOW() ORDER BY start_date ASC LIMIT 10"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch upcoming festivals")?
        .into_iter()
        .map(|row| row.id)
        .map(FestivalId::new)
        .collect();

        Ok(result)
    }
}
