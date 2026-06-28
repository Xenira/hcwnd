use chrono::{NaiveDate, NaiveDateTime};
use derive_builder::Builder;
use maud::{html, Markup, Render};
use uuid::Uuid;

use crate::{act::card::ActCard, component::dialog};

pub const SEARCH_RESULTS_ID: &str = "search_results";
pub const ADD_ACT_DIALOG_ID: &str = "add_act_dialog";

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct SelectAct {
    pub event_id: Uuid,
    pub search_results: ActSearchResults,
}

impl Render for SelectAct {
    fn render(&self) -> maud::Markup {
        let select_stage_url = format!("/event/{}/timetable/add/stage", self.event_id);
        let content = html! {
            div.slide-it {
                form hx-get=(select_stage_url) hx-swap="outerHTML transition:true" {
                    h2 { "Add Act to Timetable" }
                    input
                        type="text"
                        name="q"
                        placeholder="Search for acts..."
                        autofocus
                        hx-get=(format!("/event/{}/timetable/add/act/search", self.event_id))
                        hx-target=(format!("#{SEARCH_RESULTS_ID}"))
                        hx-swap="innerHTML"
                        hx-trigger="keyup changed delay:500ms"
                    { }
                    div id=(SEARCH_RESULTS_ID) {
                        (self.search_results)
                    }
                }
            }
        };

        content
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct ActSearchResults {
    pub acts: Vec<ActCard>,
    pub create_new: Option<ActCard>,
}

impl Render for ActSearchResults {
    fn render(&self) -> Markup {
        html! {
            @if self.acts.is_empty() {
                div.flash.primary {
                    "No acts found. Try searching for something else or create a new act."
                }
            } @else {
                h3 { "From Lineup" }
                @for act in &self.acts {
                    button type="submit" name="select_act" value=(act.id) {
                        (act)
                    }
                }
            }

            @if let Some(create_new) = &self.create_new {
                h3 { "Create New Act" }
                button.accent type="submit" name="create_act" value=(create_new.name) {
                    (create_new)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum SelectActResult {
    Select(Uuid),
    Create(String),
}

impl Render for SelectActResult {
    fn render(&self) -> Markup {
        match self {
            SelectActResult::Select(act_id) => {
                html! { input type="hidden" name="select_act" value=(act_id) { } }
            }
            SelectActResult::Create(name) => {
                html! { input type="hidden" name="create_act" value=(name) { } }
            }
        }
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct SelectStage {
    pub event_id: Uuid,
    pub act: SelectActResult,
    pub search_results: StageSearchResults,
}

impl Render for SelectStage {
    fn render(&self) -> Markup {
        let set_time_url = format!("/event/{}/timetable/add/time", self.event_id);
        html! {
            form hx-get=(set_time_url) hx-swap="outerHTML transition:true" {
                h2 { "Select Stage" }
                input
                    type="text"
                    name="q"
                    placeholder="Search for Stage..."
                    autofocus
                    hx-get=(format!("/event/{}/timetable/add/stage/search", self.event_id))
                    hx-target=(format!("#{SEARCH_RESULTS_ID}"))
                    hx-trigger="keyup changed delay:500ms"
                    hx-swap="innerHTML"
                { }

                (self.act)
                div id=(SEARCH_RESULTS_ID) {
                    (self.search_results)
                }
            }
        }
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct StageSearchResults {
    pub stages: Vec<StageSearchResult>,
    pub create_new: Option<StageSearchResult>,
}

impl Render for StageSearchResults {
    fn render(&self) -> Markup {
        html! {
            @if self.stages.is_empty() {
                div.flash.primary {
                    "No stages found. Try searching for something else or create a new stage."
                }
            } @else {
                h3 { "Existing Stages" }
                @for stage in &self.stages {
                    (stage)
                }
            }

            @if let Some(create_new) = &self.create_new {
                h3 { "Create New Stage" }
                (create_new)
            }
        }
    }
}

#[derive(Debug)]
pub enum StageSearchResult {
    Existing { stage_id: Uuid, name: String },
    New { name: String },
}

impl Render for StageSearchResult {
    fn render(&self) -> Markup {
        let (name, new, key, value) = match self {
            StageSearchResult::Existing { stage_id, name } => {
                (name, false, "select_stage", stage_id.to_string())
            }
            StageSearchResult::New { name } => (name, true, "create_stage", name.clone()),
        };
        html! {
            button.accent[new] type="submit" name=(key) value=(value) {
                (name)
            }
        }
    }
}

#[derive(Debug)]
pub enum SelectStageResult {
    Select(Uuid),
    Create(String),
}

impl Render for SelectStageResult {
    fn render(&self) -> Markup {
        match self {
            Self::Select(act_id) => {
                html! { input type="hidden" name="select_stage" value=(act_id) { } }
            }
            Self::Create(name) => {
                html! { input type="hidden" name="create_stage" value=(name) { } }
            }
        }
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct SetTime {
    pub event_id: Uuid,
    pub act: SelectActResult,
    pub stage: SelectStageResult,
    pub days: Vec<(Uuid, NaiveDateTime, NaiveDateTime)>,
}

impl Render for SetTime {
    fn render(&self) -> Markup {
        let select_stage_url = format!("/event/{}/timetable/add/stage", self.event_id);
        let add_act_and_time_url = format!("/event/{}/timetable/add/act_and_time", self.event_id);
        let add_act_url = format!("/event/{}/timetable/add/act", self.event_id);
        let days = self
            .days
            .iter()
            .enumerate()
            .map(|(i, (id, day, _))| {
                html! {
                    label {
                        (day.date().format("%A"))
                        input type="radio" name="days" value=(id) required checked[i==0] { }
                    }
                }
            })
            .collect::<Vec<_>>();
        html! {
            form hx-post=(add_act_and_time_url) {
                h2 { "Set Time" }
                .flex {
                    @for day in days {
                        (day)
                    }
                }
                label {
                    "Start Time"
                    input type="time" name="start_time" required { }
                }
                label {
                    "End Time"
                    input type="time" name="end_time" required { }
                }

                (self.act)
                (self.stage)

                footer {
                    button.error type="button" hx-get=(select_stage_url) hx-target="closest form" hx-include="closest form" hx-swap="outerHTML transition:true" {
                        "Back"
                    }
                    button.primary type="button" hx-post=(add_act_url) hx-include="closest form" {
                        "Add without time"
                    }
                    button.accent type="submit" {
                        "Add to Timetable"
                    }
                }
            }
        }
    }
}
