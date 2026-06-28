use maud::{html, Markup};

use crate::component::{icon, Icons, Style};

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
            @if active {
                span.flex.center {
                    @if let Some(menu_icon) = &menu_icon {
                        (icon(menu_icon, Some(Style::Fill)))
                    }
                    span.phone[menu_icon.is_some()] { (title) }
                }
            } @else {
                a.flex.center
                    name=(key)
                    hx-get=(url)
                    hx-target=(target)
                    hx-push-url="true"
                    hx-swap="outerHTML"
                {
                    @if let Some(menu_icon) = &menu_icon {
                        (icon(menu_icon, None))
                    }
                    span.phone[menu_icon.is_some()] { (title) }
                }
            }
        }
    }
}
