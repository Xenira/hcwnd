use derive_builder::Builder;
use maud::{html, Markup, Render};

use crate::component::{icon, Icons};

pub const DIALOG_CONTENT_ID: &str = "dialog_content";

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct Dialog {
    pub id: String,
    pub content: Markup,
    pub url: Option<String>,
    #[builder(default = "true")]
    pub close_button: bool,
}

impl Render for Dialog {
    fn render(&self) -> Markup {
        dialog(
            &self.id,
            self.content.clone(),
            self.url.as_deref(),
            self.close_button,
        )
    }
}

#[must_use]
pub fn dialog(id: &str, content: Markup, url: Option<&str>, close_button: bool) -> Markup {
    let dialog_content_id = format!("{DIALOG_CONTENT_ID}-{id}");
    let content_id = format!("#{dialog_content_id}");
    html! {
        dialog #(id) hx-get=[url] hx-trigger="toggle" hx-target=(content_id) {
            @if close_button {
                button.close type="button" command="close" commandfor=(id) {
                    (icon(&Icons::Close, None))
                }
            }
            #(dialog_content_id) {
                (content)
            }
        }
    }
}
