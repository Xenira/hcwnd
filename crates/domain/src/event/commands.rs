use std::ops::Deref;

use chrono::{NaiveDate, NaiveTime};
use eventwise::command::Handle;
use url::Url;
use uuid::Uuid;

use crate::{
    event::{
        events::FestivalSuggested,
        models::festival::{Festival, FestivalEvent},
    },
    user::models::user::User,
};

pub struct SuggestFestival {
    pub name: String,
    pub description: String,
    pub venue_id: Uuid,
    pub website_url: Url,
    pub image_url: Url,
    pub start_date: NaiveDate,
    pub days: Vec<(Option<NaiveTime>, Option<NaiveTime>)>,
    pub end_date: String,
    pub end_time: Option<String>,
    pub user: User,
}

impl Handle<SuggestFestival> for Festival {
    fn create(
        cmd: SuggestFestival,
    ) -> Result<((), Vec<Self::Event>), eventwise::command::CommandError<Self::Error>> {
        let event = FestivalEvent::FestivalSuggested(FestivalSuggested {
            id: Uuid::now_v7(),
            name: cmd.name,
            description: cmd.description,
            venue: cmd.venue_id,
            website_url: cmd.website_url,
            image_url: cmd.image_url,
            start_date: cmd.start_date,
            days: cmd.days,
            score: cmd.user.reputation(),
            user_id: *cmd.user.id().deref(),
        });

        Ok(((), vec![event]))
    }
}

pub struct VoteForSuggestionCommand {
    pub user_id: Uuid,
    pub power: i32,
}

impl Handle<VoteForSuggestionCommand> for Festival {}
