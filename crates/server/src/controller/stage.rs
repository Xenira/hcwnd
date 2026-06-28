use actix_web::{get, post, web, HttpResponse};
use api::stage::StageDetails;
use itertools::Itertools as _;
use uuid::Uuid;

use crate::{
    controller::StageRepoData,
    entity::{event::EventId, stage::NewStageBuilder},
};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_stages).service(create_stage);
}

#[get("/events/{event_id}/stages")]
async fn get_stages(
    stages: StageRepoData,
    event_id: web::Path<EventId>,
) -> impl actix_web::Responder {
    HttpResponse::Ok().json(
        stages
            .stages_for_event(event_id.into_inner())
            .await
            .unwrap()
            .into_iter()
            .map_into::<StageDetails>()
            .collect_vec(),
    )
}

#[post("/events/{event_id}/stages")]
async fn create_stage(
    stages: StageRepoData,
    event_id: web::Path<EventId>,
    new_stage: web::Json<api::stage::NewStage>,
) -> impl actix_web::Responder {
    let stage = stages
        .create(
            NewStageBuilder::default()
                .id(Uuid::new_v4())
                .event_id(event_id.into_inner())
                .name(&new_stage.name)
                .user_id(Uuid::new_v4())
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    HttpResponse::Created().json(StageDetails::from(stage))
}
