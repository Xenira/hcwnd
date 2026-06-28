use maud::{Markup, Render, html};

pub struct SignUp;

#[cfg(debug_assertions)]
const MIN_PASSWORD_LENGTH: usize = 3;
#[cfg(not(debug_assertions))]
const MIN_PASSWORD_LENGTH: usize = 12;

pub const BASE_ROUTE: &str = "/signup";

impl Render for SignUp {
    fn render(&self) -> Markup {
        html! {
            form hx-post=(BASE_ROUTE) hx-swap="innerHTML" {
                fieldset {
                    label {
                        (t!("user.sign_up.username.label"))
                        input
                            type="text"
                            name="username"
                            minlength="3"
                            maxlength="32"
                            required;
                        small {
                            (t!("user.sign_up.username.hint"))
                        }
                    }
                    label {
                        (t!("user.sign_up.email.label"))
                        input type="email" name="email" required;
                        small {
                            (t!("user.sign_up.email.hint"))
                        }
                    }
                    label {
                        (t!("user.sign_up.password.label"))
                        input
                            type="password"
                            name="password"
                            minlength=(MIN_PASSWORD_LENGTH)
                            required;
                    }
                    label {
                        (t!("user.sign_up.confirm_password.label"))
                        input
                            type="password"
                            name="confirm_password"
                            minlength=(MIN_PASSWORD_LENGTH)
                            required;
                    }
                    label {
                        input type="checkbox" name="privacy_policy" value="true" required;
                        a href="/privacy-policy" { (t!("user.sign_up.privacy_policy")) }
                    }
                }
                button.btn-primary
                    type="submit"
                {
                    (t!( "user.sign_up.submit"))
                }
                a
                    href=(super::sign_in::BASE_ROUTE)
                    hx-boost="true"
                    hx-target="main"
                {
                    (t!( "user.sign_up.sign_in_link"))
                }
            }
        }
    }
}
