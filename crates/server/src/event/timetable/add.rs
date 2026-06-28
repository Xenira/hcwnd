use chrono::NaiveTime;
use es_entity::Idempotent;
use imgproxy::SignedUrlRepo;
use serde::Deserialize;
use ui::{
    act::card::ActCardBuilder,
    event::timetable::{
        ActSearchResultsBuilder, SelectActBuilder, SelectActResult, SelectStageBuilder,
        SelectStageResult, SetTimeBuilder, StageSearchResult, StageSearchResultsBuilder,
    },
};

use crate::{
    entity::{
        act::{ActId, NewActBuilder},
        day::DayId,
        event::EventId,
        stage::{NewStageBuilder, StageId},
    },
    event::timetable::TIMETABLE_ROUTE_NAME,
    prelude::*,
    service::act::get_act_cards,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_timetable_add)
        .service(search_timetable_add)
        .service(get_timetable_add_to_stage)
        .service(search_stage_timetable_add)
        .service(get_timetable_add_time)
        .service(add_act_to_timetable);
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AddToStageData {
    CreateAct { create_act: String },
    SelectAct { select_act: ActId },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum StageSelection {
    CreateStage { create_stage: String },
    SelectStage { select_stage: StageId },
}

#[derive(Deserialize)]
pub struct SetTimeData {
    #[serde(flatten)]
    act: AddToStageData,
    #[serde(flatten)]
    stage: StageSelection,
}

#[derive(Deserialize)]
pub struct AddToTimetableData {
    #[serde(flatten)]
    act: AddToStageData,
    #[serde(flatten)]
    stage: StageSelection,
    day: DayId,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

#[get("/act", name = "timetable_add")]
async fn get_timetable_add(
    req: HttpRequest,
    act_repo: ActRepoData,
    stage_repo: StageRepoData,
    url_repo: web::Data<SignedUrlRepo>,
    event_id: web::Path<EventId>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();
    let acts = act_repo
        .unassigned_acts_for_event(event_id, Some(5))
        .await?;

    let stages = stage_repo.get_stage_names(event_id).await?;
    let act_cards = get_act_cards(&acts, &stages, &url_repo).await?;

    let search_results = ActSearchResultsBuilder::default()
        .acts(act_cards)
        .create_new(None)
        .build()
        .expect("Failed to build search results component");

    let select_act = SelectActBuilder::default()
        .event_id(event_id)
        .search_results(search_results)
        .build()
        .expect("Failed to build select act component");

    let body = if htmx.is_htmx {
        select_act.render_html()
    } else {
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                "Location",
                req.url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])
                    .expect("Failed to generate URL for timetable")
                    .as_str(),
            ))
            .finish());
    };

    Ok(HttpResponse::Ok().body(body))
}

/// Adds an act to the timetable, without having selected a time.
#[post("/act", name = "timetable_add")]
async fn add_act_to_timetable(
    req: HttpRequest,
    act_repo: ActRepoData,
    stage_repo: StageRepoData,
    day_repo: DayRepoData,
    event_id: web::Path<EventId>,
    data: web::Form<AddToTimetableData>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();
    let user_id = Uuid::new_v4();
    let Some(day) = day_repo.get_day_by_id(event_id, data.day).await? else {
        return Ok(HttpResponse::NotFound().finish());
    };

    let stage_id = match &data.stage {
        StageSelection::CreateStage { create_stage } => {
            let new_stage = NewStageBuilder::default()
                .event_id(event_id)
                .user_id(user_id)
                .name(create_stage)
                .build()?;
            stage_repo.create(new_stage).await?.id
        }
        StageSelection::SelectStage { select_stage } => select_stage.clone(),
    };
    match &data.act {
        AddToStageData::CreateAct { create_act } => {
            let new_act = NewActBuilder::default()
                .event_id(event_id)
                .user_id(user_id)
                .name(create_act)
                .stage_id(Some(stage_id))
                .build()?;
            act_repo.create(new_act).await?;
        }
        AddToStageData::SelectAct { select_act } => {
            let mut act = act_repo.find_by_id(select_act).await?;
            if let Idempotent::Executed(act) = act.set_stage(Some(stage_id), user_id.into()) {
                act_repo.update(act).await?;
            };
        }
    };

    let timetable_url = req.url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])?;

    Ok(if htmx.is_htmx {
        htmx.redirect(timetable_url);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::TemporaryRedirect()
            .insert_header(("Location", timetable_url.as_str()))
            .finish()
    })
}

