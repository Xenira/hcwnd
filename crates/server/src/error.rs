use actix_web::{error::UrlGenerationError, HttpResponseBuilder, ResponseError};
use thiserror::Error;

use crate::entity::{
    act::{ActCreateError, ActFindError, ActModifyError, ActQueryError, NewActBuilderError},
    day::{DayCreateError, DayQueryError, NewDayBuilderError},
    event::{EventCreateError, EventFindError, EventQueryError, NewEventBuilderError},
    stage::{NewStageBuilderError, StageCreateError, StageFindError, StageQueryError},
};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(String),
    // === Routing ===
    #[error("Failed to generate URL: {0}")]
    UrlGeneration(#[from] UrlGenerationError),
    // TimeyWimeyError(#[from] chrono::E
    #[error("Failed to generate image URL: {0}")]
    ImageProxy(#[from] imgproxy::error::Error),
    #[error("Failed to build resizing options: {0}")]
    ResizingOptionsBuilder(#[from] imgproxy::ResizingOptionsBuilderError),
    // === Act errors ===
    #[error("Failed to build new act: {0}")]
    NewActBuilder(#[from] NewActBuilderError),
    #[error("Failed to create act: {0}")]
    ActCreate(#[from] ActCreateError),
    #[error("Failed to search acts: {0}")]
    ActQuery(#[from] ActQueryError),
    #[error("Failed to find acts: {0}")]
    ActFind(#[from] ActFindError),
    #[error("Failed to update act: {0}")]
    ActModify(#[from] ActModifyError),
    // === Stage errors ===
    #[error("Failed to create stage: {0}")]
    StageCreate(#[from] StageCreateError),
    #[error("Failed to build new stage: {0}")]
    NewStageBuilder(#[from] NewStageBuilderError),
    #[error("Failed to query stages: {0}")]
    StageQuery(#[from] StageQueryError),
    #[error("Failed to find stage: {0}")]
    StageFind(#[from] StageFindError),
    // === Event errors ===
    #[error("Failed to create event: {0}")]
    EventCreate(#[from] EventCreateError),
    #[error("Failed to build new event: {0}")]
    NewEventBuilder(#[from] NewEventBuilderError),
    #[error("Failed to query events: {0}")]
    EventQuery(#[from] EventQueryError),
    #[error("Failed to find event: {0}")]
    EventFind(#[from] EventFindError),
    // === Day errors ===
    #[error("Failed to build new day: {0}")]
    NewDayBuilder(#[from] NewDayBuilderError),
    #[error("Failed to create day: {0}")]
    DayCreate(#[from] DayCreateError),
    #[error("Failed to query days: {0}")]
    DayQuery(#[from] DayQueryError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut response_builder = HttpResponseBuilder::new(self.status_code());
        response_builder.insert_header(("Content-Type", "application/json"));

        match self {
            _ => response_builder.body(r#"{"error": "Internal Server Error"}"#),
        }
    }
}
