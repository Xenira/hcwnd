use std::{ops::Deref, str::FromStr as _};

use anyhow::Context;
use api::event::EventListEntry;
use chrono::Duration;
use es_entity::DbOp;
use itertools::Itertools;
use itertools::Itertools as _;
use log::info;
use sqlx::{postgres::PgConnectOptions, PgPool};
use uuid::Uuid;

use crate::{
    domain::{
        artist::{
            models::artist::{
                Artist, ArtistId, CreateArtistError, CreateArtistRequest, GetArtistError,
                SearchArtistsError, SearchArtistsQuery,
            },
            ports::ArtistRepository,
        },
        event::{
            models::{
                act::{Act, CreateActError, CreateActRequest, GetActError, ListActsError},
                day::{CreateDayError, CreateDayRequest, Day, DayId, GetDayError, ListDaysError},
                event::{
                    CreateEventError, CreateEventRequest, Event, EventActs, EventDays,
                    EventDescription, EventId, EventListItem, EventName, EventStages,
                    GetEventError, ImageUrl, ListEventsError,
                },
                stage::{
                    CreateStageError, CreateStageRequest, GetStageError, ListStagesError, Stage,
                },
            },
            ports::{ActRepository, DayRepository, EventRepository, StageRepository},
        },
        user::{
            models::user::{
                CreateUserError, CreateUserRequest, FindUserError, User, UserEmail, UserId,
                UserName,
            },
            ports::UserRepository,
        },
    },
    outbound::entity::{
        self,
        act::NewAct,
        artist::NewArtist,
        day::NewDay,
        event::NewEvent,
        stage::{NewStage, NewStageBuilder},
        user::{NewUser, UserFindError},
    },
};

#[derive(Clone, Debug)]
pub struct Pg {
    pool: PgPool,
    event_repo: entity::event::EventRepo,
    stage_repo: entity::stage::StageRepo,
    act_repo: entity::act::ActRepo,
    day_repo: entity::day::DaysRepo,
    artist_repo: entity::artist::ArtistRepo,
    user_repo: entity::user::UserRepo,
}

impl Pg {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect_with(
            PgConnectOptions::from_str(url)
                .with_context(|| format!("Invalid PostgreSQL connection URL: {}", url))?,
        )
        .await
        .with_context(|| format!("Failed to connect to PostgreSQL database at {}", url))?;

        let event_repo = entity::event::EventRepo { pool: pool.clone() };
        let stage_repo = entity::stage::StageRepo { pool: pool.clone() };
        let act_repo = entity::act::ActRepo { pool: pool.clone() };
        let day_repo = entity::day::DaysRepo { pool: pool.clone() };
        let artist_repo = entity::artist::ArtistRepo { pool: pool.clone() };
        let user_repo = entity::user::UserRepo { pool: pool.clone() };

        Ok(Self {
            pool,
            event_repo,
            stage_repo,
            act_repo,
            day_repo,
            artist_repo,
            user_repo,
        })
    }

    async fn start_transaction(&self) -> anyhow::Result<DbOp> {
        DbOp::init(&self.pool)
            .await
            .context("Failed to start transaction")
    }
}

impl EventRepository for Pg {
    async fn create_event(
        &self,
        req: &CreateEventRequest,
        author_id: &UserId,
    ) -> Result<EventId, CreateEventError> {
        let mut op = self.start_transaction().await?;

        let new_event = NewEvent::from_domain(req, author_id);
        let event_id = EventId::new(
            self.event_repo
                .create_in_op(&mut op, new_event)
                .await
                // TODO: Improve error context
                .context("Failed to create event")?
                .id
                .into(),
        );

        let new_stage = NewStageBuilder::default()
            .event_id(event_id.clone().into_inner())
            .name("Main Stage")
            .user_id(author_id.clone().into_inner())
            .build()
            .context("Failed to build main stage for event")?;
        self.stage_repo
            .create_in_op(&mut op, new_stage)
            .await
            .context("Failed to create main stage for event")?;

        for day_req in req.days().as_ref() {
            let new_day = NewDay::from_domain(&event_id, day_req, author_id);
            self.day_repo
                .create_in_op(&mut op, new_day)
                .await
                .with_context(|| format!("Failed to create day {day_req:?} for event"))?;
        }

        op.commit()
            .await
            .context("Failed to commit transaction for creating event")?;

        Ok(event_id)
    }

