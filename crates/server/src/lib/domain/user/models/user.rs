use api::UiState;
use api::user::User as ApiUser;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use nutype::nutype;
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;
use uuid::Uuid;

type ValidationToken = (Uuid, chrono::DateTime<chrono::Utc>);

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    name: UserName,
    locale: String,
    email: UserEmail,
    password_hash: String,
    reputation: i32,
    validation: Option<ValidationToken>,
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

    pub fn is_email_confirmed(&self) -> bool {
        self.validation.is_none()
    }

    pub fn authenticate(&self, password: &UserPassword) -> anyhow::Result<()> {
        password.verify(&self.password_hash)
    }
}

#[nutype(derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef))]
pub struct UserId(Uuid);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 3, len_char_max = 32, regex = r"^[a-zA-Z0-9_.]+$"),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct UserName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 254),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct UserEmail(String);

#[nutype(derive(Debug, AsRef))]
pub struct UserPassword(SecretString);

impl UserPassword {
    pub fn hash(&self, salt: &SaltString) -> Result<String, anyhow::Error> {
        Argon2::default()
            .hash_password(self.as_ref().expose_secret().as_bytes(), salt)
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
        let salt = SaltString::generate(&mut OsRng);
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
