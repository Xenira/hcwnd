use actix_htmx::Htmx;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use api::event::new::EventCreateScheduleStep;
use domain::user::models::user::User;
use log::debug;
use serde_qs::web::QsForm;

use crate::inbound::http::user::{RequestUser, UiStateExtractor};

pub fn configure(cfg: &mut ServiceConfig) {
    // cfg.service(redirect_to_details_step)
    //     .service(schedule_step_form);
    // .route(
    //     ui::event::create::days_step::ADD_DAY_ROUTE,
    //     web::post().to(add_event_day),
    // )
    // .route(
    //     ui::event::create::days_step::REMOVE_DAY_ROUTE,
    //     web::post().to(remove_event_day),
    // );
}

// User should not be able to access this step directly, so we redirect them to the first step of the flow
// #[get("")]
// async fn redirect_to_details_step() -> impl Responder {
//     HttpResponse::Found()
//         .append_header((
//             "Location",
//             format!("{}{}", create::BASE_ROUTE, details::BASE_ROUTE),
//         ))
//         .finish()
// }
//
// #[post("")]
// async fn schedule_step_form(
//     state: UiStateExtractor,
//     _: RequestUser,
//     form: QsForm<EventCreateScheduleStep>,
//     htmx: Htmx,
// ) -> impl Responder {
//     let body = if htmx.is_htmx {
//         ui::event::create::days_step::render(&state, &form.into_inner())
//     } else {
//         ui::event::create::days_step::full_page(&state, &form.into_inner())
//     };
//
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(body.into_string())
// }

// async fn add_event_day(
//     state: UiStateExtractor,
//     _: RequestUser,
//     form: QsForm<EventCreateScheduleStep>,
//     htmx: Htmx,
// ) -> impl Responder {
//     debug!("Adding event day, current days: {:?}", form.days);
//     let day = EventDay {
//         day: form.days.len(),
//         start_time: None,
//         end_time: None,
//     };
//
//     let body = if htmx.is_htmx {
//         ui::event::create::days_step::render_day(&state.locale, &day, Some(form.days.len() + 1))
//     } else {
//         let mut form = form.into_inner();
//         form.days.push(day);
//         ui::event::create::days_step::full_page(&state, &form)
//     };
//
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(body.into_string())
// }

// async fn remove_event_day(
//     _: RequestUser,
//     state: UiStateExtractor,
//     form: QsForm<EventCreateDaysStep>,
//     htmx: Htmx,
// ) -> impl Responder {
//     let body = if htmx.is_htmx {
//         day_buttons(&state.locale, form.days.len() - 1, true)
//     } else {
//         let mut form = form.into_inner();
//         form.days.pop();
//         ui::event::create::days_step::full_page(&state, &form)
//     };
//
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(body.into_string())
// }
