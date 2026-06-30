use maud::{Markup, Render, html};

pub const ADD_ARTIST_DIALOG_ID: &str = "add_artist_dialog";

#[derive(Debug)]
pub struct ArtistCreate {}

impl Render for ArtistCreate {
    fn render(&self) -> Markup {
        let form_action = "/artist";
        let content = html! {
            div.slide-it {
                form #create_artist_form hx-post=(form_action) {
                    label {
                        "Artist Name"
                        input type="text" name="name" placeholder="Enter Artist Name" required {}
                    }
                    label {
                        "Genres"
                        input type="text" name="genres" placeholder="Enter genres, separated by commas" {}
                    }

                    footer {
                        button.error type="button" command="close" commandfor=(ADD_ARTIST_DIALOG_ID) {
                            "Close"
                        }

                        button type="submit" {
                            "Create Artist"
                        }
                    }
                }
            }
        };

        content
    }
}
