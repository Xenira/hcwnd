use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::models::user::{
    AuthenticateUserError, AuthenticateUserRequest, ConfirmEmailError, CreateUserError,
    CreateUserRequest, FindUserError, User, UserEmail, UserId, UserName,
};

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError>;
    async fn authenticate_user(
        &self,
        req: &AuthenticateUserRequest,
    ) -> Result<User, AuthenticateUserError>;
    async fn confirm_email(&self, name: &UserName, token: Uuid) -> Result<(), ConfirmEmailError>;
    async fn get_user(&self, id: &UserId) -> Result<Option<User>, FindUserError>;
}

#[async_trait]
pub trait UserRepository: Clone + Send + Sync + 'static {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError>;
    async fn find_user_by_id(&self, id: &UserId) -> Result<Option<User>, FindUserError>;
    async fn find_user_by_username(
        &self,
        username: &UserName,
    ) -> Result<Option<User>, FindUserError>;
    async fn find_user_by_email(&self, email: &UserEmail) -> Result<Option<User>, FindUserError>;
}
