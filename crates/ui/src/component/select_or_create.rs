use std::{borrow::Cow, fmt::Display};

use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::{
    component::Autocomplete,
    htmx::{HxMethod, HxSwap},
    primitive::button::{Button, ButtonClass, ButtonType},
};

pub enum SelectOrCreate {
    None {
        placeholder: Cow<'static, str>,
        search_url: String,
        form_url: String,
    },
    Selected(Markup),
    Create(Markup),
}

impl Render for SelectOrCreate {
    fn render(&self) -> Markup {
        match self {
            Self::None {
                placeholder,
                search_url,
                form_url,
            } => {
                let autocomplete = Autocomplete::builder()
                    .name(Uuid::new_v4().to_string())
                    .placeholder(placeholder.clone())
                    .search_url(search_url.clone())
                    .build();
                let new_button = Button::builder()
                    .label("Create new")
                    .class(ButtonClass::Link)
                    .hx_method(HxMethod::Get(form_url.clone()))
                    .hx_swap(HxSwap::OuterHtml)
                    .build();
                html! {
                    div {
                        (autocomplete)
                        (new_button)
                    }
                }
            }
            Self::Selected(selected) => html! { (selected) },
            Self::Create(create) => html! { (create) },
        }
    }
}
