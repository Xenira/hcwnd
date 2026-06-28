use api::day::EventDay;
use chrono::{Duration, NaiveDateTime, NaiveTime};
use derive_builder::Builder;
use es_entity::{
    EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo, IntoEvents, TryFromEvents,
    es_query,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{
        self,
        event::models::{
            day::{CreateDayRequest, Day as DomainDay, DayId as DomainDayId, DayRangeError},
            event::CreateEventRequest,
            event::EventId as DomainEventId,
        },
        user::models::user::UserId as DomainUserId,
    },
    outbound::entity::{event::EventId, user::UserId},
};

es_entity::entity_id! { DayId }

#[derive(EsEvent, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "DayId")]
pub enum DayEvent {
    Initialized {
        id: DayId,
        day_number: u8,
        event_id: EventId,
        start_time: Option<NaiveTime>,
        end_time: Option<NaiveTime>,
        user_id: UserId,
    },
    DayApproved {
        reason: Option<String>,
        user_id: UserId,
    },
    DayAutoApproved,
    DayRejected {
        reason: Option<String>,
        user_id: UserId,
    },
    DayDeleted {
        reason: Option<String>,
        user_id: UserId,
    },
    DayVoted {
        power: i32,
        comment: Option<String>,
        user_id: UserId,
    },
    EditProposal {
        id: Uuid,
        start_time: Option<NaiveTime>,
        end_time: Option<NaiveTime>,
        user_id: UserId,
    },
    EditApproved {
        id: Uuid,
        user_id: UserId,
    },
    EditAutoApproved {
        id: Uuid,
    },
    EditRejected {
        id: Uuid,
        reason: Option<String>,
        user_id: UserId,
    },
    EditVoted {
        id: Uuid,
        power: i32,
        comment: Option<String>,
        user_id: UserId,
    },
}

#[derive(Debug, Serialize, Deserialize, Default, sqlx::Type)]
#[sqlx(type_name = "event_state", rename_all = "snake_case")]
enum DayState {
    #[default]
    Unapproved,
    Live,
    Rejected,
    Deleted,
}

#[derive(Debug)]
struct EditProposal {
    upvotes: u32,
    downvotes: u32,
    score: i32,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    user_id: UserId,
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct Day {
    pub id: DayId,
    pub event_id: EventId,
    pub day_number: u8,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    // Approval
    #[builder(default)]
    pub state: DayState,
    pub upvotes: u32,
    pub downvotes: u32,
    pub score: i32,

    // Edit
    pub edit_proposals: IndexMap<Uuid, EditProposal>,

    events: EntityEvents<DayEvent>,
}

impl Day {
    pub fn transform_date_from(base: NaiveDateTime, time: NaiveTime) -> NaiveDateTime {
        let naive = base.date().and_time(time);
        if naive < base {
            let technically_next_day = base.date() + Duration::days(1);

            technically_next_day.and_time(time)
        } else {
            naive
        }
    }

    // pub fn transform_time(&self, time: NaiveTime) -> NaiveDateTime {
    //     Self::transform_date_from(self.start_time, time)
    // }

