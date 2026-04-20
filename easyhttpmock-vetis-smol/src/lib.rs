#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
/// Vetis smol adapter module
pub mod vetis_adapter;
pub use easyhttpmock::*;
pub use vetis_smol::Protocol;
#[cfg(test)]
mod tests;
