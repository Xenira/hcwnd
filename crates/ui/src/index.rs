use std::fmt::Display;

use api::{user::User, UiState};
use maud::{html, Markup, Render, DOCTYPE};

use crate::{
    atom::nav::{Nav, NavEntry, NavEntryAlignment, NavEntryDropdown, NavEntrySimple},
    component::menu_item,
    user::{self},
};

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
                (nav_bar(&state))
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

fn nav_bar(state: &UiState) -> Markup {
    let mut entries = if let Some(user) = &state.user {
        vec![NavEntryDropdown::builder()
            .label(&user.name)
            .entries(vec![
                NavEntrySimple::builder()
                    .label(t!("app.menu.user.profile", locale = &state.locale))
                    .href(format!("/user/{}", user.id))
                    .build()
                    .into(),
                NavEntrySimple::builder()
                    .label(t!("app.menu.user.logout", locale = &state.locale))
                    .href("/logout")
                    .build()
                    .into(),
            ])
            .alignment(NavEntryAlignment::Right)
            .build()
            .into()]
    } else {
        vec![
            NavEntrySimple::builder()
                .label(t!("app.menu.user.login", locale = &state.locale))
                .href("/login")
                .alignment(NavEntryAlignment::Right)
                .build()
                .into(),
            NavEntrySimple::builder()
                .label(t!("app.menu.user.sign_up", locale = &state.locale))
                .href("/signup")
                .alignment(NavEntryAlignment::Right)
                .build()
                .into(),
        ]
    };
    entries.push(
        NavEntrySimple::builder()
            .label(t!("app.name", locale = &state.locale))
            .href("/")
            .build()
            .into(),
    );
    entries.push(
        NavEntrySimple::builder()
            .label(t!("app.menu.artists", locale = &state.locale))
            .href(api::routes::ARTIST_ROUTE)
            .build()
            .into(),
    );

    Nav::builder().entries(entries).build().render()
}

fn scripts() -> Markup {
    html! {
        script src="/assets/htmx.min.js" {}
    }
}

fn styles() -> Markup {
    html! {
        link rel="stylesheet" href="/assets/style.css";
    }
}
