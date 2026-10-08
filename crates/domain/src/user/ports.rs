use async_trait::async_trait;
use uuid::Uuid;

use crate::user::{
    commands::CreateUser,
    models::{
        self,
        user::{
            AuthenticateUserError, AuthenticateUserRequest, ConfirmEmailError, CreateUserError,
            CreateUserRequest, FindUserError, User, UserEmail, UserId, UserName, UserProjection,
        },
    },
};

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn create_user(&self, req: CreateUser) -> Result<User, CreateUserError>;
    async fn authenticate_user(
        &self,
        req: &AuthenticateUserRequest,
    ) -> Result<User, AuthenticateUserError>;
    async fn confirm_email(&self, name: &UserName, token: Uuid) -> Result<(), ConfirmEmailError>;
    async fn get_user(&self, id: &UserId) -> Result<Option<User>, FindUserError>;
}

#[async_trait]
pub trait UserQuery: Clone + Send + Sync + 'static {
    async fn find_by_username(
        &self,
        username: &UserName,
    ) -> Result<Option<UserProjection>, FindUserError>;
    async fn find_by_email(
        &self,
        email: &UserEmail,
    ) -> Result<Option<UserProjection>, FindUserError>;
}
