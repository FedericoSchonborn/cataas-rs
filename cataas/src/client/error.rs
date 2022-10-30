use thiserror::Error;

/// Errors returned by the [`Client`](crate::Client).
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
