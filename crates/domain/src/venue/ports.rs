use async_trait::async_trait;

use crate::venue::{
    commands::SuggestVenue,
    models::venue::{Venue, VenueError, VenueId, VenueProjection},
};

#[async_trait]
pub trait VenueService: Send + Sync + 'static {
    async fn suggest(&self, req: SuggestVenue) -> Result<Venue, VenueError>;
    async fn get_by_id(&self, id: &VenueId) -> Result<Venue, VenueError>;
}

#[async_trait]
pub trait VenueQuery: Send + Sync + 'static {
    async fn search(&self, query: &str) -> Result<Vec<VenueProjection>, VenueError>;
}
