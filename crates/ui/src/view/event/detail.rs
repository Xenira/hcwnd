use api::{event::Event, UiState};
use maud::{html, Markup};

use crate::{index, view::event::View};

#[must_use]
pub fn full_page(state: &UiState, event: &Event) -> Markup {
    let title = t!(
        "event.detail.details.title",
        locale = &state.locale,
        event_name = &event.name
    );

    index::full_page(state, title, render(state, event))
}

#[must_use]
pub fn render(state: &UiState, event: &Event) -> Markup {
    html! {
        #event {
            (super::nav_bar(state, event.id, View::Detail))
            img src=(event.image_url) alt=(event.name);
            section.hero.hero-primary {
                div.container {
                    h1 { (event.name) (event.description) }
                }
            }
        }
    }
}
// impl maud::Render for EventDetails {
//     fn render(&self) -> maud::Markup {
//         maud::html! {
//             h1 { (self.title) }
//             img src=(self.image_url) alt=(self.title);
//             (self.overview())
//             (self.about())
//         }
//     }
// }
//
// impl EventDetails {
//     fn overview(&self) -> maud::Markup {
//         maud::html! {
//             section {
//                 header {
//                     h2 { "Overview" }
//                 }
//                 (icon(&Icons::Date, None)) (self.start_date.format("%d.%m.%Y").to_string())
//             }
//         }
//     }
//
//     fn about(&self) -> maud::Markup {
//         maud::html! {
//             section {
//                 header {
//                     h2 { "About" }
//                 }
//                 p { (self.description) }
//             }
//         }
//     }
// }
