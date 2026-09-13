use std::borrow::Cow;

use api::UiState;
use maud::Render;

use crate::index;

pub mod artist;
pub mod event;

mod home;
pub use home::Home;

pub trait View {
    fn render(&self, state: &UiState) -> maud::Markup;
    fn title(&self, state: &UiState) -> Cow<'_, str>;

    fn full_page(&self, state: &UiState) -> maud::Markup {
        index::full_page(state, self.title(state), self.render(state))
    }
}
