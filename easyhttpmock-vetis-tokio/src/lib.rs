#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
/// Vetis tokio adapter module
pub mod vetis_adapter;
pub use easyhttpmock::*;
pub use vetis_tokio::handler_fn;
#[cfg(test)]
mod tests;
