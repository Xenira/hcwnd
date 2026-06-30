use actix_htmx::Htmx;
use actix_web::{HttpResponse, Responder, post, web};
use anyhow::Context as _;
use serde_qs::web::QsForm;
use ui::event::create::{confirm_step::EventCreateConfirmStep, days_step::EventDay};

use crate::{
    domain::{
        event::{
            models::{
                day::CreateDayRequest,
                event::{
                    CreateEventRequest, EventDaysCreateRequests, EventDescription, EventName,
                    ImageUrl, WebsiteUrl,
                },
            },
            ports::EventService,
        },
        proposal::ProposalSource,
        user::models::user::User,
    },
    inbound::http::AppState,
};

pub mod confirm_step;
pub mod days_step;
pub mod details_step;
pub mod name_step;
pub mod stages_step;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ui::event::create::name_step::BASE_ROUTE).configure(name_step::configure),
    )
    .service(web::scope(ui::event::create::days_step::BASE_ROUTE).configure(days_step::configure))
    .service(
        web::scope(ui::event::create::details_step::BASE_ROUTE).configure(details_step::configure),
    )
    .service(
        web::scope(ui::event::create::stage_step::BASE_ROUTE).configure(stages_step::configure),
    )
    .service(
        web::scope(ui::event::create::confirm_step::BASE_ROUTE).configure(confirm_step::configure),
    )
    .service(create_event);
}

impl TryFrom<EventCreateConfirmStep> for CreateEventRequest {
    type Error = anyhow::Error;

    fn try_from(value: EventCreateConfirmStep) -> Result<Self, Self::Error> {
        let source = value.source.context("Source is required")?;
        let source = ProposalSource::try_new(source)?;
        let source_url = value.source_url.map(|url| url.try_into()).transpose()?;

        let name = EventName::try_new(value.name)?;
        let description = EventDescription::try_new(value.description)?;
        let website_url = WebsiteUrl::try_new(value.website)?;
        let image_url = ImageUrl::try_new(value.image_url)?;
        let start_date = value.start_date;

        let days = value
            .days
            .into_iter()
            .map(CreateDayRequest::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let days = EventDaysCreateRequests::try_new(days)?;

        Ok(CreateEventRequest::new(
            name,
            description,
            start_date,
            days,
            website_url,
            image_url,
            source,
            source_url,
        ))
    }
}

impl TryFrom<EventDay> for CreateDayRequest {
    type Error = anyhow::Error;

    fn try_from(value: EventDay) -> Result<Self, Self::Error> {
        // Implement the conversion logic here
        Ok(CreateDayRequest::new(
            value.day.try_into()?,
            value.start_time,
            value.end_time,
        ))
    }
}

#[post("")]
async fn create_event(
    app_state: web::Data<AppState>,
    user: User,
    form: QsForm<EventCreateConfirmStep>,
    htmx: Htmx,
) -> impl Responder {
    let req = form
        .into_inner()
        .try_into()
        .expect("Failed to convert form into CreateEventRequest");
    let event = app_state
        .event_service
        .create_event(&req, user.id())
        .await
        .expect("Failed to create event");

    let event_url = format!("{}/{event}", ui::event::BASE_ROUTE);
    if htmx.is_htmx {
        htmx.redirect_with_swap(event_url.clone());
        htmx.push_url(event_url);
        HttpResponse::Created().finish()
    } else {
        HttpResponse::Created()
            .append_header(("Location", event_url))
            .finish()
    }
}