    fn db_day_number(&self) -> i16 {
        self.day_number as i16
    }
}

impl From<Day> for DomainDay {
    fn from(day: Day) -> Self {
        DomainDay::new(
            DomainDayId::new(day.id.0),
            day.day_number,
            day.start_time,
            day.end_time,
        )
    }
}

// Any EsEntity must implement `TryFromEvents`.
// This trait is what hydrates entities after loading the events from the database
impl TryFromEvents<DayEvent> for Day {
    fn try_from_events(events: EntityEvents<DayEvent>) -> Result<Self, EntityHydrationError> {
        let mut builder = DayBuilder::default();
        let mut edit_proposals = IndexMap::new();
        let mut upvotes = 0;
        let mut downvotes = 0;
        let mut score = 0;

        for event in events.iter_all() {
            match event {
                DayEvent::Initialized {
                    id,
                    day_number,
                    event_id,
                    start_time,
                    end_time,
                    user_id: _,
                } => {
                    builder = builder
                        .id(*id)
                        .day_number(*day_number)
                        .event_id(*event_id)
                        .start_time(*start_time)
                        .end_time(*end_time);
                }
                DayEvent::DayVoted {
                    power,
                    comment: _,
                    user_id: _,
                } => {
                    score += *power;
                    if *power > 0 {
                        upvotes += 1;
                    } else {
                        downvotes += 1;
                    }
                }
                DayEvent::DayApproved { .. } | DayEvent::DayAutoApproved => {
                    builder = builder.state(DayState::Live);
                }
                DayEvent::DayRejected { reason, user_id } => {
                    builder = builder.state(DayState::Rejected);
                }
                DayEvent::DayDeleted { reason, user_id } => {
                    builder = builder.state(DayState::Deleted);
                }
                DayEvent::EditProposal {
                    id,
                    start_time,
                    end_time,
                    user_id,
                } => {
                    edit_proposals.insert(
                        *id,
                        EditProposal {
                            upvotes: 0,
                            downvotes: 0,
                            score: 0,
                            start_time: *start_time,
                            end_time: *end_time,
                            user_id: *user_id,
                        },
                    );
                }
                DayEvent::EditVoted {
                    id,
                    power,
                    comment,
                    user_id,
                } => {
                    if let Some(proposal) = edit_proposals.get_mut(id) {
                        proposal.score += *power;
                        if *power > 0 {
                            proposal.upvotes += 1;
                        } else {
                            proposal.downvotes += 1;
                        }
                    }
                }
                DayEvent::EditApproved { id, .. } | DayEvent::EditAutoApproved { id } => {
                    if let Some(proposal) = edit_proposals.shift_remove(id) {
                        builder = builder
                            .start_time(proposal.start_time)
                            .end_time(proposal.end_time);
                    }
                }
                DayEvent::EditRejected {
                    id,
                    reason,
                    user_id,
                } => {
                    edit_proposals.shift_remove(id);
                }
            }
        }
        builder
            .events(events)
            .score(score)
            .upvotes(upvotes)
            .downvotes(downvotes)
            .edit_proposals(edit_proposals)
            .build()
    }
}

#[derive(Debug)]
pub struct NewDay {
    id: DayId,
    event_id: EventId,
    day_number: u8,
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    user_id: UserId,
}

impl NewDay {
    pub fn from_domain(
        event_id: &DomainEventId,
        day: &CreateDayRequest,
        author_id: &DomainUserId,
    ) -> Self {
        let id = Uuid::new_v4().into();
        Self {
            id,
            event_id: event_id.clone().into_inner().into(),
            day_number: day.day_number(),
            start_time: day.start_time(),
            end_time: day.end_time(),
            user_id: author_id.clone().into_inner().into(),
        }
    }

    fn db_day_number(&self) -> i16 {
        self.day_number as i16
    }
}

impl IntoEvents<DayEvent> for NewDay {
    fn into_events(self) -> EntityEvents<DayEvent> {
        EntityEvents::init(
            self.id,
            [DayEvent::Initialized {
                id: self.id,
                day_number: self.day_number,
                event_id: self.event_id,
                start_time: self.start_time,
                end_time: self.end_time,
                user_id: self.user_id,
            }],
        )
    }
}

#[derive(EsRepo, Debug, Clone)]
#[es_repo(
    entity = "Day",
    // Configure the columns that need populating in the index table
    columns(
        event_id(ty = "EventId", list_by),
        day_number(ty = "i16", create(accessor = "db_day_number()"), update(accessor = "db_day_number()")),
        start_time(ty = "Option<NaiveTime>"),
        end_time(ty = "Option<NaiveTime>"),
    )
)]
pub struct DaysRepo {
    // Mandatory field so that the Repository can begin transactions
    pub pool: sqlx::PgPool,
}
impl DaysRepo {
    pub async fn get_day(&self, event_id: EventId, n: u16) -> Result<Option<Day>, DayQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM days WHERE event_id = $1 ORDER BY start_time OFFSET $2 LIMIT 1",
            event_id,
            n as i32
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_day_by_id(
        &self,
        event_id: EventId,
        day_id: DayId,
    ) -> Result<Option<Day>, DayQueryError> {
        let event_id: Uuid = event_id.into();
        let day_id: Uuid = day_id.into();
        es_query!(
            "SELECT * FROM days WHERE event_id = $1 and id = $2",
            event_id,
            day_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn days_for_event(&self, event_id: EventId) -> Result<Vec<Day>, DayQueryError> {
        let event_id: Uuid = event_id.into();
        Ok(es_query!(
            "SELECT * FROM days WHERE event_id = $1 ORDER BY start_time ASC",
            Some(event_id)
        )
        .fetch_n(&self.pool, usize::MAX)
        .await?
        .0)
    }

    pub async fn has_multiple_days(&self, event_id: EventId) -> Result<bool, sqlx::Error> {
        let event_id: Uuid = event_id.into();
        Ok(sqlx::query!(
            "SELECT COUNT(*) > 1 as has_multiple FROM days WHERE event_id = $1",
            event_id
        )
        .fetch_one(&self.pool)
        .await?
        .has_multiple
        .unwrap_or(false))
    }

    pub async fn get_first_day(&self, event_id: &EventId) -> Result<Option<Day>, DayQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM days WHERE event_id = $1 ORDER BY day_number ASC LIMIT 1",
            Some(event_id)
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_last_day(&self, event_id: &EventId) -> Result<Option<Day>, DayQueryError> {
        let event_id: Uuid = event_id.into();
        es_query!(
            "SELECT * FROM days WHERE event_id = $1 ORDER BY day_number DESC LIMIT 1",
            Some(event_id)
        )
        .fetch_optional(&self.pool)
        .await
    }
}
