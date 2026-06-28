use crate::domain::{
    artist::models::artist::{
        Artist, ArtistId, CreateArtistError, CreateArtistRequest, GetArtistError,
        SearchArtistsError, SearchArtistsQuery,
    },
    user::models::user::UserId,
};

pub trait ArtistService: Clone + Sync + Send + 'static {
    fn create_artist(
        &self,
        req: &CreateArtistRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<Artist, CreateArtistError>>;

    fn artists_by_act(
        &self,
        act_name: &str,
    ) -> impl Future<Output = Result<Vec<Artist>, SearchArtistsError>>;

    fn get_artist_by_id(
        &self,
        id: &ArtistId,
    ) -> impl Future<Output = Result<Artist, GetArtistError>>;
}

pub trait ArtistRepository: Clone + Sync + Send + 'static {
    fn create_artist(
        &self,
        req: &CreateArtistRequest,
        author_id: &UserId,
    ) -> impl Future<Output = Result<Artist, CreateArtistError>>;

    fn search_artist(
        &self,
        query: &SearchArtistsQuery,
    ) -> impl Future<Output = Result<Vec<Artist>, SearchArtistsError>>;

    fn get_artist_by_id(
        &self,
        id: &ArtistId,
    ) -> impl Future<Output = Result<Artist, GetArtistError>>;
}
