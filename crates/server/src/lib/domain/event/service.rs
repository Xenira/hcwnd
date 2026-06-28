use crate::domain::{
    event::{
        models::{
            act::{Act, ActId, CreateActError, CreateActRequest},
            event::{
                CreateEventError, CreateEventRequest, Event, EventId, EventListItem, GetEventError,
                ListEventsError,
            },
        },
        ports::{ActRepository, DayRepository, EventRepository, EventService, StageRepository},
    },
    user::models::user::UserId,
};

#[derive(Debug, Clone)]
pub struct Service<ER, DR, SR, AR>
where
    ER: EventRepository,
    DR: DayRepository,
    SR: StageRepository,
    AR: ActRepository,
{
    pub event_repository: ER,
    pub day_repository: DR,
    pub slot_repository: SR,
    pub act_repository: AR,
}

impl<ER, DR, SR, AR> Service<ER, DR, SR, AR>
where
    ER: EventRepository,
    DR: DayRepository,
    SR: StageRepository,
    AR: ActRepository,
{
    pub fn new(
        event_repository: ER,
        day_repository: DR,
        slot_repository: SR,
        act_repository: AR,
    ) -> Self {
        Self {
            event_repository,
            day_repository,
            slot_repository,
            act_repository,
        }
    }
}

impl<ER, DR, SR, AR> EventService for Service<ER, DR, SR, AR>
where
    ER: EventRepository,
    DR: DayRepository,
    SR: StageRepository,
    AR: ActRepository,
{
    async fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> Result<EventId, CreateEventError> {
        self.event_repository.create_event(req, author_id).await
    }

    async fn list_events(&self) -> Result<Vec<EventListItem>, ListEventsError> {
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
}
