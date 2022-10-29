#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod client;

pub use client::*;
pub mod methods;
pub mod types;
