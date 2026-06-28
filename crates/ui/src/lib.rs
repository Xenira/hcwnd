#![warn(clippy::all, clippy::pedantic)]
#![deny(unsafe_code, clippy::unwrap_used, clippy::expect_used)]

pub mod act;
pub mod artist;
pub mod component;
pub mod data_protection;
pub mod event;
pub mod index;
pub mod user;
pub mod util;

pub trait Ui {
    fn render(&self) -> String;
}
