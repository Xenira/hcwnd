use maud::{Markup, html};

use crate::component::{Icons, icon, link};

pub fn artist_card(state: &api::UiState, artist: &api::artist::Artist) -> Markup {
    html! {
        article.artist-card {
            aside {
            @if let Some(image_url) = &artist.image_card {
                img src=(image_url)
                    alt=(t!("artist.card.image.alt", locale = &state.locale, name = &artist.name))
                    width="128"
                    height="128"
                    lazy
                {}
            } @else {
                (icon(&Icons::ArtistImagePlaceholder, None))
            }
            }
            section {
                h3 {
                    (artist.name)
                }
                @if let Some(website) = &artist.website_url {
                    (link(
                        website,
                        None,
                        &icon(&Icons::Website, None),
                    ))
                }
            }
        }
    }
}
