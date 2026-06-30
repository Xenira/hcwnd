use api::user::User;
use maud::{Markup, html};

pub mod sign_in;
pub mod sign_up;

#[must_use]
pub fn handle(user: &User) -> Markup {
    html! {
        span { "@" (&user.name) }
    }
}

#[must_use]
pub fn unauthenticated(locale: &str) -> Markup {
    html! {
        div.alert.alert-error {
            span.alert-title { (t!("user.unauthenticated.title", locale = locale)) }
            p { (t!("user.unauthenticated.message", locale = locale)) }
            a
                href=(sign_in::BASE_ROUTE)
                hx-boost="true"
                hx-target="#main"
                hx-push-url="true"
            {
                (t!("user.unauthenticated.sign_in_link", locale = locale))
            }
        }
    }
}
