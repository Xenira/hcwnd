use anyhow::Context;
use imgproxy::{
    Gravity, GravityOptionsBuilder, ImageUrl, ProcessingOption, ResizeMode, ResizingOptionsBuilder,
    SignedUrlRepo,
};
use url::Url;

use crate::{
    domain::{artist::models::artist::Artist, event::models::event::Event},
    inbound::http::mapper::act::ActMapper,
};

#[derive(Clone)]
pub struct ArtistMapper {
    image_signer: SignedUrlRepo,
}

impl ArtistMapper {
    pub(crate) fn new(image_signer: SignedUrlRepo) -> Self {
        Self { image_signer }
    }

    pub fn map_artist(&self, artist: &Artist) -> anyhow::Result<api::artist::Artist> {
        let image_card = artist
            .image_url()
            .map(|image| {
                self.image_signer
                    .get(&self.card_image(image)?)
                    .context("Failed to sign image URL")
            })
            .transpose()?;

        Ok(api::artist::Artist {
            id: artist.id().clone().into_inner(),
            name: artist.name().as_ref().to_string(),
            image_card,
        })
    }

    fn card_image(&self, url: &Url) -> anyhow::Result<ImageUrl> {
        Ok(ImageUrl::new(url)
            .with_option(ProcessingOption::Resize(
                ResizingOptionsBuilder::default()
                    .width(128)
                    .height(128)
                    .mode(ResizeMode::FillDown)
                    .build()?,
            ))
            .with_option(ProcessingOption::Gravity(
                GravityOptionsBuilder::default()
                    .gravity(Gravity::Center)
                    .build()?,
            )))
    }
}
