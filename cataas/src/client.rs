use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::methods::{Cat, Says};

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
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
            .header(
                "User-Agent",
                format!("cataas-rs/{}", env!("CARGO_PKG_VERSION")),
            )
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

    pub async fn tags(&self) -> Result<Vec<String>, Error> {
        self.request(Method::GET, "/api/tags", ()).await
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}
