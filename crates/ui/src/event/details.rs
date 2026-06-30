use chrono::NaiveDate;
use derive_builder::Builder;
use uuid::Uuid;

use crate::component::{Icons, icon};

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct EventDetails {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub start_date: NaiveDate,
}

impl maud::Render for EventDetails {
    fn render(&self) -> maud::Markup {
        maud::html! {
            h1 { (self.title) }
            img src=(self.image_url) alt=(self.title);
            (self.overview())
            (self.about())
        }
    }
}

impl EventDetails {
    fn overview(&self) -> maud::Markup {
        maud::html! {
            section {
                header {
                    h2 { "Overview" }
                }
                (icon(&Icons::Date, None)) (self.start_date.format("%d.%m.%Y").to_string())
            }
        }
    }

    fn about(&self) -> maud::Markup {
        maud::html! {
            section {
                header {
                    h2 { "About" }
                }
                p { (self.description) }
            }
        }
    }
}
