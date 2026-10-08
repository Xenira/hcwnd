use argon2::password_hash::phc::SaltString;
use eventwise::command::{CommandError, Handle};
use secrecy::SecretString;
use uuid::Uuid;

use crate::user::{
    events::UserCreated,
    models::user::{User, UserEmail, UserError, UserId, UserName, UserPassword, ValidationToken},
};

pub struct CreateUser {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: SecretString,
    pub validation_token: ValidationToken,
}

impl CreateUser {
    pub fn new(name: String, email: String, password: SecretString) -> Self {
        let user_id = Uuid::new_v4();
        let validation_token = (
            Uuid::new_v4(),
            chrono::Utc::now() + chrono::Duration::hours(24),
        );
        Self {
            user_id,
            name,
            email,
            password,
            validation_token,
        }
    }
}

impl Handle<CreateUser> for User {
    fn create(cmd: CreateUser) -> Result<((), Vec<Self::Event>), CommandError<Self::Error>> {
        let user_id = UserId::new(cmd.user_id);
        let name = UserName::try_new(cmd.name)
            .map_err(|e| CommandError::Domain(UserError::InvalidUsername))?;
        let email = UserEmail::try_new(cmd.email)
            .map_err(|e| CommandError::Domain(UserError::InvalidEmail))?;
        let password_hash = UserPassword::new(cmd.password)
            .hash(&SaltString::generate())
            .map_err(|e| CommandError::Domain(UserError::InvalidUsername))?;
        let created_event = UserCreated {
            user_id: user_id.into_inner(),
            name: name.into_inner(),
            email: email.into_inner(),
            password_hash,
            validation_token: cmd.validation_token,
        };

        Ok(((), vec![Self::Event::UserCreated(created_event)]))
    }
}

#[cfg(test)]
mod tests {
    use crate::user::models::user::UserEvent;

    use super::*;

    #[test]
    fn test_create_user_command() {
        let uuid = uuid::Uuid::new_v4();
        let user_id = UserId::new(uuid);
        let name = UserName::try_new("TestUser".to_string()).expect("Failed to create UserName");
        let email = UserEmail::try_new("foo.bar@example.com".to_string())
            .expect("Failed to create UserEmail");

        let password_hash = "hashed_password".to_string();
        let validation_token = (uuid, chrono::Utc::now());

        let create_user_cmd = CreateUser {
            user_id: user_id.clone(),
            name: name.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
            validation_token,
        };

        let result = User::create(create_user_cmd);
        assert!(result.is_ok());
        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        if let UserEvent::UserCreated(event) = &events[0] {
            assert_eq!(event.user_id, user_id.into_inner());
            assert_eq!(event.name, name.into_inner());
            assert_eq!(event.email, email.into_inner());
            assert_eq!(event.password_hash, password_hash);
            assert_eq!(event.validation_token, validation_token);
        } else {
            panic!("Expected UserCreated event");
        }
    }
}
