use maud::{html, Markup};
use uuid::Uuid;

use crate::component::{menu_item, Icons};

pub mod detail;

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
pub fn nav_bar(state: &api::UiState, event_id: Uuid, active_view: View) -> Markup {
    let target = format!("#{EVENT_CONTAINER_ID}");

    let details_url = format!("/event/{event_id}");
    let details_menu = menu_item(
        &t!("event.detail.menu.details", locale = &state.locale),
        Some(Icons::EventDetails),
        &details_url,
        &target,
        "event_menu",
        active_view == View::Detail,
    );

    let timetable_url = format!("/event/{event_id}/timetable");
    let timetable_menu = menu_item(
        &t!("event.detail.menu.timetable", locale = &state.locale),
        Some(Icons::EventTimetable),
        &timetable_url,
        &target,
        "event_menu",
        active_view == View::Timetable,
    );

    let lineup_url = format!("/event/{event_id}/lineup");
    let lineup_menu = menu_item(
        "Lineup",
        Some(Icons::EventLineup),
        &lineup_url,
        &target,
        "event_menu",
        active_view == View::Lineup,
    );

    html! {
        nav {
            menu {
                (details_menu)
                (timetable_menu)
                (lineup_menu)
            }
        }
    }
}
