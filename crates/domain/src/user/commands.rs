use eventwise::command::{CommandError, Handle};

use crate::user::{
    events::UserCreated,
    models::user::{User, UserEmail, UserId, UserName, ValidationToken},
};

pub struct CreateUser {
    pub user_id: UserId,
    pub name: UserName,
    pub email: UserEmail,
    pub password_hash: String,
    pub validation_token: ValidationToken,
}

impl Handle<CreateUser> for User {
    fn create(cmd: CreateUser) -> Result<Vec<Self::Event>, CommandError<Self::Error>> {
        let created_event = UserCreated {
            user_id: cmd.user_id.into_inner(),
            name: cmd.name.into_inner(),
            email: cmd.email.into_inner(),
            password_hash: cmd.password_hash,
            validation_token: cmd.validation_token,
        };

        Ok(vec![Self::Event::UserCreated(created_event)])
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
