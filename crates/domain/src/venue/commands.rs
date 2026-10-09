use std::ops::Deref as _;

use eventwise::command::{CommandError, Handle};
use uuid::Uuid;

use crate::{
    user::models::user::User,
    venue::{
        events::VenueSuggested,
        models::venue::{
            Venue, VenueAddress, VenueCity, VenueCountry, VenueError, VenueId, VenueName,
            VenuePostalCode,
        },
    },
};

pub struct SuggestVenue {
    pub id: Uuid,
    pub user: User,
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub capacity: Option<i32>,
}

impl Handle<SuggestVenue> for Venue {
    fn create(cmd: SuggestVenue) -> Result<((), Vec<Self::Event>), CommandError<Self::Error>> {
        let name = VenueName::try_new(cmd.name)
            .map_err(|_| CommandError::Domain(VenueError::InvalidName))?;
        let address = VenueAddress::try_new(cmd.address)
            .map_err(|_| CommandError::Domain(VenueError::InvalidAddress))?;
        let city = VenueCity::try_new(cmd.city)
            .map_err(|_| CommandError::Domain(VenueError::InvalidCity))?;
        let country = VenueCountry::try_new(cmd.country)
            .map_err(|_| CommandError::Domain(VenueError::InvalidCountry))?;
        let postal_code = VenuePostalCode::try_new(cmd.postal_code)
            .map_err(|_| CommandError::Domain(VenueError::InvalidPostalCode))?;

        let suggested_venue = VenueSuggested {
            id: cmd.id,
            name: name.into_inner(),
            address: address.into_inner(),
            city: city.into_inner(),
            country: country.into_inner(),
            postal_code: postal_code.into_inner(),
            capacity: cmd.capacity,
            user_id: *cmd.user.id().deref(),
        };

        Ok(((), vec![Self::Event::VenueSuggested(suggested_venue)]))
    }
}
