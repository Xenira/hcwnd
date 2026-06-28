use derive_builder::Builder;
use maud::{html, Markup, Render};

use crate::event::card::{EventCard, EventSuggestionCard};

pub enum EventListRoute {
    List(EventList),
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct EventList {
    pub events: Vec<EventCard>,
    pub page: usize,
    pub has_more: bool,
}

impl Render for EventList {
    fn render(&self) -> Markup {
        let preview = EventSuggestionCard {
            editable: true,
            suggested_by: crate::user::User {
                username: "hardcore".to_string(),
            },
            title: "Preview Event".to_string(),
            description: "This is a preview of an event that you can create.".to_string(),
            image_url: "https://hardcore.localhost:8443/image/ck4Rfi6qHGlka-X0JAqusANAg-r-fEldIUtoZqx13wQ/rs:auto:1158:650/aHR0cHM6Ly93d3cuc3luZGljYXRlLWZlc3RpdmFsLmRlLzE3NzMyMS8xNzYyNDMzMjY3LXN5bjI2X3Byb2ZpbGVfMTAwMHgxMDAwcHgtbWluLnBuZw.png".to_string(),
            start_date: chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            start_time: Some(chrono::NaiveTime::from_hms_opt(18, 0, 0).unwrap()),
            end_date: chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            end_time: Some(chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap()),
            upvotes: 3,
            downvotes: 1,
        };
        html! {
            form {
                fieldset role="search" {
                    input type="search" name="query" placeholder="Search events...";
                    input type="submit" value="Search";
                }
                @if self.events.is_empty() {
                    p {
                        h2 { "No events found" }
                        a role="button" href="/create-event/name" hx-target="#main" hx-push-url="true" hx-boost="true" { "Create an event" }
                    }
                }
            }

            (preview)

            @for event in &self.events {
                (event)
            }

            (self.pagination())
        }
    }
}

impl EventList {
    fn pagination(&self) -> Markup {
        if self.has_more {
            let next_page = self.page + 1;
            let url = format!("/events?page={}", next_page);
            html! {
                a hx-get=(url) hx-target="#main" { "Load more" }
            }
        } else {
            html! {}
        }
    }
}
