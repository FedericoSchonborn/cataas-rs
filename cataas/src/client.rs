//! Cat as a Service API client.

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::methods::{Cat, Says};

mod builder;
mod error;
pub use builder::*;
pub use error::*;

/// User agent of this library, intended to be appended to the user agent of a consuming library or
/// program.
pub const USER_AGENT: &str = concat!("cataas-rs/", env!("CARGO_PKG_VERSION"));

/// Cat as a Service client.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    user_agent: String,
}

impl Client {
    /// Create a new [`Client`].
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            user_agent: USER_AGENT.to_owned(),
        }
    }

    /// Create a new [`Builder`].
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub(crate) async fn request<P, R>(
        &self,
        method: Method,
        path: &str,
        params: P,
    ) -> Result<R, Error>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        Ok(self
            .client
            .request(method, format!("https://cataas.com{}", path))
            .header("User-Agent", &self.user_agent)
            .query(&params)
            .send()
            .await?
            .json()
            .await?)
    }

    #[must_use]
    pub fn cat(&self) -> Cat<'_> {
        Cat::new(self)
    }

    #[must_use]
    pub fn says(&self, text: String) -> Says<'_> {
        Says::new(self, text)
    }

    /// Get all available tags.
    pub async fn tags(&self) -> Result<Vec<String>, Error> {
        self.request(Method::GET, "/api/tags", ()).await
    }
}
