use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
}
