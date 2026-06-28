use derive_builder::Builder;
use es_entity::{
    EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo, IntoEvents, TryFromEvents,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::models::user::{
    CreateUserRequest, User as DomainUser, UserEmail, UserId as DomainUserId, UserName,
    UserPassword,
};

es_entity::entity_id! { UserId }

#[derive(EsEvent, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "UserId")]
pub enum UserEvent {
    Initialized {
        id: UserId,
        name: String,
        email: String,
        sanitized_email: String,
        password_hash: String,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub sanitized_email: String,
    password_hash: String,
    #[builder(default = "0")]
    pub reputation: i32,
    #[builder(default)]
    pub verification: Option<(Uuid, chrono::DateTime<chrono::Utc>)>,

    events: EntityEvents<UserEvent>,
}

impl User {
    pub fn authenticate(&self, password: UserPassword) -> anyhow::Result<()> {
        password.verify(&self.password_hash)
    }
}

impl TryFrom<User> for DomainUser {
    type Error = anyhow::Error;

    fn try_from(value: User) -> anyhow::Result<Self> {
        let id = DomainUserId::new(value.id.into());
        let name = UserName::try_new(value.name)?;
        let email = UserEmail::try_new(value.email)?;
        let password_hash = value.password_hash;
        let reputation = value.reputation;
        let validation = value.verification;

        Ok(DomainUser::new(
            id,
            name,
            email,
            password_hash,
            reputation,
            validation,
        ))
    }
}

// Any EsEntity must implement `TryFromEvents`.
// This trait is what hydrates entities after loading the events from the database
impl TryFromEvents<UserEvent> for User {
    fn try_from_events(events: EntityEvents<UserEvent>) -> Result<Self, EntityHydrationError> {
        let mut builder = UserBuilder::default();
        for event in events.iter_all() {
            match event {
                UserEvent::Initialized {
                    id,
                    name,
                    email,
                    sanitized_email,
                    password_hash,
                } => {
                    builder = builder
                        .id(*id)
                        .name(name.clone())
                        .email(email.clone())
                        .sanitized_email(sanitized_email.clone())
                        .password_hash(password_hash.clone())
                }
            }
        }
        builder.events(events).build()
    }
}

pub struct NewUser {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub sanitized_email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn from_domain(req: &CreateUserRequest) -> anyhow::Result<Self> {
        Ok(Self {
            id: Uuid::new_v4().into(),
            name: req.name().as_ref().to_string(),
            email: req.email().as_ref().to_string(),
            sanitized_email: req.email().as_ref().to_string().to_lowercase(),
            password_hash: req.hashed_password()?,
        })
    }
}

impl IntoEvents<UserEvent> for NewUser {
    fn into_events(self) -> EntityEvents<UserEvent> {
        EntityEvents::init(
            self.id,
            [UserEvent::Initialized {
                id: self.id,
                name: self.name,
                email: self.email,
                sanitized_email: self.sanitized_email,
                password_hash: self.password_hash,
            }],
        )
    }
}

#[derive(EsRepo, Debug, Clone)]
#[es_repo(
    entity = "User",
    // Configure the columns that need populating in the index table
    columns(
        name(ty = "String"),
        email(ty = "String"),
        sanitized_email(ty = "String"),
    )
)]
pub struct UserRepo {
    // Mandatory field so that the Repository can begin transactions
    pub pool: sqlx::PgPool,
}
