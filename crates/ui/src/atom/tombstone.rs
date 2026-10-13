use maud::{Markup, Render, html};

pub enum Tombstone {
    TextShort,
    TextMedium,
    TextLong,
    TextFullWidth,
    TextParagraph,
    Heading,
    AvatarSmall,
    AvatarLarge,
    Icon,
    Chip,
    Button,
    ImageSquare,
    ImageBanner,
}

impl Render for Tombstone {
    fn render(&self) -> Markup {
        let class = match self {
            Tombstone::TextShort => "text short",
            Tombstone::TextMedium => "text medium",
            Tombstone::TextLong => "text long",
            Tombstone::TextFullWidth => "text full",
            Tombstone::TextParagraph => {
                return html! {
                    (Tombstone::TextFullWidth)
                    (Tombstone::TextFullWidth)
                    (Tombstone::TextFullWidth)
                    (Tombstone::TextShort)
                };
            }
            Tombstone::Heading => "heading",
            Tombstone::AvatarSmall => "avatar-small",
            Tombstone::AvatarLarge => "avatar-large",
            Tombstone::Icon => "icon",
            Tombstone::Chip => "chip",
            Tombstone::Button => "button",
            Tombstone::ImageSquare => "image-square",
            Tombstone::ImageBanner => "image-banner",
        };

        html! {
            .tombstone.(class) {}
        }
    }
}

pub trait Tombstoneable {
    fn tombstone() -> impl Render;
}
