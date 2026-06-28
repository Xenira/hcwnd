use ui::index::{Index, IndexBuilder, IndexRoute};

pub mod act;
pub mod event;
mod timetable;

pub use timetable::*;

pub fn index(title: &str, route: IndexRoute) -> Index {
    let index = IndexBuilder::default()
        .title(format!("{title} | Hardcore Will Never Diy"))
        .outlet(route)
        .build()
        .expect("Failed to build index page");

    index
}
