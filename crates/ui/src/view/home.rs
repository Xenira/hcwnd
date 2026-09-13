use std::{borrow::Cow, fmt::Display};

use api::{event::Event, UiState};
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::{
    atom::{input::Search, select::Select},
    component::EventCard,
    view::View,
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Home {
    #[builder(default)]
    events: Vec<Event>,
}

impl View for Home {
    fn render(&self, state: &UiState) -> maud::Markup {
        let genre_select = Select::builder()
            .name("genre")
            .options(vec![(
                "*".to_string(),
                t!("app.search.genre.all", locale = &state.locale).to_string(),
            )])
            .build();
        let sort_select = Select::builder()
            .name("sort")
            .options(vec![
                (
                    (SortOption::Popularity as u8).to_string(),
                    t!("app.search.sort.popularity", locale = &state.locale).to_string(),
                ),
                (
                    (SortOption::Date as u8).to_string(),
                    t!("app.search.sort.date", locale = &state.locale).to_string(),
                ),
            ])
            .value((SortOption::Popularity as u8).to_string())
            .build();

        let search = Search::builder()
            .name("q")
            .placeholder(t!("home.search.placeholder", locale = &state.locale))
            .build();

        html! {
            div #home {
                header {
                    img src="https://hardcore.localhost:8443/image/ck4Rfi6qHGlka-X0JAqusANAg-r-fEldIUtoZqx13wQ/rs:auto:1158:650/aHR0cHM6Ly93d3cuc3luZGljYXRlLWZlc3RpdmFsLmRlLzE3NzMyMS8xNzYyNDMzMjY3LXN5bjI2X3Byb2ZpbGVfMTAwMHgxMDAwcHgtbWluLnBuZw.png" alt="Header image" {}
                    hgroup {
                        h1 { (t!("home.title", locale = &state.locale)) }
                        p { (t!("home.subtitle", locale = &state.locale)) }
                    }
                    (search.render())
                }
                section {
                    #search-header {
                        h2 { (t!("home.search.title", locale = &state.locale)) }
                        search {
                            (genre_select.render())
                            (sort_select.render())
                        }
                    }
                    #search-results {
                        @for event in &self.events {
                            (EventCard::from(event).render())
                        }
                    }
                }
            }
        }
    }

    fn title(&self, state: &UiState) -> Cow<'_, str> {
        t!("app.title.home", locale = &state.locale)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SortOption {
    Popularity = 0,
    Date = 1,
}
