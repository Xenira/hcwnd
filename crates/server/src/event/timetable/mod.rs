use ui::{
    event::{EventBuilder, EventRoute},
    index::{IndexRoute, UiComponent},
};

use crate::{
    entity::event::EventId,
    prelude::*,
    service::{self, get_event_timetable},
};

pub mod add;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_timetable)
        .service(get_timetable_day)
        .service(get_timetable_stage)
        .service(web::scope("/add").configure(add::configure));
}

pub const TIMETABLE_ROUTE_NAME: &str = "timetable";
#[get("", name = "timetable")]
async fn get_timetable(
    req: HttpRequest,
    day_repo: DayRepoData,
    stage_repo: StageRepoData,
    act_repo: ActRepoData,
    event_id: web::Path<EventId>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner();
    let days = day_repo.days_for_event(event_id).await?;
    let multiple_days = days.len() > 1;
    let first_day = days
        .first()
        .expect("Expected at least one day for the event");

    if multiple_days {
        let url = req
            .url_for(
                DAY_ROUTE_NAME,
                [event_id.to_string(), first_day.n.to_string()],
            )
            .expect("Failed to generate URL for timetable day");
        if htmx.is_htmx {
            htmx.push_url(url.as_str());
        } else {
            // If there is only one day, redirect to the lineup page
            return Ok(HttpResponse::TemporaryRedirect()
                .insert_header(("Location", url.as_str()))
                .finish());
        }
    }

    let stages = stage_repo.stages_for_event(event_id).await?;

    if stages.len() > 1 && days.len() == 1 {
        let url = req
            .url_for(
                STAGE_ROUTE_NAME,
                [
                    event_id.to_string(),
                    stages
                        .first()
                        .expect("Expected at least one stage for the event")
                        .id
                        .to_string(),
                ],
            )
            .expect("Failed to generate URL for timetable day");
        if htmx.is_htmx {
            htmx.push_url(url.as_str());
        } else {
            // If there is only one day, redirect to the lineup page
            return Ok(HttpResponse::TemporaryRedirect()
                .insert_header(("Location", url.as_str()))
                .finish());
        }
    }

    let acts = act_repo.acts_for_event(event_id).await?;
    let timetable = get_event_timetable(event_id, None, &days, &acts, &stages);
    let event_route = EventRoute::Timetable(timetable);
    let event = EventBuilder::default()
        .id(event_id.into())
        .outlet(event_route)
        .build()
        .expect("Failed to build event page");

    let body = if htmx.is_htmx {
        event.render_html()
    } else {
        let route = IndexRoute::Event(event);
        service::index("Timetable", route).render_html()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub const DAY_ROUTE_NAME: &str = "timetable_day";
#[get("/day/{event_day}", name = "timetable_day")]
async fn get_timetable_day(
    req: HttpRequest,
    day_repo: DayRepoData,
    stage_repo: StageRepoData,
    act_repo: ActRepoData,
    route: web::Path<(EventId, u16)>,
    htmx: Htmx,
) -> Result<impl Responder> {
    let (event_id, day) = route.into_inner();
    let Some(day) = day_repo.get_day(event_id, day).await? else {
        let url = req
            .url_for(TIMETABLE_ROUTE_NAME, [event_id.to_string()])
            .expect("Failed to generate URL for timetable");
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header(("Location", url.as_str()))
            .finish());
    };

    let stages = stage_repo.stages_for_event(event_id).await?;
    let acts = act_repo.acts_for_event(event_id).await?;
    let days = day_repo.days_for_event(event_id).await?;
    let timetable = get_event_timetable(event_id, Some(day.n), &days, &acts, &stages);

    let body = if htmx.is_htmx {
        // let has_multiple_days = day_repo.has_multiple_days(event_id).await?;
        // service::get_event_day_timetable(event_id, &day, has_multiple_days, &acts, &stages)
        //     .render_html()
        timetable.render_html()
    } else {
        let event_route = EventRoute::Timetable(timetable);
        let event = EventBuilder::default()
            .id(event_id.into())
            .outlet(event_route)
            .build()
            .expect("Failed to build event page");
        let route = IndexRoute::Event(event);
        service::index("Timetable", route).render_html()
    };

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(body))
}

// TODO: Implement this route to filter the timetable by stage
pub const STAGE_ROUTE_NAME: &str = "timetable_stage";
#[get("/stage/{stage}", name = "timetable_stage")]
async fn get_timetable_stage(
    stage_repo: StageRepoData,
    act_repo: ActRepoData,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("Timetable stage"))
}
