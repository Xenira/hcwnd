use std::hash::Hash;

use nutype::nutype;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Stage {
    id: StageId,
    name: StageName,
    description: Option<StageDescription>,
}

impl Stage {
    pub fn new(id: StageId, name: StageName, description: Option<StageDescription>) -> Self {
        Self {
            id,
            name,
            description,
        }
    }

    #[must_use]
    pub fn id(&self) -> &StageId {
        &self.id
    }

    #[must_use]
    pub fn name(&self) -> &StageName {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> Option<&StageDescription> {
        self.description.as_ref()
    }
}

#[nutype(derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef))]
pub struct StageId(Uuid);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct StageName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 1000),
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef)
)]
pub struct StageDescription(String);

pub struct CreateStageRequest {
    name: StageName,
    description: Option<StageDescription>,
}

impl CreateStageRequest {
    pub fn new(name: StageName, description: Option<StageDescription>) -> Self {
        Self { name, description }
    }
}

#[derive(Error, Debug)]
pub enum CreateStageError {
    #[error("A stage with the same name already exists")]
    DuplicateName,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ListStagesError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetStageError {
    #[error("Stage not found")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
