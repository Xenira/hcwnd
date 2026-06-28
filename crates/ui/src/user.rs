use maud::{Markup, html};

pub mod sign_in;
pub mod sign_up;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
}

impl User {
    #[must_use]
    pub fn handle(&self) -> Markup {
        html! {
            span { "@" (&self.username) }
        }
    }
}
