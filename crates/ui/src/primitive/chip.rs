use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::component::{icon, Icons};

#[derive(TypedBuilder)]
pub struct Chip {
    pub label: String,
    #[builder(default, setter(strip_option))]
    pub icon: Option<Icons>,
}

impl Render for Chip {
    fn render(&self) -> Markup {
        let icon = if let Some(i) = &self.icon {
            icon(i, None)
        } else {
            html! {}
        };

        html! {
            div.chip
                title=(self.label)
            {
                (icon)
                span {
                    (self.label)
                }
            }
        }
    }
}
