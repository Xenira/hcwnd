use actix_web::{get, post, web, HttpResponse};
use api::act::ActDetails;
use itertools::Itertools as _;
use uuid::Uuid;

use crate::{
    controller::ActRepoData,
    entity::{act::NewActBuilder, event::EventId},
    prelude::*,
};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_acts).service(create_act);
}

#[get("/events/{event_id}/acts")]
async fn get_acts(
    acts: ActRepoData,
    event_id: web::Path<EventId>,
) -> Result<impl actix_web::Responder> {
    Ok(HttpResponse::Ok().json(
        acts.acts_for_event(event_id.into_inner())
            .await?
            .into_iter()
            .map_into::<ActDetails>()
            .collect_vec(),
    ))
}

#[post("/events/{event_id}/acts")]
async fn create_act(
    acts: ActRepoData,
    event_id: web::Path<EventId>,
    new_act: web::Json<api::act::NewAct>,
) -> Result<impl actix_web::Responder> {
    let new_act = NewActBuilder::default()
        .id(Uuid::new_v4())
        .event_id(event_id.into_inner())
        .name(&new_act.name)
        .description(new_act.description.clone())
        .image_url(new_act.image_url.clone())
        .user_id(Uuid::new_v4())
        .build()?;
    let act = acts.create(new_act).await?;

    Ok(HttpResponse::Created().json(ActDetails::from(act)))
}
