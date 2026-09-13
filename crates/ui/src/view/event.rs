use api::event::Event;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    atom::{
        nav::{Nav, NavEntry},
        tag::Tag,
    },
    component::{menu_item, Icons},
    index,
};

pub mod detail;
pub mod lineup;
pub mod timetable;

pub const BASE_ROUTE: &str = "/event";

pub const EVENT_CONTAINER_ID: &str = "event";
pub const EVENT_CONTENT_CONTAINER_ID: &str = "event_content";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Detail,
    Timetable,
    Lineup,
}

#[must_use]
pub fn full_page(
    state: &api::UiState,
    title: &str,
    event: &Event,
    active_view: View,
    content: Markup,
) -> Markup {
    index::full_page(state, title, render(state, event, active_view, content))
}

#[must_use]
pub fn render(state: &api::UiState, event: &Event, active_view: View, content: Markup) -> Markup {
    html! {
        div id=(EVENT_CONTAINER_ID) {
            header {
                (header(state, event))
            }
            div role="tablist" {
                (nav_bar(state, event.id, active_view))
            }

            div id=(EVENT_CONTENT_CONTAINER_ID) {
                (content)
            }
        }
    }
}

#[must_use]
fn header(state: &api::UiState, event: &Event) -> Markup {
    let genres = ["Hardcore", "Uptempo"]; // TODO: Replace with actual genres from the event
    html! {
        img src=(event.image_url) alt="Header image" {}
        div {
            @for (i, genre) in genres.iter().enumerate().take(5) {
                (Tag::builder()
                    .label(*genre)
                    .index(i as u8)
                    .build())
            }
        }
        h1 { (event.name) }
    }
}

#[must_use]
pub fn nav_bar(state: &api::UiState, event_id: Uuid, active_view: View) -> Nav {
    Nav::builder()
        .entries(vec![
            NavEntry::builder()
                .href(format!("{}/{event_id}", api::routes::EVENT_ROUTE))
                .label(t!("event.detail.menu.details", locale = &state.locale).to_string())
                .active(active_view == View::Detail)
                .build(),
            NavEntry::builder()
                .href(format!(
                    "{}/{event_id}{}",
                    api::routes::EVENT_ROUTE,
                    api::routes::EVENT_TIMETABLE_ROUTE
                ))
                .label(t!("event.detail.menu.timetable", locale = &state.locale).to_string())
                .active(active_view == View::Timetable)
                .build(),
            NavEntry::builder()
                .href(format!(
                    "{}/{event_id}{}",
                    api::routes::EVENT_ROUTE,
                    api::routes::EVENT_LINEUP_ROUTE
                ))
                .label(t!("event.detail.menu.lineup", locale = &state.locale).to_string())
                .active(active_view == View::Lineup)
                .build(),
        ])
        .build()
}
