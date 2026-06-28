use chrono::{NaiveDate, NaiveTime};
use derive_builder::Builder;
use maud::{Markup, Render, html};
use url::Url;
use uuid::Uuid;

use crate::{
    component::{Icons, icon},
    user::User,
};

#[derive(Debug)]
pub struct EventCard {
    pub id: Uuid,
    pub title: String,
    pub image_url: Url,
    pub start_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_date: NaiveDate,
    pub end_time: Option<NaiveTime>,
}

impl Render for EventCard {
    fn render(&self) -> Markup {
        html! {
            article {
                header {
                    img src=(self.image_url) alt=(self.title);
                    h2 { (self.title) }
                }
                (date_line(self.start_date, self.start_time, self.end_date, self.end_time))
            }
        }
    }
}

#[derive(Debug)]
pub struct EventSuggestionCard {
    pub editable: bool,
    pub suggested_by: User,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub start_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_date: NaiveDate,
    pub end_time: Option<NaiveTime>,
    pub upvotes: u32,
    pub downvotes: u32,
}

impl Render for EventSuggestionCard {
    fn render(&self) -> Markup {
        html! {
            article.card-warning.event {
                img fetchpriority="high" src=(self.image_url) alt=(self.title);
                section.hero.hero-warning {
                    div.container {
                        h1 { (self.title) }
                        p {
                            (date_line(self.start_date, self.start_time, self.end_date, self.end_time))
                        }
                        p.short-description { (self.description) }
                    }
                }
            }
        }
    }
}

// footer {
//     span { "Suggested by" (self.suggested_by.handle()) }
//     (vote_count(self.upvotes, self.downvotes))
//     (vote())
// }

fn date_line(
    start_date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_date: NaiveDate,
    end_time: Option<NaiveTime>,
) -> Markup {
    let start = if let Some(time) = start_time {
        format!("{} {}", start_date.format("%d.%m.%Y"), time.format("%H:%M"))
    } else {
        start_date.format("%d.%m.%Y").to_string()
    };

    let end = if let Some(time) = end_time {
        if start_date == end_date {
            time.format("%H:%M").to_string()
        } else {
            format!("{} {}", end_date.format("%d.%m.%Y"), time.format("%H:%M"))
        }
    } else {
        end_date.format("%d.%m.%Y").to_string()
    };

    html! {
        (icon(&Icons::Date, None))
        span {
            (format!("{} - {}", start, end))
        }
    }
}

fn vote() -> Markup {
    html! {
        div role="group" {
            button.btn-error
                title="Downvote"
                data-tooltip="Downvote"
                data-placement="bottom"
            {
                (icon(&Icons::Downvote, None))
            }
            button.btn-success
                title="Upvote" data-tooltip="Upvote"
            {
                (icon(&Icons::Upvote, None))
            }
        }
    }
}

fn vote_count(upvotes: u32, downvotes: u32) -> Markup {
    let max = u32::max(upvotes, downvotes);
    html! {
        div.votes {
            progress.progress-error.reverse
                value=(downvotes)
                max=(max)
            {}
            progress.progress-success
                value=(upvotes)
                max=(max)
            {}
        }
    }
}
