use eventwise::aggregate::{Apply, Create};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::models::user::{User, UserEmail, UserId, UserName, ValidationToken};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreated {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub validation_token: ValidationToken,
}

impl Create<UserCreated> for User {
    fn bootstrap(event: UserCreated) -> Self {
        User::new(
            UserId::new(event.user_id),
            unsafe { UserName::new_unchecked(event.name) },
            unsafe { UserEmail::new_unchecked(event.email) },
            event.password_hash,
            0,
            Some(event.validation_token),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEmailConfirmed;

impl Apply<UserEmailConfirmed> for User {
    fn apply(&mut self, _event: UserEmailConfirmed) {
        self.set_validated();
    }
}
