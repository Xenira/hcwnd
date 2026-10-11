#![warn(clippy::all, clippy::pedantic)]
#![deny(clippy::unwrap_used, unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::user::User;

pub mod act;
pub mod artist;
pub mod day;
pub mod festival;
pub mod routes;
pub mod serde_utils;
pub mod stage;
pub mod user;
pub mod venue;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PagedResult<T, U> {
    pub items: Vec<T>,
    pub has_next_page: bool,
    pub after: Option<U>,
}

#[derive(Clone)]
pub struct UiState {
    pub locale: String,
    pub user: Option<User>,
}
