use anyhow::Context;
use imgproxy::{
    Gravity, GravityOptionsBuilder, ImageUrl, ProcessingOption, ResizeMode, ResizingOptionsBuilder,
    SignedUrlRepo,
};
use url::Url;

use crate::domain::event::models::{act::Act, event::Event};

#[derive(Clone)]
pub struct ActMapper {
    image_signer: SignedUrlRepo,
}

impl ActMapper {
    pub(crate) fn new(image_signer: SignedUrlRepo) -> Self {
        Self { image_signer }
    }

    pub fn map_act(&self, act: &Act) -> anyhow::Result<api::act::Act> {
        let img_url = act
            .act_img()
            .map(|img| {
                self.image_signer
                    .get(&self.card_image(img.clone().into_inner())?)
                    .context("Failed to sign image URL")
            })
            .transpose()?;

        let event = api::act::Act {
            id: act.id().clone().into_inner(),
            name: act.name().clone().into_inner(),
            image_url: img_url,
        };

        Ok(event)
    }

    fn card_image(&self, url: Url) -> anyhow::Result<ImageUrl> {
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
