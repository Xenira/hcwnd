use actix_web::web;

use crate::entity::{act::ActRepo, day::DaysRepo, event::EventRepo, stage::StageRepo};

mod act;
mod day;
mod event;
mod stage;

pub type EventRepoData = web::Data<EventRepo>;
pub type StageRepoData = web::Data<StageRepo>;
pub type ActRepoData = web::Data<ActRepo>;
pub type DayRepoData = web::Data<DaysRepo>;

pub fn api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(event::config)
            .configure(act::configure)
            .configure(stage::configure)
            .configure(day::configure),
    );
}
