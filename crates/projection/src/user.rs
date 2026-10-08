use std::ops::Deref;

use anyhow::Context as _;
use async_trait::async_trait;
use chrono::Utc;
use domain::user::{
    models::{
        self,
        user::{FindUserError, User, UserEmail, UserEvent, UserId, UserName, UserProjection},
    },
    ports::UserQuery,
};
use eventwise::{aggregate::Aggregate, event::Envelope, store::ProjectionError, TxProjection};
use log::debug;
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresUserProjection {
    pool: sqlx::PgPool,
}

impl PostgresUserProjection {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TxProjection<User> for PostgresUserProjection {
    fn kinds(&self) -> &'static [&'static str] {
        todo!()
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'static, Postgres>,
        events: &[Envelope<UserEvent, UserId>],
    ) -> Result<(), ProjectionError> {
        for envelope in events {
            match &envelope.event {
                UserEvent::UserCreated(created) => {
                    sqlx::query!(
                        "INSERT INTO users (id, name, email, sanitized_email, created_at) VALUES ($1, $2, $3, $4, $5)",
                        created.user_id,
                        created.name,
                        created.email,
                        created.email.to_lowercase(),
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
impl UserQuery for PostgresUserProjection {
    async fn find_by_username(
        &self,
        username: &UserName,
    ) -> Result<Option<UserProjection>, FindUserError> {
        Ok(sqlx::query_as!(
            UserProjection,
            "SELECT id, name, email FROM users WHERE name = $1",
            username.deref()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by username")?)
    }

    async fn find_by_email(
        &self,
        email: &UserEmail,
    ) -> Result<Option<UserProjection>, FindUserError> {
        let result = sqlx::query_as!(
            UserProjection,
            "SELECT id, name, email FROM users WHERE email = $1",
            email.deref()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by email")?;

        debug!("find_by_email result: {:?}", result);

        Ok(result)
    }
}
