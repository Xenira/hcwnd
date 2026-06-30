use api::{UiState, event::Event, user::User};
use maud::{Markup, Render, html};
use uuid::Uuid;

use crate::{
    event::{self, card::EventSuggestionCard},
    index,
};

#[must_use]
pub fn full_page(state: &UiState, events: &[Event]) -> Markup {
    index::full_page(
        state,
        t!("app.title.home", locale = &state.locale),
        render(state, events, 1, false),
    )
}

#[must_use]
pub fn render(state: &UiState, events: &[Event], page: usize, has_more: bool) -> Markup {
    let preview = EventSuggestionCard {
            editable: true,
            suggested_by: User {
                id: Uuid::nil(),
                name: "hardcore".to_string(),
                score: 0,
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
                input type="search" name="query" placeholder=(t!("event.list.search.placeholder", locale = &state.locale));
                input type="submit" value=(t!("event.list.search.submit", locale = &state.locale));
            }
            @if events.is_empty() {
                p {
                    h2 { (t!("event.list.empty", locale = &state.locale)) }
                    a role="button" href="/create-event/name" hx-target="#main" hx-push-url="true" hx-boost="true" { (t!("event.list.empty.create", locale = &state.locale)) }
                }
            }
        }

        (preview)

        @for event in events {
            (event::card::render(state, event))
        }

        (pagination(page, has_more))
    }
}

fn pagination(page: usize, has_more: bool) -> Markup {
    if has_more {
        let next_page = page + 1;
        let url = format!("/events?page={next_page}");
        html! {
            a hx-get=(url) hx-target="#main" { "Load more" }
        }
    } else {
        html! {}
    }
}
