use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Tag {
    label: String,
    #[builder(default)]
    index: Option<u8>,
}

impl Render for Tag {
    fn render(&self) -> Markup {
        html! {
            span class=(self.index.map_or_else(|| "tag".to_string(), |i| format!("tag tag-{i}"))) {
                (self.label)
            }
        }
    }
}
