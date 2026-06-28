use uuid::Uuid;

use crate::domain::user::models::user::{
    AuthenticateUserError, AuthenticateUserRequest, ConfirmEmailError, CreateUserError,
    CreateUserRequest, FindUserError, User, UserEmail, UserId, UserName,
};

pub trait UserService: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        req: &CreateUserRequest,
    ) -> impl Future<Output = Result<User, CreateUserError>>;
    fn authenticate_user(
        &self,
        req: &AuthenticateUserRequest,
    ) -> impl Future<Output = Result<User, AuthenticateUserError>>;
    fn confirm_email(
        &self,
        name: &UserName,
        token: Uuid,
    ) -> impl Future<Output = Result<(), ConfirmEmailError>>;
    fn get_user(&self, id: &UserId) -> impl Future<Output = Result<Option<User>, FindUserError>>;
}

pub trait UserRepository: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        req: &CreateUserRequest,
    ) -> impl Future<Output = Result<User, CreateUserError>>;
    fn find_user_by_id(
        &self,
        id: &UserId,
    ) -> impl Future<Output = Result<Option<User>, FindUserError>>;
    fn find_user_by_username(
        &self,
        username: &UserName,
    ) -> impl Future<Output = Result<Option<User>, FindUserError>>;
    fn find_user_by_email(
        &self,
        email: &UserEmail,
    ) -> impl Future<Output = Result<Option<User>, FindUserError>>;
}
