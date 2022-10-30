use reqwest::Method;
use serde::Serialize;

use crate::{
    types::{self, Filter, ImageType},
    Client, ClientError,
};

use super::Format;

#[derive(Debug, Default, Serialize)]
struct Params {
    #[serde(rename = "type")]
    image_type: Option<ImageType>,
    filter: Option<Filter>,
    width: Option<usize>,
    height: Option<usize>,
    size: Option<usize>,
    color: Option<String>,
    json: Option<bool>,
}

/// Provides access to `/cat/says`, `/cat/:tag/says` and `/cat/gif/says` methods.
#[derive(Debug)]
pub struct Says<'a> {
    client: &'a Client,
    format: Format,
    text: String,
    params: Params,
}

impl<'a> Says<'a> {
    #[must_use]
    pub fn new(client: &'a Client, text: String) -> Self {
        Self {
            client,
            format: Format::Jpeg,
            text,
            params: Params {
                json: Some(true),
                ..Default::default()
            },
        }
    }

    pub fn with_gif(&mut self, gif: bool) -> &mut Self {
        if gif {
            self.format = Format::Gif;
        } else if let Format::Gif = self.format {
            self.format = Format::Jpeg;
        }

        self
    }

    pub fn gif(&mut self) -> &mut Self {
        self.with_gif(true)
    }

    pub fn with_tag<S>(&mut self, tag: Option<S>) -> &mut Self
    where
        S: Into<String>,
    {
        match tag {
            Some(value) => self.format = Format::TaggedJpeg(value.into()),
            None => self.format = Format::Jpeg,
        }

        self
    }

    pub fn tag<S>(&mut self, tag: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.with_tag(Some(tag))
    }

    pub fn with_image_type(&mut self, image_type: Option<ImageType>) -> &mut Self {
        self.params.image_type = image_type;
        self
    }

    pub fn image_type(&mut self, image_type: ImageType) -> &mut Self {
        self.with_image_type(Some(image_type))
    }

    pub fn with_filter(&mut self, filter: Option<Filter>) -> &mut Self {
        self.params.filter = filter;
        self
    }

    pub fn filter(&mut self, filter: Filter) -> &mut Self {
        self.with_filter(Some(filter))
    }

    pub fn with_width(&mut self, width: Option<usize>) -> &mut Self {
        self.params.width = width;
        self
    }

    pub fn width(&mut self, width: usize) -> &mut Self {
        self.with_width(Some(width))
    }

    pub fn with_height(&mut self, height: Option<usize>) -> &mut Self {
        self.params.height = height;
        self
    }

    pub fn height(&mut self, height: usize) -> &mut Self {
        self.with_height(Some(height))
    }

    pub fn with_size(&mut self, size: Option<usize>) -> &mut Self {
        self.params.size = size;
        self
    }

    pub fn size(&mut self, size: usize) -> &mut Self {
        self.with_size(Some(size))
    }

    pub fn with_color<S>(&mut self, color: Option<S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.params.color = color.map(Into::into);
        self
    }

    pub fn color<S>(&mut self, color: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.with_color(Some(color))
    }

    pub async fn send(&self) -> Result<types::Cat, ClientError> {
        let mut path = String::from("/cat");
        match self.format {
            Format::Jpeg => {}
            Format::TaggedJpeg(ref tag) => {
                path.push('/');
                path.push_str(tag);
            }
            Format::Gif => path.push_str("/gif"),
        }

        path.push_str("/says/");
        path.push_str(&self.text);
        self.client.request(Method::GET, &path, &self.params).await
    }
}
