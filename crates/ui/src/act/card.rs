use derive_builder::Builder;
use maud::{html, Markup, Render};
use uuid::Uuid;

use crate::component::{icon, Icons};

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct ActCard {
    pub id: Uuid,
    pub name: String,
    #[builder(default)]
    pub stage: Option<String>,
    #[builder(default)]
    pub image_url: Option<String>,
    pub artists: Vec<String>,
}

impl Render for ActCard {
    fn render(&self) -> Markup {
        let image = if let Some(image_url) = &self.image_url {
            html! { img src=(image_url) alt=(self.name) loading="lazy" {} }
        } else {
            html! { (icon(&Icons::ActImagePlaceholder, None)) }
        };

        let stage = if let Some(stage) = &self.stage {
            html! {
              div title=(format!("Stage: {stage}")) {
                (icon(&Icons::Stage, None))
                (stage)
              }
            }
        } else {
            html! {}
        };

        html! {
            article.center {
                aside {
                    (image)
                }
                div {
                    h2 { (self.name) }
                    (stage)
                    @for artist in &self.artists {
                        span { (artist) }
                    }
                }
            }
        }
    }
}
