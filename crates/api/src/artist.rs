use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub image_card: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ArtistCreateForm {
    pub name: Option<String>,
    pub image_url: Option<Url>,
    pub website_url: Option<Url>,
    pub genres: IndexSet<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArtistCreateSubmitForm {
    pub name: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub image_url: Option<Url>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub website_url: Option<Url>,
    pub genres: Option<IndexSet<String>>,
}
