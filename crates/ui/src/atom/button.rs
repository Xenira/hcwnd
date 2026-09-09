use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::component::Icons;

#[derive(TypedBuilder)]
pub struct Button {
    label: String,
    #[builder(default, setter(strip_option))]
    icon: Option<Icons>,
}

impl Render for Button {
    fn render(&self) -> Markup {
        let icon = if let Some(i) = &self.icon {
            crate::component::icon(i, None)
        } else {
            html! {}
        };

        html! {
            button type="button" {
                (icon)
                span {
                    (self.label)
                }
            }
        }
    }
}
