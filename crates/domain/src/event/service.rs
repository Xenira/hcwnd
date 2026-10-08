use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use eventwise::{repository::Repository, store::EventStore};
use futures::future::try_join_all;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use url::Url;
use uuid::Uuid;

use crate::{
    event::{
        commands::SuggestFestival,
        models::{
            act::{Act, CreateActError, CreateActRequest},
            festival::{
                CreateEventError, CreateEventRequest, Festival, FestivalId, GetEventError,
                ListEventsError,
            },
        },
        ports::{
            ActRepository, DayRepository, EventRepository, EventService, FestivalQuery,
            ImageRepository, StageRepository,
        },
    },
    user::models::user::UserId,
};

#[derive(Debug, Clone)]
pub struct Service<S, IR, FQ>
where
    S: EventStore,
    FQ: FestivalQuery,
    IR: ImageRepository,
{
    festival_repository: Arc<Repository<Festival, S>>,
    image_repository: IR,
    festival_query: FQ,
}

impl<S, IR, FQ> Service<S, IR, FQ>
where
    S: EventStore,
    IR: ImageRepository,
    FQ: FestivalQuery,
{
    pub fn new(
        festival_repository: Repository<Festival, S>,
        image_repository: IR,
        festival_query: FQ,
    ) -> Self {
        Self {
            festival_repository: Arc::new(festival_repository),
            image_repository,
            festival_query,
        }
    }
}

#[async_trait]
impl<S, IR, FQ> EventService for Service<S, IR, FQ>
where
    S: EventStore + 'static,
    IR: ImageRepository + 'static,
    FQ: FestivalQuery,
{
    async fn create_event(
        &self,
        req: SuggestFestival,
        author_id: &UserId,
    ) -> Result<FestivalId, CreateEventError> {
        let id = FestivalId::new(Uuid::now_v7());

        self.festival_repository
            .as_ref()
            .execute(&id, req)
            .await
            .context("Failed to execute festival creation command")?;

        Ok(id)
    }

    async fn list_events(&self) -> Result<Vec<Festival>, ListEventsError> {
        let upcoming_festival_ids = self.festival_query.upcoming_festivals().await?;
        let upcoming = upcoming_festival_ids
            .iter()
            .map(|id| self.get_event_by_id(id))
            .collect_vec();

        try_join_all(upcoming)
            .await
            .map_err(|e| ListEventsError::Unknown(e.into()))
    }

    async fn get_event_by_id(&self, id: &FestivalId) -> Result<Festival, GetEventError> {
        self.festival_repository
            .as_ref()
            .load(id)
            .await
            .context("Failed to load festival by ID")?
            .0
            .ok_or(GetEventError::EventNotFound)
    }

    async fn upload_form_image(
        &self,
        image_bytes: &[u8],
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        self.image_repository.upload_form_image(image_bytes).await
    }
}

// #[async_trait]
// impl<ER, DR, SR, AR, IR> EventService for Service<ER, DR, SR, AR, IR>
// where
//     ER: EventRepository,
//     DR: DayRepository,
//     SR: StageRepository,
//     AR: ActRepository,
//     IR: ImageRepository,
// {
//     async fn create_event(
//         &self,
//         req: &CreateEventRequest,
//         author_id: &UserId,
//     ) -> Result<FestivalId, CreateEventError> {
//         self.event_repository.create_event(req, author_id).await
//     }
//
//     async fn list_events(&self) -> Result<Vec<Festival>, ListEventsError> {
//         self.event_repository.list_events().await
//     }
//
//     async fn get_event_by_id(&self, id: &FestivalId) -> Result<Festival, GetEventError> {
//         self.event_repository.get_event_by_id(id).await
//     }
//
//     async fn create_act(
//         &self,
//         event_id: &FestivalId,
//         req: &CreateActRequest,
//         author_id: &UserId,
//     ) -> Result<Act, CreateActError> {
//         self.act_repository
//             .create_act(event_id, req, author_id)
//             .await
//     }
// }
