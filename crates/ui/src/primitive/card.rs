use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;

use crate::component::{Icons, icon};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(fallback_suffix = "_opt"))))]
pub struct Card {
    #[builder(default)]
    header: Option<Markup>,
    #[builder(default)]
    body: Option<Markup>,
    #[builder(default)]
    footer: Option<Markup>,
    #[builder(default)]
    image: Option<String>,
}

impl Render for Card {
    fn render(&self) -> Markup {
        html! {
            article {
                @if let Some(image) = &self.image {
                    img src=(image) {}
                }
                section {
                    @if let Some(header) = &self.header {
                        header {
                            (header)
                        }
                    }
                    @if let Some(body) = &self.body {
                        (body)
                    }
                    @if let Some(footer) = &self.footer {
                        footer {
                            (footer)
                        }
                    }
                }
            }
        }
    }
}
