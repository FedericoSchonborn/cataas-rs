#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod methods;
pub mod types;

#[doc(inline)]
pub use client::Client;
pub use client::{Builder as ClientBuilder, Error as ClientError};
