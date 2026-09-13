use api::{event::Event, UiState};
use maud::{html, Markup};

use crate::{index, view::event::View};

#[must_use]
pub fn full_page(state: &UiState, event: &Event) -> Markup {
    let title = t!(
        "event.detail.details.title",
        locale = &state.locale,
        name = &event.name
    );

    super::full_page(
        state,
        &title,
        event,
        View::Detail,
        detail_view(state, event),
    )
}

#[must_use]
pub fn render(state: &UiState, event: &Event) -> Markup {
    super::render(state, event, View::Detail, detail_view(state, event))
}

fn detail_view(state: &UiState, event: &Event) -> Markup {
    html! {
        h2 {
            (t!("event.detail.details.title", locale = &state.locale))
        }
        p {
            (event.description)
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
