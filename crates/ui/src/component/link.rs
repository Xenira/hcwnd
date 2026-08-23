use maud::{Markup, html};

pub fn link(href: &str, target: Option<&str>, content: &Markup) -> Markup {
    html! {
        a href=(href)
            hx-target=(target.unwrap_or("#main"))
            hx-push-url="true"
            hx-boost="true"
        {
            (content)
        }
    }
}
