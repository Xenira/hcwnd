use async_trait::async_trait;

use crate::domain::{
    artist::models::artist::{
        Artist, ArtistId, CreateArtistError, CreateArtistRequest, GetArtistError,
        SearchArtistsError, SearchArtistsQuery,
    },
    user::models::user::UserId,
};

#[async_trait]
pub trait ArtistService: Sync + Send {
    async fn create_artist(
        &self,
        req: &CreateArtistRequest,
        author_id: &UserId,
    ) -> Result<Artist, CreateArtistError>;

    async fn artists_by_act(&self, act_name: &str) -> Result<Vec<Artist>, SearchArtistsError>;

    async fn get_artist_by_id(&self, id: &ArtistId) -> Result<Artist, GetArtistError>;
}

#[async_trait]
pub trait ArtistRepository: Clone + Sync + Send + 'static {
    async fn create_artist(
        &self,
        req: &CreateArtistRequest,
        author_id: &UserId,
    ) -> Result<Artist, CreateArtistError>;

    async fn search_artist(
        &self,
        query: &SearchArtistsQuery,
    ) -> Result<Vec<Artist>, SearchArtistsError>;

    async fn get_artist_by_id(&self, id: &ArtistId) -> Result<Artist, GetArtistError>;
}
