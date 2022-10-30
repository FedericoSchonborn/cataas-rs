use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename = "lowercase")]
pub enum ImageType {
    Small,
    Medium,
    Square,
    Original,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename = "lowercase")]
pub enum Filter {
    Blur,
    Mono,
    Sepia,
    Negative,
    Paint,
    Pixel,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Cat {
    pub tags: Option<Vec<String>>,
    #[serde(rename = "createdAt", with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(rename = "updatedAt", with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub validated: bool,
    pub owner: String,
    pub file: String,
    pub mimetype: String,
    pub size: Option<usize>,
    #[serde(rename = "_id")]
    pub id: String,
    pub url: String,
}
