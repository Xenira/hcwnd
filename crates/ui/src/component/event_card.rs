use std::ops::Not as _;

use api::event::Event;
use chrono::{DateTime, Local};
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::{
    atom::{
        button::{ButtonType, LinkButton},
        card::Card,
        tag::Tag,
    },
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
    #[builder(default)]
    flair: Option<String>,
    #[builder(setter(!strip_option,strip_bool))]
    non_interactive: bool,
}

impl Render for EventCard {
    fn render(&self) -> Markup {
        let details_button = LinkButton::builder()
            .label("")
            .icon(Icons::OpenCard)
            .href(format!("/event/{}", self.id))
            .button_type(ButtonType::Flat)
            .build();

        let footer = html! {
            @if !self.non_interactive {
                (details_button)
            }
        };

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
                            @if let Some(flair) = &self.flair {
                                .flair {
                                    (Tag::builder()
                                        .label(flair)
                                        .build())
                                }
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
                    .footer(footer)
                    .build()
                    .render())
            }
        }
    }
}

impl From<&Event> for EventCard {
    fn from(event: &Event) -> Self {
        Self::builder()
            .id(event.id)
            .name(&event.name)
            .location("TBD")
            .start_time(Local::now())
            .end_time(Local::now())
            .image(&event.image_url)
            .saved(false)
            .build()
    }
}

fn time_range(start: &DateTime<Local>, end: &DateTime<Local>) -> String {
    if (*end - *start).num_days() < 1 {
        format!("{}", start.format("%d. %h %Y"))
    } else {
        format!("{} - {}", start.format("%d. %h"), end.format("%d. %h %Y"))
    }
}
