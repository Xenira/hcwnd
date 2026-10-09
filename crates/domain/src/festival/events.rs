use chrono::{NaiveDate, NaiveTime};
use eventwise::aggregate::{Apply, Create};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    festival::models::festival::{
        EventDescription, EventName, Festival, FestivalActs, FestivalDays, FestivalId,
        FestivalStages, FestivalState, ImageUrl, WebsiteUrl,
    },
    suggestion::{
        events::{
            impl_suggestion_events, SuggestionAccepted, SuggestionRejected, SuggestionVoteRemoved,
            SuggestionVoted,
        },
        SuggestionState,
    },
    user::models::user::UserId,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FestivalSuggested {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    // pub event_type: EventType,
    pub venue: Uuid,
    pub website_url: Url,
    pub image_url: Url,
    pub start_date: NaiveDate,
    pub days: Vec<(Option<NaiveTime>, Option<NaiveTime>)>,
    pub score: i32,
    pub user_id: Uuid,
}

impl Create<FestivalSuggested> for Festival {
    fn bootstrap(event: FestivalSuggested) -> Self {
        let id = FestivalId::new(event.id);
        let name = unsafe { EventName::new_unchecked(event.name) };
        let description = unsafe { EventDescription::new_unchecked(event.description) };
        let website_url = unsafe { WebsiteUrl::new_unchecked(event.website_url) };
        let image_url = unsafe { ImageUrl::new_unchecked(event.image_url) };
        let event_days = unsafe { FestivalDays::new_unchecked(event.days) };
        let user_id = UserId::new(event.user_id);

        Self::builder()
            .id(id)
            .name(name)
            .description(description)
            .website_url(website_url)
            .image_url(image_url)
            .stages(FestivalStages::new(vec![]))
            .acts(FestivalActs::new(vec![]))
            .start_date(event.start_date)
            .days(event_days)
            .state(SuggestionState::Suggested {
                votes: vec![(user_id.clone(), event.score)].into_iter().collect(),
                user_id,
            })
            .build()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeSuggested {
    pub id: Uuid,
    pub change: FestivalChange,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FestivalChange {
    Name(String),
    Description(String),
    WebsiteUrl(Url),
    ImageUrl(Url),
    StartDate(NaiveDate),
    Days(u8),
}

impl Apply<ChangeSuggested> for Festival {
    fn apply(&mut self, event: ChangeSuggested) {
        match event.change {
            FestivalChange::Name(name) => {
                self.set_name(unsafe { EventName::new_unchecked(name) });
            }
            FestivalChange::Description(description) => {
                self.set_description(unsafe { EventDescription::new_unchecked(description) });
            }
            FestivalChange::WebsiteUrl(website_url) => {
                self.set_website_url(unsafe { WebsiteUrl::new_unchecked(website_url) });
            }
            FestivalChange::ImageUrl(image_url) => {
                self.set_image_url(unsafe { ImageUrl::new_unchecked(image_url) });
            }
            FestivalChange::StartDate(start_date) => {
                self.set_start_date(start_date);
            }
            FestivalChange::Days(days) => {
                self.set_days(unsafe {
                    FestivalDays::new_unchecked(vec![(None, None); days as usize])
                });
            }
        }
    }
}

impl_suggestion_events!(Festival, state_mut);
