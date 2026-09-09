use maud::{Markup, Render, html};

pub struct Button {
    pub label: String,
}

impl Render for Button {
    fn render(&self) -> Markup {
        html! {
            span { (self.label) }
        }
    }
}
