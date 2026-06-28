use maud::{Markup, Render, html};

pub struct SignIn;

pub const BASE_ROUTE: &str = "/login";

impl Render for SignIn {
    fn render(&self) -> Markup {
        html! {
            form hx-post=(BASE_ROUTE) hx-swap="innerHTML" {
                label {
                    (t!("user.sign_in.email.label"))
                    input type="email" name="email" required;
                }
                label {
                    (t!("user.sign_in.password.label"))
                    input type="password" name="password" required;
                }
                footer {
                    button.btn-primary
                        type="submit"
                    {
                        (t!("user.sign_in.submit"))
                    }
                    a
                        href=(super::sign_up::BASE_ROUTE)
                        hx-boost="true"
                        hx-target="main"
                    {
                        (t!("user.sign_in.sign_up_link"))
                    }
                }
            }
        }
    }
}
