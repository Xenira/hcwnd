use derive_builder::Builder;
use itertools::Itertools as _;
use maud::{html, Markup, Render};
use uuid::Uuid;

use crate::act::{
    card::ActCard,
    create::{ActCreate, CREATE_ACT_DIALOG_ID},
};

pub const ACTS_LIST_ID: &str = "acts_list";

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct EventLineup {
    pub event_id: Uuid,
    pub acts: Vec<ActCard>,
    pub stages: Vec<(Uuid, String)>,
}

impl Render for EventLineup {
    fn render(&self) -> Markup {
        html! {
            section {
                form.flex.row {
                    input.grow type="search" name="search" placeholder="Search acts...";
                    select name="stage" {
                        option value="" { "All Stages" }
                        @for (stage_id, stage_name) in &self.stages {
                            option value=(stage_id) { (stage_name) }
                        }
                    }
                }
                button command="show-modal" commandfor=(CREATE_ACT_DIALOG_ID) {
                    "Add Act"
                }
                div id=(ACTS_LIST_ID) {
                    @for act in &self.acts {
                        (act)
                    }
                }
            }

            (ActCreate { event_id: self.event_id })
        }
    }
}