#[get("/act/search", name = "timetable_add_search")]
async fn search_timetable_add(
    req: HttpRequest,
    act_repo: ActRepoData,
    stage_repo: StageRepoData,
    url_repo: web::Data<SignedUrlRepo>,
    query: web::Query<SearchQuery>,
    event_id: web::Path<EventId>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();

    let new_act = if query.q.trim().is_empty() {
        None
    } else {
        Some(
            ActCardBuilder::default()
                .id(Uuid::nil())
                .name(query.q.clone())
                .image_url(None)
                .build()
                .expect("Failed to build new act"),
        )
    };

    let mut acts = act_repo
        .search_unassigned_acts_for_event(event_id, &query.q, Some(5))
        .await?;

    let new_act = if acts
        .iter()
        .any(|a| a.name.to_lowercase() == query.q.to_lowercase())
    {
        None
    } else {
        new_act
    };

    if new_act.is_some() {
        acts.truncate(4);
    };

    let stages = stage_repo.get_stage_names(event_id).await?;
    let act_cards = get_act_cards(&acts, &stages, &url_repo).await?;
    let search_results = ActSearchResultsBuilder::default()
        .acts(act_cards)
        .create_new(new_act)
        .build()
        .expect("Failed to build search results component");

    let body = if htmx.is_htmx {
        search_results.render_html()
    } else {
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                "Location",
                req.url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])
                    .expect("Failed to generate URL for timetable")
                    .as_str(),
            ))
            .finish());
    };

    Ok(HttpResponse::Ok().body(body))
}

#[get("/stage", name = "timetable_add_to_stage")]
async fn get_timetable_add_to_stage(
    req: HttpRequest,
    stage_repo: StageRepoData,
    event_id: web::Path<EventId>,
    data: web::Query<AddToStageData>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();
    let stages = stage_repo
        .stages_for_event(event_id)
        .await?
        .into_iter()
        .map_into()
        .collect_vec();
    let search_results = StageSearchResultsBuilder::default()
        .stages(stages)
        .create_new(None)
        .build()
        .expect("Failed to build stage search results component");
    let select_stage = SelectStageBuilder::default()
        .event_id(event_id)
        .act(match data.into_inner() {
            AddToStageData::CreateAct { create_act } => SelectActResult::Create(create_act),
            AddToStageData::SelectAct { select_act } => SelectActResult::Select(select_act.into()),
        })
        .search_results(search_results)
        .build()
        .expect("Failed to build select stage component");

    let body = if htmx.is_htmx {
        select_stage.render_html()
    } else {
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                "Location",
                req.url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])
                    .expect("Failed to generate URL for timetable")
                    .as_str(),
            ))
            .finish());
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/stage/search", name = "timetable_add_search")]
async fn search_stage_timetable_add(
    req: HttpRequest,
    stage_repo: StageRepoData,
    query: web::Query<SearchQuery>,
    event_id: web::Path<EventId>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();

    let new_stage = if query.q.trim().is_empty() {
        None
    } else {
        Some(StageSearchResult::New {
            name: query.q.clone(),
        })
    };

    let stages = stage_repo.search_for_event(event_id, &query.q).await?;

    let new_stage = if stages
        .iter()
        .any(|a| a.name.to_lowercase() == query.q.to_lowercase())
    {
        None
    } else {
        new_stage
    };

    let search_results = StageSearchResultsBuilder::default()
        .stages(stages.into_iter().map_into().collect_vec())
        .create_new(new_stage)
        .build()
        .expect("Failed to build stage search results component");

    let body = if htmx.is_htmx {
        search_results.render_html()
    } else {
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                "Location",
                req.url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])
                    .expect("Failed to generate URL for timetable")
                    .as_str(),
            ))
            .finish());
    };

    Ok(HttpResponse::Ok().body(body))
}

const ADD_TO_TIMETABLE_TIME_ROUTE_NAME: &str = "timetable_add_time";
#[get("/time", name = "timetable_add_time")]
async fn get_timetable_add_time(
    req: HttpRequest,
    day_repo: DayRepoData,
    event_id: web::Path<EventId>,
    data: web::Query<SetTimeData>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();
    let data = data.into_inner();
    let days = day_repo.days_for_event(event_id).await?;

    let set_time = SetTimeBuilder::default()
        .event_id(event_id)
        .act(match data.act {
            AddToStageData::CreateAct { create_act } => SelectActResult::Create(create_act),
            AddToStageData::SelectAct { select_act } => SelectActResult::Select(select_act.into()),
        })
        .stage(match data.stage {
            StageSelection::CreateStage { create_stage } => SelectStageResult::Create(create_stage),
            StageSelection::SelectStage { select_stage } => {
                SelectStageResult::Select(select_stage.into())
            }
        })
        .days(
            days.into_iter()
                .map(|d| (d.id.into(), d.start_time, d.end_time))
                .collect_vec(),
        )
        .build()
        .expect("Failed to build set time component");

    let body = if htmx.is_htmx {
        set_time.render_html()
    } else {
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                "Location",
                req.url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])
                    .expect("Failed to generate URL for timetable")
                    .as_str(),
            ))
            .finish());
    };

    Ok(HttpResponse::Ok().body(body))
}
