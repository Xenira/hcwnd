use domain::festival::models::festival::Festival;
use imgproxy::{
    Gravity, GravityOptionsBuilder, ImageUrl, ProcessingOption, ResizeMode, ResizingOptionsBuilder,
    SignedUrlRepo,
};
use url::Url;

use crate::inbound::http::mapper::act::ActMapper;

#[derive(Clone)]
pub struct FestivalMapper {
    image_signer: SignedUrlRepo,
    act_mapper: ActMapper,
}

impl FestivalMapper {
    pub(crate) fn new(image_signer: SignedUrlRepo) -> Self {
        Self {
            act_mapper: ActMapper::new(image_signer.clone()),
            image_signer,
        }
    }

    pub fn map_festival(&self, festival: &Festival) -> anyhow::Result<api::event::Event> {
        let image_url = self
            .image_signer
            .get(&self.card_image(festival.image_url().as_ref())?)?;

        let festival = api::event::Event {
            id: festival.id().clone().into_inner(),
            name: festival.name().clone().into_inner(),
            description: festival.description().clone().into_inner(),
            start_date: festival.start_date(),
            start_time: festival.days().first().0,
            end_date: festival.end_date(),
            end_time: festival.days().last().1,
            website_url: festival.website_url().clone().into_inner(),
            image_url,
            state: api::event::EventState::Online,
            acts: festival
                .acts()
                .iter()
                .map(|act| self.act_mapper.map_act(act))
                .collect::<anyhow::Result<Vec<_>>>()?,
            stages: vec![], // TODO: Map stages when available
        };

        Ok(festival)
    }

    fn card_image(&self, url: &Url) -> anyhow::Result<ImageUrl> {
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
