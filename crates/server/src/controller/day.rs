use actix_web::{get, web, HttpResponse};
use api::day::EventDay;
use itertools::Itertools as _;

use crate::{controller::DayRepoData, entity::event::EventId, prelude::*};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_days);
}

#[get("/events/{event_id}/days")]
async fn get_days(
    days_repo: DayRepoData,
    event_id: web::Path<EventId>,
) -> Result<impl actix_web::Responder> {
    Ok(HttpResponse::Ok().json(
        days_repo
            .days_for_event(event_id.into_inner())
            .await?
            .into_iter()
            .map_into::<EventDay>()
            .collect_vec(),
    ))
}
