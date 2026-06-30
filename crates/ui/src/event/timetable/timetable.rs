use chrono::{NaiveDate, NaiveDateTime};
use derive_builder::Builder;
use maud::{Markup, Render, html};
use uuid::Uuid;

use crate::component::{Icons, dialog, icon, menu_item};

pub const CONTENT_OUTLET_ID: &str = "timetable_content";
pub const ADD_ACT_DIALOG_ID: &str = "add-act-dialog";

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct EventTimetable {
    pub event_id: Uuid,
    pub days: Vec<EventDay>,
    pub active_day: EventDay,
}

impl Render for EventTimetable {
    fn render(&self) -> Markup {
        let add_act_url = format!("/event/{}/timetable/add/act", self.event_id);
        let add_act_dialog = dialog(ADD_ACT_DIALOG_ID, html! {}, Some(&add_act_url), true);
        html! {
            div id=(CONTENT_OUTLET_ID) {
                (self.day_menu())
                (self.active_day)
            }
            (add_act_dialog)
        }
    }
}

impl EventTimetable {
    fn day_menu(&self) -> Markup {
        if self.days.len() <= 1 {
            return html! {};
        }
        html! {
            nav {
                menu {
                    @for day in &self.days {
                        (self.day_menu_entry(day))
                    }
                }
            }
        }
    }

    fn day_menu_entry(&self, day: &EventDay) -> Markup {
        let url = format!("/event/{}/timetable/day/{}", self.event_id, day.n);
        let outlet_id = format!("#{CONTENT_OUTLET_ID}");
        html! {
            (menu_item(
                &day.date.format("%d.%m.%Y").to_string(),
                None,
                &url,
                &outlet_id,
                "timetable_day_menu",
                day.n == self.active_day.n
            ))
        }
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", setter(into))]
pub struct EventDay {
    pub event_id: Uuid,
    pub n: u16,
    pub date: NaiveDate,
    pub has_multiple_days: bool,
    pub stages: Vec<EventStage>,
}

impl Render for EventDay {
    fn render(&self) -> Markup {
        html! {
            (self.stage_menu())
            @for stage in &self.stages {
                (stage)
            }
        }
    }
}

impl EventDay {
    fn stage_menu(&self) -> Markup {
        if self.has_multiple_days || self.stages.len() <= 1 {
            return html! {};
        }
        html! {
            nav {
                menu {
                    @for stage in &self.stages {
                        (self.stage_menu_entry(stage))
                    }
                }
            }
        }
    }

    fn stage_menu_entry(&self, stage: &EventStage) -> Markup {
        let url = format!("/event/{}/timetable/stage/{}", self.event_id, stage.name);
        html! {
            li {
                a hx-get=(url) hx-push-url="true" {
                    (stage.name)
                }
            }
        }
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", setter(into))]
pub struct EventStage {
    pub event_id: Uuid,
    pub stage_id: Uuid,
    pub name: String,
    pub has_multiple_stages: bool,
    pub acts: Vec<EventAct>,
}

impl Render for EventStage {
    fn render(&self) -> Markup {
        let details_id = format!("stage-{}", self.stage_id);
        let acts = if self.acts.is_empty() {
            html! {
                div.flash.primary {
                    (icon(&Icons::NoTimetable, None))
                    p {
                        "This seems to be a quiet stage. No acts are scheduled here yet."
                        button command="show-modal" commandfor=(ADD_ACT_DIALOG_ID) {
                            (icon(&Icons::AddAct, None))
                            span.phone { "Add Act" }
                        }
                    }
                }
            }
        } else {
            html! {
                div id=(details_id) {
                    @for act in &self.acts {
                        (act)
                    }
                }
            }
        };

        html! {
            @if self.has_multiple_stages {
                details open {
                    summary {
                        h2 { (self.name) }
                    }
                    (acts)
                }
            } @else {
                (acts)
            }
        }
    }
}

#[derive(Debug, Builder, Clone)]
#[builder(pattern = "owned", setter(into))]
pub struct EventAct {
    pub name: String,
    pub image_url: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

impl Render for EventAct {
    fn render(&self) -> Markup {
        let image = if let Some(image_url) = &self.image_url {
            html! { img src=(image_url) alt=(self.name) { } }
        } else {
            html! { (icon(&Icons::ActImagePlaceholder, None)) }
        };

        html! {
            article {
                aside {
                    (image)
                }
                h2 { (self.name) }
            }
        }
    }
}
