use std::ops::Not as _;

use chrono::{DateTime, Local};
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::{
    atom::{card::Card, tag::Tag},
    component::{icon, Icons},
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct EventCard {
    id: Uuid,
    name: String,
    location: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    image: String,
    #[builder(default)]
    saved: bool,
    #[builder(default)]
    genres: Vec<String>,
}

impl Render for EventCard {
    fn render(&self) -> Markup {
        html! {
            .event-card {
                (Card::builder()
                    .image(&self.image)
                    .header(html! {
                        strong {
                            (time_range(&self.start_time, &self.end_time))
                        }
                        hgroup {
                            h3 { (self.name) }
                            p { (self.location) }
                        }
                    })
                    .body_opt(self.genres.is_empty().not().then(|| {
                        html! {
                            .flair {
                                (Tag::builder()
                                    .label("Preview")
                                    .build())
                            }
                            div {
                                @for (i, genre) in self.genres.iter().enumerate().take(5) {
                                    (Tag::builder()
                                        .label(genre.clone())
                                        .index(i as u8)
                                        .build())
                                }
                            }
                        }
                    }))
                    .footer(html! {
                        (icon(&Icons::OpenCard, None))
                    })
                    .build()
                    .render())
            }
        }
    }
}

fn time_range(start: &DateTime<Local>, end: &DateTime<Local>) -> String {
    if (*end - *start).num_days() < 1 {
        format!("{}", start.format("%d. %h %Y"))
    } else {
        format!("{} - {}", start.format("%d. %h"), end.format("%d. %h %Y"))
    }
}
