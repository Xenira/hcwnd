use maud::{html, Markup, Render};
use uuid::Uuid;

use crate::{component::dialog, event::EVENT_CONTENT_CONTAINER_ID};

pub const CREATE_ACT_DIALOG_ID: &str = "create_act_form";
pub const SEARCH_RESULTS_ID: &str = "artist_search_results";

#[derive(Debug)]
pub struct ActCreate {
    pub event_id: Uuid,
}

impl Render for ActCreate {
    fn render(&self) -> Markup {
        let form_action = format!("/event/{}/act", self.event_id);
        let target = format!("#{EVENT_CONTENT_CONTAINER_ID}");
        let form = html! {
            header {
                h1 { "Add Act" }
            }
            form #create_act_form hx-post=(form_action) hx-target=(target) {
                label {
                    "Act Name"
                    input
                        type="text"
                        name="name"
                        placeholder="Enter Act Name"
                        hx-get=(format!("/artist/act"))
                        hx-target=(format!("#{SEARCH_RESULTS_ID}"))
                        hx-swap="innerHTML"
                        hx-trigger="keyup changed delay:500ms"
                        required {}
                }
                label {
                    "Act Image"
                    input type="url" name="image_url" placeholder="https://example.com/image.jpg" {}
                }
                label {
                    "Artists"
                }
                div #(SEARCH_RESULTS_ID) {
                    (ArtistSearchResults { artists: vec![] })
                }
                footer {
                    button.error type="button" command="close" commandfor=(CREATE_ACT_DIALOG_ID) {
                        "Close"
                    }

                    button type="submit" {
                        "Create Act"
                    }
                }
            }
        };

        html! {
            (dialog(CREATE_ACT_DIALOG_ID, form, None, true))
        }
    }
}

#[derive(Debug)]
pub struct ArtistSearchResults {
    pub artists: Vec<(Uuid, String)>,
}

impl Render for ArtistSearchResults {
    fn render(&self) -> Markup {
        html! {
            @if self.artists.is_empty() {
                div.flash.primary {
                    "No artists found."
                }
            } @else {
                @for artist in &self.artists {
                    input type="checkbox" name="select_artist" value=(artist.0) {
                        (artist.1)
                    }
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
