use async_trait::async_trait;

use crate::{
    domain::user::{
        models::user::{
            AuthenticateUserError, AuthenticateUserRequest, ConfirmEmailError, CreateUserError,
            CreateUserRequest, FindUserError, User, UserId, UserName,
        },
        ports::{UserRepository, UserService},
    },
    outbound::entity::user::UserFindError,
};

#[derive(Debug, Clone)]
pub struct Service<UR: UserRepository> {
    user_repository: UR,
}

impl<UR: UserRepository> Service<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<UR: UserRepository> UserService for Service<UR> {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError> {
        self.user_repository.create_user(req).await
    }

    async fn authenticate_user(
        &self,
        req: &AuthenticateUserRequest,
    ) -> Result<User, AuthenticateUserError> {
        let user = self
            .user_repository
            .find_user_by_email(&req.email())
            .await
            .map_err(|e| match e {
                FindUserError::UserNotFound => AuthenticateUserError::InvalidCredentials,
                FindUserError::Unknown(e) => AuthenticateUserError::Unknown(e.into()),
            })?
            .ok_or(AuthenticateUserError::InvalidCredentials)?;

        user.authenticate(req.password())
            .map_err(|_| AuthenticateUserError::InvalidCredentials)?;

        Ok(user)
    }

    async fn confirm_email(
        &self,
        name: &UserName,
        token: uuid::Uuid,
    ) -> Result<(), ConfirmEmailError> {
        todo!("Implement email confirmation logic, verifying the provided token and updating the user's validation status.")
    }

    async fn get_user(&self, id: &UserId) -> Result<Option<User>, FindUserError> {
        self.user_repository.find_user_by_id(id).await
    }
}
