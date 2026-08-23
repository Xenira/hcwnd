use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ArtistCreateForm {
    pub name: Option<String>,
    pub image_url: Option<Url>,
    pub website_url: Option<Url>,
    pub genres: IndexSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArtistCreateSubmitForm {
    pub name: String,
    pub image_url: Option<Url>,
    pub website_url: Option<Url>,
    pub genres: Option<IndexSet<String>>,
}