    async fn list_events(&self) -> Result<Vec<EventListItem>, ListEventsError> {
        let events = self
            .event_repo
            .future_events()
            .await
            .context("Failed to list future events from db")?;

        let mut result = Vec::with_capacity(events.len());
        for event in &events {
            let start_date = event.start_date;
            let event_id = EventId::new(event.id.into());
            let first_day = self
                .get_first_day(&event_id)
                .await
                .context("Failed to get first day for event")?;
            let last_day = self
                .get_last_day(&event_id)
                .await
                .context("Failed to get last day for event")?;

            result.push(EventListItem::new(
                EventId::new(event.id.into()),
                EventName::try_new(event.name.clone()).context("Invalid event name")?,
                EventDescription::try_new(event.description.clone())
                    .context("Invalid event description")?,
                ImageUrl::try_new(
                    url::Url::parse(&event.image_url).context("Invalid event image URL")?,
                )
                .context("Invalid event image URL")?,
                start_date,
                first_day.start_time(),
                start_date + Duration::days(last_day.day_number() as i64),
                last_day.end_time(),
            ));
        }

        Ok(result)
    }

    async fn get_event_by_id(&self, event_id: &EventId) -> Result<Event, GetEventError> {
        let days = self.list_days(event_id).await?;
        let days = EventDays::try_new(days).context("Failed to parse days for event")?;

        let stages = self.list_stages(event_id).await?;
        let stages = EventStages::try_new(stages).context("Failed to parse stages for event")?;

        let acts = self.list_acts(event_id).await?;
        let acts = EventActs::new(acts);

        let event: entity::event::Event = self
            .event_repo
            .find_by_id(&event_id.as_ref().clone().into())
            .await
            // TODO: Convert to not found error if the event doesn't exist
            .context("Failed to fetch events from db")?
            .try_into()
            .context("Failed to parse event data")?;

        let event_id = EventId::new(event.id.into());
        let event_name = EventName::try_new(event.name).context("Invalid event name")?;
        let event_description =
            EventDescription::try_new(event.description).context("Invalid event description")?;

        Ok(Event::new(
            event_id,
            event_name,
            event_description,
            stages,
            acts,
            event.start_date,
            days,
        ))
    }
}

impl DayRepository for Pg {
    async fn create_day(
        &self,
        event_id: &EventId,
        req: &CreateDayRequest,
        author_id: &UserId,
    ) -> Result<Day, CreateDayError> {
        let new_day = NewDay::from_domain(event_id, req, author_id);
        let created_day = self
            .day_repo
            .create(new_day)
            .await
            .context("Failed to create day")?;

        Ok(created_day
            .try_into()
            .context("Failed to parse created day")?)
    }

    async fn list_days(&self, event_id: &EventId) -> Result<Vec<Day>, ListDaysError> {
        let days = self
            .day_repo
            .days_for_event(event_id.as_ref().clone().into())
            .await
            .context("Failed to list days for event")?;

        Ok(days
            .into_iter()
            .map(TryInto::try_into)
            .try_collect()
            .context("Failed to fetch events from db")?)
    }

    async fn get_day_by_id(&self, day_id: &DayId) -> Result<Day, GetDayError> {
        let day = self
            .day_repo
            .find_by_id(&day_id.as_ref().clone().into())
            .await
            .context("Failed to get day by ID")?;

        Ok(day.try_into().context("Failed to parse day")?)
    }

    async fn get_first_day(&self, event_id: &EventId) -> Result<Day, GetDayError> {
        let day = self
            .day_repo
            .get_first_day(&event_id.as_ref().clone().into())
            .await
            .transpose()
            .ok_or(GetDayError::DayNotFound)?
            .context("Failed to get first day for event")?;

        Ok(day.try_into().context("Failed to parse day")?)
    }

    async fn get_last_day(&self, event_id: &EventId) -> Result<Day, GetDayError> {
        let day = self
            .day_repo
            .get_last_day(&event_id.as_ref().clone().into())
            .await
            .transpose()
            .ok_or(GetDayError::DayNotFound)?
            .context("Failed to get last day for event")?;

        Ok(day.try_into().context("Failed to parse day")?)
    }
}

impl StageRepository for Pg {
    async fn create_stage(
        &self,
        stage: &CreateStageRequest,
        author_id: &UserId,
    ) -> Result<Stage, CreateStageError> {
        // let new_stage = NewStage::f
        unimplemented!()
    }

