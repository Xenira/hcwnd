use std::collections::HashMap;

use chrono::NaiveDate;
use derive_builder::Builder;
use es_entity::{
    EntityEvents, EntityHydrationError, EsEntity, EsEvent, EsRepo, IntoEvents, TryFromEvents,
    es_query,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use ui::event::details::EventDetails;

use crate::{
    domain::{
        event::models::event::CreateEventRequest, user::models::user::UserId as DomainUserId,
    },
    outbound::entity::user::UserId,
};

es_entity::entity_id! { EventId }

#[derive(EsEvent, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "EventId")]
pub enum EventEvent {
    Initialized {
        id: EventId,
        name: String,
        description: String,
        start_date: NaiveDate,
        website_url: String,
        image_url: String,
        source: String,
        source_url: Option<String>,
        user_id: UserId,
    },
    EventApproved {
        reason: Option<String>,
        user_id: UserId,
    },
    EventAutoApproved,
    EventRejected {
        reason: Option<String>,
        user_id: UserId,
    },
    EventDeleted {
        reason: Option<String>,
        user_id: UserId,
    },
    EventVoted {
        power: i32,
        comment: Option<String>,
        user_id: UserId,
    },
    EditProposal {
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        website_url: Option<String>,
        image_url: Option<String>,
        source: String,
        source_url: Option<String>,
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
    StartVerification {
        user_id: UserId,
        verification_id: Uuid,
    },
    VerificationCompleted {
        user_id: UserId,
        verification_id: Uuid,
    },
}

#[derive(Debug, Serialize, Deserialize, Default, sqlx::Type)]
#[sqlx(type_name = "event_state", rename_all = "snake_case")]
enum EventState {
    #[default]
    Unapproved,
    Live,
    Rejected,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize)]
struct EditProposal {
    upvotes: u32,
    downvotes: u32,
    score: i32,
    name: Option<String>,
    description: Option<String>,
    website_url: Option<String>,
    image_url: Option<String>,
    source: String,
    source_url: Option<String>,
    user_id: UserId,
}

#[derive(EsEntity, Builder, Serialize)]
#[builder(pattern = "owned", build_fn(error = "EntityHydrationError"))]
pub struct Event {
    pub id: EventId,
    pub name: String,
    #[builder(default)]
    pub description: String,
    pub website_url: String,
    pub image_url: String,
    pub start_date: NaiveDate,

    // Approval
    #[builder(default)]
    pub state: EventState,
    pub upvotes: u32,
    pub downvotes: u32,
    pub score: i32,

    // Edits
    pub edit_proposals: IndexMap<Uuid, EditProposal>,

    // Official organizer
    #[builder(default)]
    pub organizer_id: Option<UserId>,
    #[builder(default)]
    pub verified: bool,

    #[serde(skip)]
    #[builder(default)]
    verification_id: HashMap<UserId, Uuid>,

    #[serde(skip)]
    events: EntityEvents<EventEvent>,
}

impl From<Event> for EventDetails {
    fn from(event: Event) -> Self {
        Self {
            id: event.id.into(),
            title: event.name,
            description: event.description,
            image_url: event.image_url,
            start_date: event.start_date,
        }
    }
}

// Any EsEntity must implement `TryFromEvents`.
// This trait is what hydrates entities after loading the events from the database
impl TryFromEvents<EventEvent> for Event {
    fn try_from_events(events: EntityEvents<EventEvent>) -> Result<Self, EntityHydrationError> {
        let mut builder = EventBuilder::default();
        let mut edit_proposals = IndexMap::new();
        let mut upvotes = 0;
        let mut downvotes = 0;
        let mut score = 0;

        for event in events.iter_all() {
            match event {
                EventEvent::Initialized {
                    id,
                    name,
                    description,
                    start_date,
                    website_url,
                    image_url,
                    source: _,
                    source_url: _,
                    user_id: _,
                } => {
                    builder = builder
                        .id(*id)
                        .name(name.clone())
                        .description(description.clone())
                        .start_date(*start_date)
                        .website_url(website_url.clone())
                        .image_url(image_url.clone())
                        .organizer_id(None)
                        .verified(false)
                        .verification_id(HashMap::new());
                }
                EventEvent::EventVoted {
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
                EventEvent::EventApproved { .. } | EventEvent::EventAutoApproved => {
                    builder = builder.state(EventState::Live);
                }
                EventEvent::EventRejected {
                    reason: _,
                    user_id: _,
                } => {
                    builder = builder.state(EventState::Rejected);
                }
                EventEvent::EventDeleted {
                    reason: _,
                    user_id: _,
                } => {
                    builder = builder.state(EventState::Deleted);
                }
                EventEvent::EditProposal {
                    id,
                    name,
                    description,
                    website_url,
                    image_url,
                    source,
                    source_url,
                    user_id,
                } => {
                    edit_proposals.insert(
                        *id,
                        EditProposal {
                            upvotes: 0,
                            downvotes: 0,
                            score: 0,
                            name: name.clone(),
                            description: description.clone(),
                            website_url: website_url.clone(),
                            image_url: image_url.clone(),
                            source: source.clone(),
                            source_url: source_url.clone(),
                            user_id: *user_id,
                        },
                    );
                }
                EventEvent::EditVoted {
                    id,
                    power,
                    comment: _,
                    user_id: _,
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
                EventEvent::EditApproved { id, .. } | EventEvent::EditAutoApproved { id } => {
                    if let Some(proposal) = edit_proposals.shift_remove(id) {
                        if let Some(name) = &proposal.name {
                            builder = builder.name(name.clone());
                        }
                        if let Some(description) = &proposal.description {
                            builder = builder.description(description.clone());
                        }
                        if let Some(website_url) = &proposal.website_url {
                            builder = builder.website_url(website_url.clone());
                        }
                        if let Some(image_url) = &proposal.image_url {
                            builder = builder.image_url(image_url.clone());
                        }
                    }
                }
                EventEvent::EditRejected {
                    id,
                    reason: _,
                    user_id: _,
                } => {
                    edit_proposals.shift_remove(id);
                }
                EventEvent::StartVerification {
                    user_id,
                    verification_id,
                } => {
                    builder
                        .verification_id
                        .get_or_insert_default()
                        .insert(*user_id, *verification_id);
                }
                EventEvent::VerificationCompleted {
                    user_id,
                    verification_id,
                } => {
                    if builder.verification_id.get_or_insert_default().get(user_id)
                        != Some(verification_id)
                    {
                        // This means that either the verification ID doesn't exist, or it doesn't match the one in the event.
                        // We will ignore this event, as it is either invalid or has already been processed.
                        continue;
                    }

                    builder = builder.organizer_id(Some(*user_id)).verified(true);
                    builder
                        .verification_id
                        .get_or_insert_default()
                        .remove(user_id);
                }
            }
        }

        builder
            .upvotes(upvotes)
            .downvotes(downvotes)
            .score(score)
            .events(events)
            .edit_proposals(edit_proposals)
            .build()
    }
}

#[derive(Debug)]
pub struct NewEvent {
    id: EventId,
    name: String,
    description: String,
    website_url: String,
    image_url: String,
    start_date: NaiveDate,
    source: String,
    source_url: Option<String>,
    state: EventState,
    user_id: UserId,
}

impl NewEvent {
    pub fn from_domain(request: &CreateEventRequest, author_id: &DomainUserId) -> Self {
        let event_id = Uuid::new_v4().into();
        Self {
            id: event_id,
            name: request.name().as_ref().to_string(),
            description: request.description().as_ref().to_string(),
            website_url: request.website_url().as_ref().to_string(),
            image_url: request.image_url().as_ref().to_string(),
            start_date: request.start_date(),
            source: request.source().as_ref().to_string(),
            source_url: request
                .source_url()
                .as_ref()
                .map(|u| u.as_ref().to_string()),
            state: EventState::Unapproved,
            user_id: author_id.clone().into_inner().into(),
        }
    }
}

impl IntoEvents<EventEvent> for NewEvent {
    fn into_events(self) -> EntityEvents<EventEvent> {
        EntityEvents::init(
            self.id,
            [EventEvent::Initialized {
                id: self.id,
                name: self.name,
                description: self.description,
                website_url: self.website_url,
                image_url: self.image_url,
                start_date: self.start_date,
                source: self.source,
                source_url: self.source_url,
                user_id: self.user_id,
            }],
        )
    }
}

#[derive(EsRepo, Debug, Clone)]
#[es_repo(
    entity = "Event",
    // Configure the columns that need populating in the index table
    columns(
        // The 'name' column
        name(ty = "String"),
        start_date(ty = "NaiveDate", list_by),
        state(ty = "EventState")
    )
)]
pub struct EventRepo {
    // Mandatory field so that the Repository can begin transactions
    pub pool: sqlx::PgPool,
}

impl EventRepo {
    pub async fn future_events(&self) -> Result<Vec<Event>, EventQueryError> {
        es_query!("SELECT * FROM events WHERE start_date > NOW()")
            .fetch_n(&self.pool, usize::MAX)
            .await
            .map(|(r, _)| r)
    }
}
