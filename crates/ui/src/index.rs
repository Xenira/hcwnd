use derive_builder::Builder;
use maud::{html, Markup, Render, DOCTYPE};

use crate::{
    artist::{ArtistCreate, ADD_ARTIST_DIALOG_ID},
    component::{dialog, icon},
    event::{card::EventCard, create::EventCreate, list::EventList, Event},
    user::{sign_in::SignIn, sign_up::SignUp, User},
};

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct Index {
    pub title: String,
    #[builder(default)]
    pub user: Option<User>,
    pub outlet: IndexRoute,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum IndexRoute {
    Home(EventList),
    Event(Event),
    CreateEvent(EventCreate),
    SignUp,
    Login,
}

pub trait UiComponent {
    fn render_html(&self) -> String;
}

impl<T: Render> UiComponent for T {
    fn render_html(&self) -> String {
        self.render().into_string()
    }
}

impl Render for Index {
    fn render(&self) -> Markup {
        let hx_config = r#"{"disableInheritance": true}"#;
        let add_artist_dialog = dialog(ADD_ARTIST_DIALOG_ID, html! {}, Some("/artist/add"), true);
        html! {
            (DOCTYPE)
            html {
                head {
                    title { (self.title) " | Hardcore Will Never Diy - Your source for harder style events" }
                    meta content="text/html;charset=utf-8" http-equiv="Content-Type";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta charset="UTF-8";
                    meta name="htmx-config" content=(hx_config);
                    (self.styles())
                    (self.scripts())
                }
                body {
                    (self.nav_bar())
                    main.container #main {
                        (self.outlet)
                    }
                }
                (add_artist_dialog)
            }
        }
    }
}

impl Index {
    fn nav_bar(&self) -> Markup {
        let user = if let Some(user) = self.user.as_ref() {
            let profile_url = format!("/users/{}", user.username);
            html! {
                li {
                    details.dropdown {
                        summary {
                            (user.handle())
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
    // li { button command="show-modal" commandfor="create_event_form" { "Create Event" } }
    // li {
    //     button command="show-modal" commandfor=(ADD_ARTIST_DIALOG_ID) {
    //         span.phone { "Create Artist" }
    //     }
    // }
    // aside {
    //     .flex.column.h100 {
    //         menu.grow {
    //             li { a href="/" { "Home" } }
    //             li { button command="show-modal" commandfor="create_event_form" { "Create Event" } }
    //         }
    //         div.flex.wrap {
    //             a href="/help" { "Help" }
    //             a href="/imprint" { "Imprint" }
    //             a href="/privacy" { "Privacy Policy" }
    //             a href="/contact" { "Contact" }
    //         }
    //     }
    // }

    fn scripts(&self) -> Markup {
        html! {
            script src="/assets/htmx.min.js" {}
        }
    }

    fn styles(&self) -> Markup {
        html! {
            link rel="stylesheet" href="/assets/mu.css";
            link rel="stylesheet" href="/assets/style.css";
            link rel="stylesheet" href="/assets/icons/regular/style.css";
            link rel="stylesheet" href="/assets/icons/fill/style.css";
            link rel="stylesheet" href="/assets/fonts/index.css";
        }
    }
}

impl Render for IndexRoute {
    fn render(&self) -> Markup {
        match self {
            IndexRoute::Home(list) => list.render(),
            IndexRoute::Event(event) => event.render(),
            IndexRoute::CreateEvent(create) => create.render(),
            IndexRoute::SignUp => SignUp {}.render(),
            IndexRoute::Login => SignIn {}.render(),
        }
    }
}