    async fn list_stages(&self, event_id: &EventId) -> Result<Vec<Stage>, ListStagesError> {
        let stages = self
            .stage_repo
            .stages_for_event((*event_id.as_ref()).into())
            .await
            .context("Failed to list stages for event")?;

        info!("Fetched stages from db: {:#?}", stages.len());

        Ok(stages
            .into_iter()
            .map(TryInto::try_into)
            .try_collect()
            .context("Failed to parse stages for event")?)
    }

    async fn get_stage_by_id(
        &self,
        _stage_id: &crate::domain::event::models::stage::StageId,
    ) -> Result<Stage, GetStageError> {
        unimplemented!()
    }
}

impl ActRepository for Pg {
    async fn create_act(
        &self,
        event_id: &EventId,
        act: &CreateActRequest,
        author_id: &UserId,
    ) -> Result<Act, CreateActError> {
        let new_act = NewAct::from_domain(event_id, act, author_id);

        Ok(self
            .act_repo
            .create(new_act)
            .await
            .context("Failed to create act")?
            .try_into()?)
    }

    async fn list_acts(&self, event_id: &EventId) -> Result<Vec<Act>, ListActsError> {
        let acts = self
            .act_repo
            .acts_for_event((*event_id.as_ref()).into())
            .await
            .context("Failed to list acts for event")?;

        Ok(acts
            .into_iter()
            .map(TryInto::try_into)
            .try_collect()
            .context("Failed to parse acts for event")?)
    }

    async fn get_act_by_id(
        &self,
        _act_id: &crate::domain::event::models::act::ActId,
    ) -> Result<Act, GetActError> {
        unimplemented!()
    }
}

impl ArtistRepository for Pg {
    async fn create_artist(
        &self,
        req: &CreateArtistRequest,
        author_id: &UserId,
    ) -> Result<Artist, CreateArtistError> {
        let new_artist = NewArtist::from_domain(req, author_id);

        Ok(self
            .artist_repo
            .create(new_artist)
            .await
            .context("Failed to create artist")?
            .try_into()?)
    }

    async fn search_artist(
        &self,
        query: &SearchArtistsQuery,
    ) -> Result<Vec<Artist>, SearchArtistsError> {
        Ok(self
            .artist_repo
            .search(query)
            .await
            .context("Failed to search artists in db")?
            .into_iter()
            .map(TryInto::try_into)
            .try_collect()
            .context("Failed to parse artists from db")?)
    }

    async fn get_artist_by_id(&self, id: &ArtistId) -> Result<Artist, GetArtistError> {
        Ok(self
            .artist_repo
            .find_by_id(&(*id.as_ref()).into())
            .await
            .context("Failed to get artist by ID from db")?
            .try_into()
            .context("Failed to parse artist from db")?)
    }
}

impl UserRepository for Pg {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, CreateUserError> {
        let new_user =
            NewUser::from_domain(req).context("Failed to build new user from request")?;
        Ok(self
            .user_repo
            .create(new_user)
            .await
            .context("Failed to create user in db")?
            .try_into()
            .context("Failed to parse created user from db")?)
    }

    async fn find_user_by_username(
        &self,
        username: &UserName,
    ) -> Result<Option<User>, FindUserError> {
        todo!()
    }

    async fn find_user_by_id(&self, id: &UserId) -> Result<Option<User>, FindUserError> {
        let user = self.user_repo.find_by_id(&(*id.as_ref()).into()).await;

        let Ok(user) = user else {
            return match user.err().unwrap() {
                UserFindError::NotFound { .. } => Ok(None),
                _ => Err(anyhow::anyhow!("Failed to fetch user from db").into()),
            };
        };

        Ok(Some(
            user.try_into().context("Failed to parse user from db")?,
        ))
    }

    async fn find_user_by_email(&self, email: &UserEmail) -> Result<Option<User>, FindUserError> {
        let user = self
            .user_repo
            .find_by_email(&(*email.as_ref()).to_string())
            .await;

        let Ok(user) = user else {
            return match user.err().unwrap() {
                UserFindError::NotFound { .. } => Ok(None),
                _ => Err(anyhow::anyhow!("Failed to fetch user from db").into()),
            };
        };

        Ok(Some(
            user.try_into().context("Failed to parse user from db")?,
        ))
    }
}
