#![warn(clippy::all, clippy::pedantic)]
#![deny(clippy::unwrap_used, unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::user::User;

pub mod act;
pub mod day;
pub mod event;
pub mod stage;
pub mod user;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PagedResult<T, U> {
    pub items: Vec<T>,
    pub has_next_page: bool,
    pub after: Option<U>,
}

pub struct UiState {
    pub locale: String,
    pub user: Option<User>,
}
