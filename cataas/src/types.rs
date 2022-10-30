use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename = "lowercase")]
pub enum ImageType {
    Small,
    Medium,
    Square,
    Original,
}

#[derive(Debug, Error)]
#[error("cannot parse an invalid ImageType")]
pub struct ParseImageTypeError(());

impl FromStr for ImageType {
    type Err = ParseImageTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "small" | "sm" => Self::Small,
            "medium" | "md" => Self::Medium,
            "square" | "sq" => Self::Square,
            "original" | "or" => Self::Original,
            _ => return Err(ParseImageTypeError(())),
        })
    }
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

#[derive(Debug, Error)]
#[error("cannot parse an invalid Filter")]
pub struct ParseFilterError(());

impl FromStr for Filter {
    type Err = ParseFilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "blur" => Self::Blur,
            "mono" => Self::Mono,
            "sepia" => Self::Sepia,
            "negative" => Self::Negative,
            "paint" => Self::Paint,
            "pixel" => Self::Pixel,
            _ => return Err(ParseFilterError(())),
        })
    }
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
