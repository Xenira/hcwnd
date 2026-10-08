use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use eventwise::{repository::Repository, store::EventStore};
use uuid::Uuid;

use crate::user::{
    commands::CreateUser,
    models::user::{
        AuthenticateUserError, AuthenticateUserRequest, ConfirmEmailError, CreateUserError,
        CreateUserRequest, FindUserError, User, UserId, UserName,
    },
    ports::{UserQuery, UserService},
};

#[derive(Debug, Clone)]
pub struct Service<S: EventStore, UQ: UserQuery> {
    user_repository: Arc<Repository<User, S>>,
    user_query: UQ,
}

impl<S: EventStore, UQ: UserQuery> Service<S, UQ> {
    pub fn new(user_repository: Repository<User, S>, user_query: UQ) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
            user_query,
        }
    }
}

#[async_trait]
impl<S: EventStore + 'static, UQ: UserQuery> UserService for Service<S, UQ> {
    async fn create_user(&self, req: CreateUser) -> Result<User, CreateUserError> {
        let id = UserId::new(req.user_id);

        self.user_repository
            .as_ref()
            .execute(&id, req)
            .await
            .context("Failed to execute user creation command")?;

        self.get_user(&id)
            .await
            .context("Failed to retrieve the newly created user after creation")?
            .ok_or(CreateUserError::Unknown(anyhow::anyhow!(
                "User creation failed, user not found after creation"
            )))
    }

    async fn authenticate_user(
        &self,
        req: &AuthenticateUserRequest,
    ) -> Result<User, AuthenticateUserError> {
        let user = self
            .user_query
            .find_by_email(req.email())
            .await
            .map_err(|e| match e {
                FindUserError::UserNotFound => AuthenticateUserError::InvalidCredentials,
                FindUserError::Unknown(e) => AuthenticateUserError::Unknown(e),
            })?
            .ok_or(AuthenticateUserError::InvalidCredentials)?;

        // TODO: Sign in command

        let user = self
            .get_user(&UserId::new(user.id))
            .await
            .context("Failed to retrieve user details for authentication")?
            .ok_or(AuthenticateUserError::InvalidCredentials)?;

        user.authenticate(req.password())
            .map_err(|_| AuthenticateUserError::InvalidCredentials)?;

        Ok(user)
    }

    async fn confirm_email(
        &self,
        _name: &UserName,
        _token: uuid::Uuid,
    ) -> Result<(), ConfirmEmailError> {
        todo!(
            "Implement email confirmation logic, verifying the provided token and updating the user's validation status."
        )
    }

    async fn get_user(&self, id: &UserId) -> Result<Option<User>, FindUserError> {
        Ok(self
            .user_repository
            .load(id)
            .await
            .context("Failed to load user from repository")?
            .0)
    }
}
