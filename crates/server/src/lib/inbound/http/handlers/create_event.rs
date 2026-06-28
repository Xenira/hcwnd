use actix_web::web;

use crate::domain::{
    artist::ports::ArtistService, event::ports::EventService, user::ports::UserService,
};

pub mod confirm_step;
pub mod days_step;
pub mod details_step;
pub mod name_step;
pub mod stages_step;

pub fn configure<ES, AS, US>(cfg: &mut web::ServiceConfig)
where
    ES: EventService + 'static,
    AS: ArtistService + 'static,
    US: UserService + 'static,
{
    cfg.service(
        web::scope(ui::event::create::name_step::BASE_ROUTE)
            .configure(name_step::configure::<ES, AS, US>),
    )
    .service(
        web::scope(ui::event::create::days_step::BASE_ROUTE)
            .configure(days_step::configure::<ES, AS, US>),
    )
    .service(
        web::scope(ui::event::create::details_step::BASE_ROUTE)
            .configure(details_step::configure::<ES, AS, US>),
    )
    .service(
        web::scope(ui::event::create::stage_step::BASE_ROUTE)
            .configure(stages_step::configure::<ES, AS, US>),
    )
    .service(
        web::scope(ui::event::create::confirm_step::BASE_ROUTE)
            .configure(confirm_step::configure::<ES, AS, US>),
    );
}
