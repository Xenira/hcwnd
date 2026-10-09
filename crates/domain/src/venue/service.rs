use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use eventwise::{repository::Repository, store::EventStore};
use uuid::Uuid;

use crate::{
    user::{
        commands::CreateUser,
        models::user::{
            AuthenticateUserError, AuthenticateUserRequest, ConfirmEmailError, CreateUserError,
            CreateUserRequest, FindUserError, User, UserId, UserName,
        },
        ports::{UserQuery, UserService},
    },
    venue::{
        commands::SuggestVenue,
        models::venue::{Venue, VenueError, VenueId, VenueProjection},
        ports::{VenueQuery, VenueService},
    },
};

#[derive(Debug, Clone)]
pub struct Service<S: EventStore, Q: VenueQuery> {
    repository: Arc<Repository<Venue, S>>,
    query: Q,
}

impl<S: EventStore, Q: VenueQuery> Service<S, Q> {
    pub fn new(repository: Repository<Venue, S>, query: Q) -> Self {
        Self {
            repository: Arc::new(repository),
            query,
        }
    }
}

#[async_trait]
impl<S: EventStore + 'static, Q: VenueQuery> VenueService for Service<S, Q> {
    async fn suggest(&self, req: SuggestVenue) -> Result<Venue, VenueError> {
        let id = VenueId::new(Uuid::now_v7());

        self.repository
            .as_ref()
            .execute(&id, req)
            .await
            .context("Failed to execute user creation command")?;

        self.get_by_id(&id).await
    }

    async fn get_by_id(&self, id: &VenueId) -> Result<Venue, VenueError> {
        self.repository
            .load(id)
            .await
            .context("Failed to query user by ID")?
            .0
            .ok_or(VenueError::NotFound)
    }
}

#[async_trait]
impl<S: EventStore + 'static, Q: VenueQuery> VenueQuery for Service<S, Q> {
    async fn search(&self, query: &str) -> Result<Vec<VenueProjection>, VenueError> {
        self.query.search(query).await
    }
}
