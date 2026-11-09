use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Image {
    src: String,
    alt: String,
    #[builder(default = true)]
    lazy: bool,
    #[builder(default)]
    format: ImageFormat,
}

#[derive(Default, Clone, Copy)]
pub enum ImageFormat {
    #[default]
    Square,
    Banner,
    Avatar,
}

impl Render for Image {
    fn render(&self) -> Markup {
        html! {
            img
                src=(self.src)
                alt=(self.alt)
                lazy[self.lazy]
                class=(match self.format {
                    ImageFormat::Square => "square",
                    ImageFormat::Banner => "banner",
                    ImageFormat::Avatar => "avatar",
                })
            {}
        }
    }
}
