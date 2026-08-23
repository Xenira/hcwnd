use maud::{Markup, html};

pub fn link(href: &impl ToString, target: Option<&str>, content: &Markup) -> Markup {
    let href = href.to_string();
    let blank = if href.starts_with("http://") || href.starts_with("https://") {
        Some("_blank")
    } else {
        None
    };
    html! {
        a href=(href)
            hx-target=(target.unwrap_or("#main"))
            hx-push-url="true"
            hx-boost="true"
            target=[blank]
        {
            (content)
        }
    }
}
