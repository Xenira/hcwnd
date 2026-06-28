use maud::{html, Markup, Render};

pub struct SignIn;

impl Render for SignIn {
    fn render(&self) -> Markup {
        html! {
            form hx-post="/login" hx-swap="innerHTML" {
                label {
                    "Email"
                    input type="email" name="email" required;
                }
                label {
                    "Password"
                    input type="password" name="password" required;
                }
                footer {
                    button type="submit" {
                        "Sign In"
                    }
                }
            }
        }
    }
}
