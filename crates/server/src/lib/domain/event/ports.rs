use crate::domain::{
    event::models::{
        act::{Act, ActId, CreateActError, CreateActRequest, GetActError, ListActsError},
        day::{CreateDayError, CreateDayRequest, Day, DayId, GetDayError, ListDaysError},
        event::{
            CreateEventError, CreateEventRequest, Event, EventId, EventListItem, GetEventError,
            ListEventsError,
        },
        stage::{
            CreateStageError, CreateStageRequest, GetStageError, ListStagesError, Stage, StageId,
        },
    },
    user::models::user::UserId,
};

pub trait EventService: Clone + Sync + Send + 'static {
    fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<EventId, CreateEventError>>;

    fn list_events(&self) -> impl Future<Output = Result<Vec<EventListItem>, ListEventsError>>;

    fn get_event_by_id(&self, id: &EventId) -> impl Future<Output = Result<Event, GetEventError>>;

    fn create_act(
        &self,
        event_id: &EventId,
        req: &CreateActRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<Act, CreateActError>>;
}

pub trait EventRepository: Clone + Sync + Send + 'static {
    fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<EventId, CreateEventError>>;

    fn list_events(&self) -> impl Future<Output = Result<Vec<EventListItem>, ListEventsError>>;

    fn get_event_by_id(&self, id: &EventId) -> impl Future<Output = Result<Event, GetEventError>>;
}

pub trait ActRepository: Clone + Sync + Send + 'static {
    fn create_act(
        &self,
        event_id: &EventId,
        act: &CreateActRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<Act, CreateActError>>;

    fn list_acts(
        &self,
        event_id: &EventId,
    ) -> impl Future<Output = Result<Vec<Act>, ListActsError>>;

    fn get_act_by_id(&self, act_id: &ActId) -> impl Future<Output = Result<Act, GetActError>>;
}

pub trait StageRepository: Clone + Sync + Send + 'static {
    fn create_stage(
        &self,
        stage: &CreateStageRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<Stage, CreateStageError>>;

    fn list_stages(
        &self,
        event_id: &EventId,
    ) -> impl Future<Output = Result<Vec<Stage>, ListStagesError>>;

    fn get_stage_by_id(
        &self,
        stage_id: &StageId,
    ) -> impl Future<Output = Result<Stage, GetStageError>>;
}

pub trait DayRepository: Clone + Sync + Send + 'static {
    fn create_day(
        &self,
        event_id: &EventId,
        day: &CreateDayRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<Day, CreateDayError>>;

    fn list_days(
        &self,
        event_id: &EventId,
    ) -> impl Future<Output = Result<Vec<Day>, ListDaysError>>;

    fn get_day_by_id(&self, day_id: &DayId) -> impl Future<Output = Result<Day, GetDayError>>;

    fn get_first_day(&self, event_id: &EventId) -> impl Future<Output = Result<Day, GetDayError>>;

    fn get_last_day(&self, event_id: &EventId) -> impl Future<Output = Result<Day, GetDayError>>;
}
