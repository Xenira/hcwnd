use api::UiState;
use maud::{Markup, Render, html};

use crate::index;

pub struct SignUp;

#[cfg(debug_assertions)]
const MIN_PASSWORD_LENGTH: usize = 3;
#[cfg(not(debug_assertions))]
const MIN_PASSWORD_LENGTH: usize = 12;

pub const BASE_ROUTE: &str = "/signup";

#[must_use]
pub fn full_page(state: &UiState) -> Markup {
    index::full_page(
        state,
        t!("user.sign_up.title", locale = &state.locale),
        render(&state.locale),
    )
}

#[must_use]
pub fn render(locale: &str) -> Markup {
    html! {
        form hx-post=(BASE_ROUTE) hx-swap="innerHTML" {
            fieldset {
                label {
                    (t!("user.sign_up.username.label", locale = locale))
                    input
                        type="text"
                        name="username"
                        minlength="3"
                        maxlength="32"
                        required;
                    small {
                        (t!("user.sign_up.username.hint", locale = locale))
                    }
                }
                label {
                    (t!("user.sign_up.email.label", locale = locale))
                    input type="email" name="email" required;
                    small {
                        (t!("user.sign_up.email.hint", locale = locale))
                    }
                }
                label {
                    (t!("user.sign_up.password.label", locale = locale))
                    input
                        type="password"
                        name="password"
                        minlength=(MIN_PASSWORD_LENGTH)
                        required;
                }
                label {
                    (t!("user.sign_up.confirm_password.label", locale = locale))
                    input
                        type="password"
                        name="confirm_password"
                        minlength=(MIN_PASSWORD_LENGTH)
                        required;
                }
                label {
                    input type="checkbox" name="privacy_policy" value="true" required;
                    a href="/privacy-policy" { (t!("user.sign_up.privacy_policy", locale = locale)) }
                }
            }
            button.btn-primary
                type="submit"
            {
                (t!( "user.sign_up.submit", locale = locale))
            }
            a
                href=(super::sign_in::BASE_ROUTE)
                hx-boost="true"
                hx-target="main"
            {
                (t!( "user.sign_up.sign_in_link", locale = locale))
            }
        }
    }
}
