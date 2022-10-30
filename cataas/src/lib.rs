#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::missing_errors_doc)]

mod client;
pub mod methods;
pub mod types;

pub use client::*;
