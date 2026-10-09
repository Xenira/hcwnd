use crate::user::events::*;
use api::user::User as ApiUser;
use api::UiState;
use argon2::{
    password_hash::phc::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use eventwise::{
    aggregate::Apply as _, aggregate::Create as _, command::Handle as _, event::StreamId, Aggregate,
};
use nutype::nutype;
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;
use uuid::Uuid;

pub type ValidationToken = (Uuid, chrono::DateTime<chrono::Utc>);

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User with the same email already exists")]
    UserAlreadyExists,
    #[error("Invalid Username")]
    InvalidUsername,
    #[error("Invalid Email")]
    InvalidEmail,
}

impl StreamId for UserId {
    fn key(&self) -> Uuid {
        self.clone().into_inner()
    }
}
impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        UserId::new(id)
    }
}

#[derive(Aggregate, Clone, Debug)]
#[aggregate(error_type = UserError, id_type = UserId)]
#[aggregate(bootstrap(UserCreated))]
#[aggregate(events(UserEmailConfirmed))]
pub struct User {
    id: UserId,
    name: UserName,
    locale: String,
    email: UserEmail,
    password_hash: String,
    reputation: i32,
    validation: Option<ValidationToken>,
}

#[derive(Debug, Clone)]
pub struct UserProjection {
    pub id: Uuid,
    pub name: String,
    // locale: String,
    pub email: String,
    // reputation: i32,
}

impl From<&User> for ApiUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone().into_inner(),
            name: user.name.as_ref().to_string(),
            score: user.reputation,
        }
    }
}

impl From<&User> for UiState {
    fn from(value: &User) -> Self {
        Self {
            user: Some(value.into()),
            locale: value.locale.clone(),
        }
    }
}

impl User {
    pub fn new(
        id: UserId,
        name: UserName,
        email: UserEmail,
        password_hash: String,
        reputation: i32,
        validation: Option<ValidationToken>,
    ) -> Self {
        Self {
            id,
            name,
            locale: "en".to_string(),
            email,
            password_hash,
            reputation,
            validation,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn reputation(&self) -> i32 {
        self.reputation
    }

    pub fn set_validated(&mut self) {
        self.validation = None;
    }

    pub fn is_email_confirmed(&self) -> bool {
        self.validation.is_none()
    }

    pub fn authenticate(&self, password: &UserPassword) -> anyhow::Result<()> {
        password.verify(&self.password_hash)
    }
}

#[nutype(derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRef,
    Deref,
    Serialize,
    Deserialize
))]
pub struct UserId(Uuid);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 3, len_char_max = 32, regex = r"^[a-zA-Z0-9_.]+$"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct UserName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 254),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref),
    new_unchecked
)]
pub struct UserEmail(String);

#[nutype(derive(Debug, AsRef))]
pub struct UserPassword(SecretString);

impl UserPassword {
    pub fn hash(&self, salt: &SaltString) -> Result<String, anyhow::Error> {
        Argon2::default()
            .hash_password_with_salt(self.as_ref().expose_secret().as_bytes(), salt.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))
            .map(|hash| hash.to_string())
    }

    pub fn verify(&self, hash: &str) -> Result<(), anyhow::Error> {
        let hash = PasswordHash::new(hash).map_err(|_| anyhow::anyhow!("Invalid password hash"))?;

        Argon2::default()
            .verify_password(self.as_ref().expose_secret().as_bytes(), &hash)
            .map_err(|_| anyhow::anyhow!("Invalid password"))
    }
}

#[derive(Debug)]
pub struct CreateUserRequest {
    name: UserName,
    email: UserEmail,
    password: UserPassword,
}

impl CreateUserRequest {
    pub fn new(name: UserName, email: UserEmail, password: UserPassword) -> Self {
        Self {
            name,
            email,
            password,
        }
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn hashed_password(&self) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate();
        self.password.hash(&salt)
    }
}

#[derive(Debug)]
pub struct AuthenticateUserRequest {
    email: UserEmail,
    password: UserPassword,
}

impl AuthenticateUserRequest {
    pub fn new(email: UserEmail, password: UserPassword) -> Self {
        Self { email, password }
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }
}

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("User with the same email already exists")]
    UserAlreadyExists,
    #[error("Username is already taken")]
    UsernameAlreadyTaken,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AuthenticateUserError {
    #[error("Invalid email or password")]
    InvalidCredentials,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum FindUserError {
    #[error("User not found")]
    UserNotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ConfirmEmailError {
    #[error("Invalid confirmation token")]
    InvalidToken,
    #[error("Confirmation token has expired")]
    TokenExpired,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
