use eventwise::aggregate::{Apply, Create};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    suggestion::{events::impl_suggestion_events, SuggestionState},
    user::models::user::UserId,
    venue::models::venue::*,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VenueSuggested {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub capacity: Option<i32>,
    pub user_id: Uuid,
}

impl Create<VenueSuggested> for Venue {
    fn bootstrap(event: VenueSuggested) -> Self {
        let name = unsafe { VenueName::new_unchecked(event.name) };
        let address = unsafe { VenueAddress::new_unchecked(event.address) };
        let city = unsafe { VenueCity::new_unchecked(event.city) };
        let country = unsafe { VenueCountry::new_unchecked(event.country) };
        let postal_code = unsafe { VenuePostalCode::new_unchecked(event.postal_code) };

        Venue::builder()
            .id(VenueId::new(event.id))
            .name(name)
            .address(address)
            .city(city)
            .country(country)
            .postal_code(postal_code)
            .capacity(event.capacity)
            .state(SuggestionState::Suggested {
                votes: vec![(UserId::new(event.user_id), 1)].into_iter().collect(),
                user_id: UserId::new(event.user_id),
            })
            .build()
    }
}

impl_suggestion_events!(Venue, state_mut);
