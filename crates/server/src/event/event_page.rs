use std::ops::Deref;

use actix_htmx::Htmx;
use actix_web::{get, post, web, HttpResponse, Responder};
use imgproxy::SignedUrlRepo;
use itertools::Itertools as _;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::NoneAsEmptyString;
use ui::act::card::ActCard;
use ui::{
    event::{details::EventDetails, lineup::EventLineupBuilder, EventRoute},
    index::{IndexRoute, UiComponent as _},
};

use crate::event::timetable;
use crate::event_req_cache::EventReqCache;
use crate::event_req_cache::EventReqCacheProvider;
use crate::prelude::StageRepoData;
use crate::service::act::get_act_cards_for_event;
use crate::{
    controller::{ActRepoData, EventRepoData},
    entity::event::EventId,
    service,
};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_event)
        .service(get_event_lineup)
        .service(add_event_act)
        .service(web::scope("/timetable").configure(timetable::configure));
}

#[get("")]
async fn get_event(
    event_repo: EventRepoData,
    path: web::Path<EventId>,
    htmx: Htmx,
) -> crate::error::Result<impl Responder> {
    let event_id = path.into_inner();
    let event_details: EventDetails = event_repo.find_by_id(event_id).await?.into();
    let event_title = event_details.title.clone();
    let event_route = EventRoute::Details(event_details);
    let event = ui::event::EventBuilder::default()
        .id(event_id.into())
        .outlet(event_route)
        .build()
        .expect("Failed to build event page");

    let body = if htmx.is_htmx {
        event.render_html()
    } else {
        let index_route = IndexRoute::Event(event);
        service::index(event_title.as_str(), index_route).render_html()
    };

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(body))
}

#[get("/lineup")]
async fn get_event_lineup(
    act_repo: ActRepoData,
    stage_repo: StageRepoData,
    image_repo: web::Data<SignedUrlRepo>,
    path: web::Path<EventId>,
    htmx: Htmx,
) -> crate::error::Result<impl Responder> {
    let event_id = path.into_inner();

    let stages = stage_repo.get_stage_names(event_id).await?;
    let acts = get_act_cards_for_event(&act_repo, &stages, &image_repo, event_id).await?;
    let lineup = EventLineupBuilder::default()
        .event_id(event_id.into())
        .acts(acts)
        .stages(
            stages
                .into_iter()
                .map(|(id, name)| (id.into(), name))
                .collect(),
        )
        .build()
        .expect("Failed to build event lineup");
    let event_route = EventRoute::Lineup(lineup);
    let event = ui::event::EventBuilder::default()
        .id(event_id.into())
        .outlet(event_route)
        .build()
        .expect("Failed to build event page");

    let body = if htmx.is_htmx {
        event.render_html()
    } else {
        let index_route = IndexRoute::Event(event);
        service::index("Lineup", index_route).render_html()
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct AddActData {
    name: String,
    #[serde_as(as = "NoneAsEmptyString")]
    image_url: Option<String>,
}

#[post("/act")]
async fn add_event_act(
    act_repo: ActRepoData,
    stage_repo: StageRepoData,
    image_repo: web::Data<SignedUrlRepo>,
    event_id: web::Path<EventId>,
    form: web::Form<AddActData>,
) -> crate::error::Result<impl Responder> {
    let event_id = event_id.into_inner();

    service::act::create_act(
        &act_repo,
        event_id,
        uuid::Uuid::new_v4(),
        form.name.clone(),
        form.image_url.clone(),
    )
    .await?;

    let stages = stage_repo.get_stage_names(event_id).await?;
    let acts = get_act_cards_for_event(&act_repo, &stages, &image_repo, event_id).await?;

    let lineup = EventLineupBuilder::default()
        .event_id(event_id.into())
        .acts(acts)
        .stages(
            stages
                .into_iter()
                .map(|(id, name)| (id.into(), name))
                .collect(),
        )
        .build()
        .expect("Failed to build event lineup");

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(lineup.render_html()))
}
