use derive_builder::Builder;
use maud::{html, Markup, Render};
use uuid::Uuid;

use crate::{
    component::{menu_item, Icons},
    event::{details::EventDetails, lineup::EventLineup, timetable::EventTimetable},
};

pub mod act;
pub mod card;
pub mod create;
pub mod details;
pub mod lineup;
pub mod list;
pub mod timetable;

pub const EVENT_CONTAINER_ID: &str = "event";
pub const EVENT_CONTENT_CONTAINER_ID: &str = "event_content";

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct Event {
    pub id: Uuid,
    pub outlet: EventRoute,
}

impl Render for Event {
    fn render(&self) -> maud::Markup {
        html! {
            div id=(EVENT_CONTAINER_ID) {
                (self.nav_bar())
                (self.outlet)
            }
        }
    }
}

impl Event {
    pub fn nav_bar(&self) -> Markup {
        let details_url = format!("/event/{}", self.id);
        let details_active = matches!(self.outlet, EventRoute::Details(_));

        let timetable_url = format!("/event/{}/timetable", self.id);
        let timetable_active = matches!(self.outlet, EventRoute::Timetable(_));

        let lineup_url = format!("/event/{}/lineup", self.id);
        let lineup_active = matches!(self.outlet, EventRoute::Lineup(_));

        let target = format!("#{EVENT_CONTAINER_ID}");
        html! {
            nav {
                menu {
                    (menu_item("Details", Some(Icons::EventDetails), &details_url, &target, "event_menu", details_active))
                    (menu_item("Timetable", Some(Icons::EventTimetable), &timetable_url, &target, "event_menu", timetable_active))
                    (menu_item("Lineup", Some(Icons::EventLineup), &lineup_url, &target, "event_menu", lineup_active))
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum EventRoute {
    Details(EventDetails),
    Timetable(EventTimetable),
    Lineup(EventLineup),
}

impl Render for EventRoute {
    fn render(&self) -> Markup {
        let content = match self {
            EventRoute::Details(details) => details.render(),
            EventRoute::Timetable(timetable) => timetable.render(),
            EventRoute::Lineup(lineup) => lineup.render(),
        };

        html! {
            div id=(EVENT_CONTENT_CONTAINER_ID) {
                (content)
            }
        }
    }
}
