use std::collections::HashMap;

use imgproxy::{ImageUrl, ProcessingOption, ResizeMode, ResizingOptionsBuilder, SignedUrlRepo};
use itertools::Itertools as _;
use ui::act::card::{ActCard, ActCardBuilder};
use uuid::Uuid;

use crate::{
    controller::ActRepoData, entity::{
        act::{Act, NewActBuilder},
        event::EventId, stage::{StageId, StageRepo},
    }, prelude::*,
};

pub async fn create_act(
    act_repo: &ActRepoData,
    event_id: EventId,
    user_id: Uuid,
    name: String,
    image_url: Option<String>,
) -> Result<Act> {
    let new_act = NewActBuilder::default()
        .id(Uuid::new_v4())
        .event_id(event_id)
        .name(name)
        .image_url(image_url)
        .user_id(user_id)
        .build()?;

    Ok(act_repo.create(new_act).await?)
}

pub async fn get_act_cards_for_event(
    act_repo: &ActRepoData,
    stages: &HashMap<StageId, String>,
    image_repo: &SignedUrlRepo,
    event_id: EventId,
) -> Result<Vec<ActCard>> {
    let acts = act_repo.acts_for_event(event_id).await?;

   get_act_cards(&acts, &stages, image_repo).await
}

pub async fn get_act_cards(
    acts: &[Act],
    stages: &HashMap<StageId, String>,
    image_repo: &SignedUrlRepo,
) -> Result<Vec<ActCard>> {
    acts.iter().map(|act| get_act_card_with_stages(act, &stages, image_repo)).try_collect()
}

fn get_act_card_with_stages(act: &Act, stages: &HashMap<StageId, String>, image_repo: &SignedUrlRepo) -> Result<ActCard> {
    let mut act_builder = ActCardBuilder::default()
        .id(act.id.into())
        .name(act.name.clone());

    if let Some(url) = &act.image_url && !url.is_empty() {
        let resize_options = ResizingOptionsBuilder::default()
            .mode(ResizeMode::Fill)
            .width(100)
            .height(100)
            .build()?;
        let image_url = ImageUrl::parse(url)?
            .with_option(ProcessingOption::Resize(resize_options));
        act_builder =  act_builder.image_url(Some(image_repo.get(&image_url)?));
    }

    if let Some(stage_id) = act.stage_id {
        act_builder = act_builder.stage(stages.get(&stage_id).cloned());
    }

    // FIXME: Unwrap
    Ok(act_builder.build().unwrap())
}
