use maud::{html, Markup};

use crate::component::{icon, Icons, Style};

#[must_use]
pub fn menu_item(
    title: &str,
    menu_icon: Option<Icons>,
    url: &str,
    target: &str,
    key: &str,
    active: bool,
) -> Markup {
    html! {
        li.active[active] title=(title) {
            a
                name=(key)
                href=(url)
                hx-boost="true"
                hx-target=(target)
                hx-push-url="true"
                hx-swap="outerHTML"
                aria-current[active]
            {
                @if let Some(menu_icon) = &menu_icon {
                    (icon(menu_icon, None))
                }
                span.phone[menu_icon.is_some()] { (title) }
            }
        }
    }
}
