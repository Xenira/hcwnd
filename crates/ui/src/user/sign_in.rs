use api::UiState;
use maud::{Markup, Render, html};

use crate::index;

pub struct SignIn;

pub const BASE_ROUTE: &str = "/login";

#[must_use]
pub fn full_page(state: &UiState) -> Markup {
    index::full_page(
        state,
        t!("user.sign_in.title", locale = &state.locale),
        render(&state.locale),
    )
}

#[must_use]
pub fn render(locale: &str) -> Markup {
    html! {
        form hx-post=(BASE_ROUTE) hx-swap="innerHTML" {
            label {
                (t!("user.sign_in.email.label", locale = locale))
                input type="email" name="email" required;
            }
            label {
                (t!("user.sign_in.password.label", locale = locale))
                input type="password" name="password" required;
            }
            footer {
                button.btn-primary
                    type="submit"
                {
                    (t!("user.sign_in.submit", locale = locale))
                }
                a
                    href=(super::sign_up::BASE_ROUTE)
                    hx-boost="true"
                    hx-target="main"
                {
                    (t!("user.sign_in.sign_up_link", locale = locale))
                }
            }
        }
    }
}
