use async_trait::async_trait;

use crate::domain::{
    event::models::{
        act::{Act, ActId, CreateActError, CreateActRequest, GetActError, ListActsError},
        day::{CreateDayError, CreateDayRequest, Day, DayId, GetDayError, ListDaysError},
        event::{
            CreateEventError, CreateEventRequest, Event, EventId, GetEventError, ListEventsError,
        },
        stage::{
            CreateStageError, CreateStageRequest, GetStageError, ListStagesError, Stage, StageId,
        },
    },
    user::models::user::UserId,
};

#[async_trait]
pub trait EventService: Sync + Send + 'static {
    async fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> Result<EventId, CreateEventError>;

    async fn list_events(&self) -> Result<Vec<Event>, ListEventsError>;

    async fn get_event_by_id(&self, id: &EventId) -> Result<Event, GetEventError>;

    async fn create_act(
        &self,
        event_id: &EventId,
        req: &CreateActRequest,
        author_id: &UserId,
    ) -> Result<Act, CreateActError>;
}

#[async_trait]
pub trait EventRepository: Clone + Sync + Send + 'static {
    async fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> Result<EventId, CreateEventError>;

    async fn list_events(&self) -> Result<Vec<Event>, ListEventsError>;

    async fn get_event_by_id(&self, id: &EventId) -> Result<Event, GetEventError>;
}

#[async_trait]
pub trait ActRepository: Clone + Sync + Send + 'static {
    async fn create_act(
        &self,
        event_id: &EventId,
        act: &CreateActRequest,
        author_id: &UserId,
    ) -> Result<Act, CreateActError>;

    async fn list_acts(&self, event_id: &EventId) -> Result<Vec<Act>, ListActsError>;

    async fn get_act_by_id(&self, act_id: &ActId) -> Result<Act, GetActError>;
}

#[async_trait]
pub trait StageRepository: Clone + Sync + Send + 'static {
    async fn create_stage(
        &self,
        stage: &CreateStageRequest,
        author_id: &UserId,
    ) -> Result<Stage, CreateStageError>;

    async fn list_stages(&self, event_id: &EventId) -> Result<Vec<Stage>, ListStagesError>;

    async fn get_stage_by_id(&self, stage_id: &StageId) -> Result<Stage, GetStageError>;
}

#[async_trait]
pub trait DayRepository: Clone + Sync + Send + 'static {
    async fn create_day(
        &self,
        event_id: &EventId,
        day: &CreateDayRequest,
        author_id: &UserId,
    ) -> Result<Day, CreateDayError>;

    async fn list_days(&self, event_id: &EventId) -> Result<Vec<Day>, ListDaysError>;

    async fn get_day_by_id(&self, day_id: &DayId) -> Result<Day, GetDayError>;

    async fn get_first_day(&self, event_id: &EventId) -> Result<Day, GetDayError>;

    async fn get_last_day(&self, event_id: &EventId) -> Result<Day, GetDayError>;
}
