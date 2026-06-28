use std::collections::HashMap;

use crate::{
    entity::{
        event::{EventId, EventRepo},
        stage::{Stage, StageId, StageRepo},
    },
    error::Error,
    prelude::*,
};

pub struct EventReqCacheProvider {
    event_repo: EventRepo,
    stage_repo: StageRepo,
}

impl EventReqCacheProvider {
    pub fn new(event_repo: EventRepo, stage_repo: StageRepo) -> Self {
        Self {
            event_repo,
            stage_repo,
        }
    }

    pub fn for_event(&self, event_id: EventId) -> EventReqCache {
        EventReqCache::new(event_id, self.event_repo.clone(), self.stage_repo.clone())
    }
}

pub struct EventReqCache {
    event_id: EventId,
    event_repo: EventRepo,
    stage_repo: StageRepo,
    stages: HashMap<StageId, Stage>,
    stages_fetched: bool,
}

impl EventReqCache {
    pub fn new(event_id: EventId, event_repo: EventRepo, stage_repo: StageRepo) -> Self {
        Self {
            event_id,
            event_repo,
            stage_repo,
            stages: HashMap::new(),
            stages_fetched: false,
        }
    }

    pub async fn get_stage(&mut self, stage_id: StageId) -> Result<&Stage> {
        if !self.stages_fetched {
            let stage = self.stage_repo.find_by_id(stage_id).await?;
            self.stages.insert(stage_id, stage);
        }

        self.stages.get(&stage_id).ok_or(Error::NotFound)
    }

    pub async fn get_stages(&mut self) -> Result<Vec<&Stage>> {
        if !self.stages_fetched {
            let stages = self.stage_repo.stages_for_event(self.event_id).await?;
            for stage in stages {
                self.stages.insert(stage.id, stage);
            }
        }

        return Ok(self.stages.values().collect());
    }
}
