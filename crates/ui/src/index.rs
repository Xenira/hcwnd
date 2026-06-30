use std::fmt::Display;

use api::{UiState, user::User};
use maud::{DOCTYPE, Markup, Render, html};

use crate::user::{self};

pub fn full_page(state: &UiState, title: impl Display, content: Markup) -> Markup {
    let hx_config = r#"{"disableInheritance": true}"#;

    html! {
        (DOCTYPE)
        html lang=(state.locale){
            head {
                title { (format_title(&state.locale, title)) }
                meta content="text/html;charset=utf-8" http-equiv="Content-Type";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta charset="UTF-8";
                meta name="htmx-config" content=(hx_config);
                (styles())
                (scripts())
            }
            body {
                (nav_bar(state.user.as_ref()))
                main.container #main {
                    (content)
                }
            }
        }
    }
}

fn format_title(locale: &str, title: impl Display) -> String {
    let name = t!("app.name", locale = locale);
    let slogan = t!("app.slogan", locale = locale);
    format!("{title} | {name} - {slogan}")
}

fn nav_bar(user: Option<&User>) -> Markup {
    let user = if let Some(user) = user {
        let profile_url = format!("/users/{}", user.name);
        html! {
            li {
                details.dropdown {
                    summary {
                        (user::handle(user))
                    }
                    ul dir="rtl" {
                        li {
                            a hx-get=(profile_url)
                                hx-target="#main"
                                hx-swap="innerHTML"
                                hx-push-url="true"
                            {
                                "Profile"
                            }
                        }
                        li {
                            a hx-get="/logout"
                                hx-target="#main"
                                hx-swap="innerHTML"
                                hx-push-url="true"
                                href="/logout"
                            {
                                "Logout"
                            }
                        }
                    }
                }
            }
        }
    } else {
        html! {
            li {
                a hx-get="/login"
                    hx-target="#main"
                    hx-swap="innerHTML"
                    hx-push-url="true"
                    href="/login"
                {
                    span { "Login" }
                }
            }
            li {
                a hx-get="/signup"
                  hx-target="#main"
                  hx-swap="innerHTML"
                  hx-push-url="true"
                  href="/signup" {
                    span { "Sign Up" }
                }
            }
        }
    };
    html! {
        header.container-fluid.sticky-top.bg-primary {
            nav
                hx-boost="true"
            {
                ul {
                    li {
                        strong {
                            #home { a.flex href="/" { "❮" span { "HCWND" } "❯" } }
                        }
                    }
                }
                ul {
                    (user)
                }
            }
        }
    }
}

fn scripts() -> Markup {
    html! {
        script src="/assets/htmx.min.js" {}
    }
}

fn styles() -> Markup {
    html! {
        link rel="stylesheet" href="/assets/mu.css";
        link rel="stylesheet" href="/assets/style.css";
        link rel="stylesheet" href="/assets/icons/regular/style.css";
        link rel="stylesheet" href="/assets/icons/fill/style.css";
        link rel="stylesheet" href="/assets/fonts/index.css";
    }
}
