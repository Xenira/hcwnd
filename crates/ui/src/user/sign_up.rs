use maud::{html, Markup, Render};

pub struct SignUp;

#[cfg(debug_assertions)]
const MIN_PASSWORD_LENGTH: usize = 3;
#[cfg(not(debug_assertions))]
const MIN_PASSWORD_LENGTH: usize = 12;

impl Render for SignUp {
    fn render(&self) -> Markup {
        html! {
            form hx-post="/signup" hx-swap="innerHTML" {
                fieldset {
                    label {
                        "Username"
                        input
                            type="text"
                            name="username"
                            minlength="3"
                            maxlength="32"
                            required;
                        small { "This will be your public name." }
                    }
                    label {
                        "Email"
                        input type="email" name="email" required;
                        small {
                            "Used for login, verification, recovery and notifications you opt-in for. Not publicly visible."
                        }
                    }
                    label {
                        "Password"
                        input
                            type="password"
                            name="password"
                            minlength=(MIN_PASSWORD_LENGTH)
                            required;
                    }
                    label {
                        "Confirm Password"
                        input
                            type="password"
                            name="confirm_password"
                            minlength=(MIN_PASSWORD_LENGTH)
                            required;
                    }
                    label {
                        input type="checkbox" name="privacy_policy" value="true" required;
                        "I agree to the " a href="/privacy-policy" { "Privacy Policy" } "."
                    }
                }
                button type="submit" {
                    "Sign Up"
                }
            }
        }
    }
}
