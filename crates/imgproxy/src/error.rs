use thiserror::Error;

use crate::ResizingOptionsBuilderError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Failed to build resizing options: {0}")]
    ResizingOptionsBuilder(#[from] ResizingOptionsBuilderError),
}
