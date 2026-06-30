use actix_web::{HttpResponse, ResponseError, web};
use anyhow::Context;
use thiserror::Error;
use ui::event::lineup::{EventLineup, EventLineupBuilder};

use crate::{
    domain::{artist::ports::ArtistService, event::models::event::Event},
    inbound::http::handlers::event::details::act::act_card,
};

pub fn configure(_cfg: &mut web::ServiceConfig) {
    // cfg.service(get_lineup);
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    ServiceError(#[from] anyhow::Error),
}

impl ResponseError for HandlerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HandlerError::ServiceError(e) => {
                HttpResponse::InternalServerError().body(format!("Service error: {e}"))
            }
        }
    }
}

// #[get("")]
// async fn get_lineup(
//     app_state: web::Data<AppState>,
//     path: web::Path<Uuid>,
//     htmx: Htmx,
// ) -> Result<impl Responder, HandlerError> {
//     let event_id = EventId::new(path.into_inner());
//     let event = app_state
//         .event_service
//         .get_event_by_id(&event_id)
//         .await
//         .context("Failed to fetch event")?;
//
//     // let stages = stage_repo.get_stage_names(event_id).await?;
//     // let acts = get_act_cards_for_event(&act_repo, &stages, &image_repo, event_id).await?;
//     let lineup = event_lineup(&event, app_state.artist_service.as_ref())
//         .await
//         .context("Failed to build event lineup")?;
//
//     let event_route = EventRoute::Lineup(lineup);
//     let event = ui::event::EventBuilder::default()
//         .id(event_id.into_inner())
//         .outlet(event_route)
//         .build()
//         .expect("Failed to build event page");
//
//     let body = if htmx.is_htmx {
//         event.render_html()
//     } else {
//         let index_route = IndexRoute::Event(event);
//         index_markup("Lineup", index_route, None).render_html()
//     };
//
//     Ok(HttpResponse::Ok().content_type("text/html").body(body))
// }

pub async fn event_lineup<AS: ArtistService>(
    event: &Event,
    artist_service: &AS,
) -> anyhow::Result<EventLineup> {
    let mut acts = Vec::new();
    for act in event.acts().as_ref() {
        dbg!(&act);
        let mut artists = Vec::new();
        for artist_id in act.artists() {
            let artist = artist_service
                .get_artist_by_id(artist_id)
                .await
                .context("Failed to fetch artist")?;
            artists.push(artist.name().clone());
        }
        acts.push(act_card(act, &artists)?);
    }

    EventLineupBuilder::default()
        .event_id(event.id().clone().into_inner())
        .acts(acts)
        .stages(
            event
                .stages()
                .as_ref()
                .iter()
                .map(|stage| {
                    (
                        stage.id().clone().into_inner(),
                        stage.name().clone().into_inner(),
                    )
                })
                .collect(),
        )
        .build()
        .context("Failed to build event lineup")
}
