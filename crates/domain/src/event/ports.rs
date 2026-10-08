use async_trait::async_trait;
use url::Url;

use crate::{
    event::{
        commands::SuggestFestival,
        models::{
            act::{Act, ActId, CreateActError, CreateActRequest, GetActError, ListActsError},
            day::{CreateDayError, CreateDayRequest, Day, DayId, GetDayError, ListDaysError},
            festival::{
                CreateEventError, CreateEventRequest, Festival, FestivalId, GetEventError,
                ListEventsError,
            },
            stage::{
                CreateStageError, CreateStageRequest, GetStageError, ListStagesError, Stage,
                StageId,
            },
        },
    },
    user::models::user::UserId,
};

#[async_trait]
pub trait EventService: Sync + Send + 'static {
    async fn create_event(
        &self,
        req: SuggestFestival,
        author_id: &UserId,
    ) -> Result<FestivalId, CreateEventError>;

    async fn list_events(&self) -> Result<Vec<Festival>, ListEventsError>;

    async fn get_event_by_id(&self, id: &FestivalId) -> Result<Festival, GetEventError>;

    // async fn create_act(
    //     &self,
    //     event_id: &FestivalId,
    //     req: &CreateActRequest,
    //     author_id: &UserId,
    // ) -> Result<Act, CreateActError>;

    async fn upload_form_image(
        &self,
        image_bytes: &[u8],
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait FestivalQuery: Clone + Sync + Send + 'static {
    async fn upcoming_festivals(&self) -> Result<Vec<FestivalId>, ListEventsError>;
}

#[async_trait]
pub trait EventRepository: Clone + Sync + Send + 'static {
    async fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> Result<FestivalId, CreateEventError>;

    async fn list_events(&self) -> Result<Vec<Festival>, ListEventsError>;

    async fn get_event_by_id(&self, id: &FestivalId) -> Result<Festival, GetEventError>;
}

#[async_trait]
pub trait ActRepository: Clone + Sync + Send + 'static {
    async fn create_act(
        &self,
        event_id: &FestivalId,
        act: &CreateActRequest,
        author_id: &UserId,
    ) -> Result<Act, CreateActError>;

    async fn list_acts(&self, event_id: &FestivalId) -> Result<Vec<Act>, ListActsError>;

    async fn get_act_by_id(&self, act_id: &ActId) -> Result<Act, GetActError>;
}

#[async_trait]
pub trait StageRepository: Clone + Sync + Send + 'static {
    async fn create_stage(
        &self,
        stage: &CreateStageRequest,
        author_id: &UserId,
    ) -> Result<Stage, CreateStageError>;

    async fn list_stages(&self, event_id: &FestivalId) -> Result<Vec<Stage>, ListStagesError>;

    async fn get_stage_by_id(&self, stage_id: &StageId) -> Result<Stage, GetStageError>;
}

#[async_trait]
pub trait DayRepository: Clone + Sync + Send + 'static {
    async fn create_day(
        &self,
        event_id: &FestivalId,
        day: &CreateDayRequest,
        author_id: &UserId,
    ) -> Result<Day, CreateDayError>;

    async fn list_days(&self, event_id: &FestivalId) -> Result<Vec<Day>, ListDaysError>;

    async fn get_day_by_id(&self, day_id: &DayId) -> Result<Day, GetDayError>;

    async fn get_first_day(&self, event_id: &FestivalId) -> Result<Day, GetDayError>;

    async fn get_last_day(&self, event_id: &FestivalId) -> Result<Day, GetDayError>;
}

#[async_trait]
pub trait ImageRepository: Clone + Sync + Send + 'static {
    async fn upload_form_image(
        &self,
        image_bytes: &[u8],
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>>;

    async fn persist_form_image(
        &self,
        event_id: &FestivalId,
        image_url: &Url,
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>>;
}
