use async_trait::async_trait;

use crate::domain::{
    artist::{
        models::artist::{
            Artist, ArtistId, CreateArtistError, CreateArtistRequest, GetArtistError,
            SearchArtistsError, SearchArtistsQuery,
        },
        ports::{ArtistRepository, ArtistService},
    },
    user::models::user::UserId,
};

const ACT_SEPERATORS: &[&str] = &[" b2b ", " vs ", " x ", " vs. ", " b2b. "];
const SUFFIXES: &[&str] = &[" live"];

#[derive(Debug, Clone)]
pub struct Service<AR>
where
    AR: ArtistRepository,
{
    pub artist_repository: AR,
}

impl<AR> Service<AR>
where
    AR: ArtistRepository,
{
    pub fn new(artist_repository: AR) -> Self {
        Self { artist_repository }
    }
}

#[async_trait]
impl<AR> ArtistService for Service<AR>
where
    AR: ArtistRepository + Send + Sync,
{
    async fn create_artist(
        &self,
        req: &CreateArtistRequest,
        author_id: &UserId,
    ) -> Result<Artist, CreateArtistError> {
        self.artist_repository.create_artist(req, author_id).await
    }

    async fn artists_by_act(&self, act_name: &str) -> Result<Vec<Artist>, SearchArtistsError> {
        let act_names = ACT_SEPERATORS
            .iter()
            .fold(vec![act_name.to_lowercase()], |acc, sep| {
                acc.into_iter()
                    .flat_map(|name| {
                        name.split(sep)
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<_>>()
                    })
                    .collect()
            });

        let mut artists = Vec::new();
        for query in act_names.into_iter().map(|name| {
            if let Some(suffix) = SUFFIXES.iter().find(|s| name.ends_with(*s)) {
                SearchArtistsQuery::new(Some(name.trim_end_matches(suffix).to_string()))
            } else {
                SearchArtistsQuery::new(Some(name))
            }
        }) {
            artists.append(&mut self.artist_repository.search_artist(&query).await?);
        }

        Ok(artists)
    }

    async fn get_artist_by_id(&self, id: &ArtistId) -> Result<Artist, GetArtistError> {
        self.artist_repository.get_artist_by_id(id).await
    }
}
