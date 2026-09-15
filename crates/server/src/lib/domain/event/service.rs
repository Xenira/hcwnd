use async_trait::async_trait;
use url::Url;

use crate::domain::{
    event::{
        models::{
            act::{Act, CreateActError, CreateActRequest},
            event::{
                CreateEventError, CreateEventRequest, Event, EventId, GetEventError,
                ListEventsError,
            },
        },
        ports::{
            ActRepository, DayRepository, EventRepository, EventService, ImageRepository,
            StageRepository,
        },
    },
    user::models::user::UserId,
};

#[derive(Debug, Clone)]
pub struct Service<ER, DR, SR, AR, IR>
where
    ER: EventRepository,
    DR: DayRepository,
    SR: StageRepository,
    AR: ActRepository,
    IR: ImageRepository,
{
    pub event_repository: ER,
    pub day_repository: DR,
    pub slot_repository: SR,
    pub act_repository: AR,
    pub image_repository: IR,
}

impl<ER, DR, SR, AR, IR> Service<ER, DR, SR, AR, IR>
where
    ER: EventRepository,
    DR: DayRepository,
    SR: StageRepository,
    AR: ActRepository,
    IR: ImageRepository,
{
    pub fn new(
        event_repository: ER,
        day_repository: DR,
        slot_repository: SR,
        act_repository: AR,
        image_repository: IR,
    ) -> Self {
        Self {
            event_repository,
            day_repository,
            slot_repository,
            act_repository,
            image_repository,
        }
    }
}

#[async_trait]
impl<ER, DR, SR, AR, IR> EventService for Service<ER, DR, SR, AR, IR>
where
    ER: EventRepository,
    DR: DayRepository,
    SR: StageRepository,
    AR: ActRepository,
    IR: ImageRepository,
{
    async fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> Result<EventId, CreateEventError> {
        self.event_repository.create_event(req, author_id).await
    }

    async fn list_events(&self) -> Result<Vec<Event>, ListEventsError> {
        self.event_repository.list_events().await
    }

    async fn get_event_by_id(&self, id: &EventId) -> Result<Event, GetEventError> {
        self.event_repository.get_event_by_id(id).await
    }

    async fn create_act(
        &self,
        event_id: &EventId,
        req: &CreateActRequest,
        author_id: &UserId,
    ) -> Result<Act, CreateActError> {
        self.act_repository
            .create_act(event_id, req, author_id)
            .await
    }

    async fn upload_form_image(
        &self,
        image_bytes: &[u8],
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        self.image_repository.upload_form_image(image_bytes).await
    }
}
