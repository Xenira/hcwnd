use imgproxy::{
    Gravity, GravityOptionsBuilder, ImageUrl, ProcessingOption, ResizeMode, ResizingOptionsBuilder,
    SignedUrlRepo,
};
use url::Url;

use crate::{domain::event::models::event::Event, inbound::http::mapper::act::ActMapper};

#[derive(Clone)]
pub struct EventMapper {
    image_signer: SignedUrlRepo,
    act_mapper: ActMapper,
}

impl EventMapper {
    pub(crate) fn new(image_signer: SignedUrlRepo) -> Self {
        Self {
            act_mapper: ActMapper::new(image_signer.clone()),
            image_signer,
        }
    }

    pub fn map_event(&self, event: &Event) -> anyhow::Result<api::event::Event> {
        let image_url = self
            .image_signer
            .get(&self.card_image(event.image_url().clone().into_inner())?)?;

        let event = api::event::Event {
            id: event.id().clone().into_inner(),
            name: event.name().clone().into_inner(),
            description: event.description().clone().into_inner(),
            start_date: event.start_date().date(),
            start_time: Some(event.start_date().time()),
            end_date: event.end_date().date(),
            end_time: Some(event.end_date().time()),
            website_url: event.website_url().clone().into_inner(),
            image_url,
            state: api::event::EventState::Online,
            acts: event
                .acts()
                .iter()
                .map(|act| self.act_mapper.map_act(act))
                .collect::<anyhow::Result<Vec<_>>>()?,
            stages: vec![], // TODO: Map stages when available
        };

        Ok(event)
    }

    fn card_image(&self, url: Url) -> anyhow::Result<ImageUrl> {
        Ok(ImageUrl::new(url)
            .with_option(ProcessingOption::Resize(
                ResizingOptionsBuilder::default()
                    .width(1158)
                    .height(650)
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
