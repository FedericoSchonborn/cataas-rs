use reqwest::Method;
use serde::Serialize;

use crate::{
    types::{self, Filter, ImageType},
    Client, Error,
};

use super::Format;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub(super) struct Params {
    #[serde(rename = "type")]
    image_type: Option<ImageType>,
    filter: Option<Filter>,
    width: Option<usize>,
    height: Option<usize>,
    json: Option<bool>,
}

pub struct Cat<'a> {
    client: &'a Client,
    format: Format,
    params: Params,
}

impl<'a> Cat<'a> {
    #[must_use]
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            format: Format::Jpeg,
            params: Params {
                json: Some(true),
                ..Default::default()
            },
        }
    }

    pub fn gif(&mut self) -> &mut Self {
        self.format = Format::Gif;
        self
    }

    pub fn tag(&mut self, tag: String) -> &mut Self {
        self.format = Format::TaggedJpeg(tag);
        self
    }

    pub fn image_type(&mut self, image_type: ImageType) -> &mut Self {
        self.params.image_type = Some(image_type);
        self
    }

    pub fn filter(&mut self, filter: Filter) -> &mut Self {
        self.params.filter = Some(filter);
        self
    }

    pub fn width(&mut self, width: usize) -> &mut Self {
        self.params.width = Some(width);
        self
    }

    pub fn height(&mut self, height: usize) -> &mut Self {
        self.params.height = Some(height);
        self
    }

    pub async fn send(&self) -> Result<types::Cat, Error> {
        let mut path = String::from("/cat");
        match self.format {
            Format::Jpeg => {}
            Format::TaggedJpeg(ref tag) => {
                path.push('/');
                path.push_str(tag);
            }
            Format::Gif => path.push_str("/gif"),
        }

        self.client.request(Method::GET, &path, &self.params).await
    }
}