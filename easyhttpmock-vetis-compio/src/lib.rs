#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
/// Vetis smol adapter module
pub mod vetis_adapter;
pub use easyhttpmock::*;
pub use vetis_compio::{handler_fn, Protocol};
#[cfg(test)]
mod tests;
