use maud::{Markup, Render, html};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::atom::tombstone::Tombstoneable;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct Lazy<T, ID = Uuid>
where
    T: Tombstoneable + Fetchable<ID>,
{
    id: ID,
    #[builder(default)]
    mode: LazyMode,
    #[builder(default)]
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Default, Clone, Copy)]
pub enum LazyMode {
    #[default]
    Revealed,
    Load,
}

impl<T, ID> Render for Lazy<T, ID>
where
    T: Tombstoneable + Fetchable<ID>,
{
    fn render(&self) -> Markup {
        html! {
            div
                hx-trigger=(match self.mode {
                    LazyMode::Load => "load",
                    LazyMode::Revealed => "revealed",
                })
                hx-get=(T::fetch_url(&self.id))
                hx-swap="outerHTML"
            {
                (T::tombstone())
            }
        }
    }
}

pub trait Fetchable<ID> {
    fn fetch_url(id: &ID) -> String;
}
