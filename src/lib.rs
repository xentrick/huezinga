// #[cfg(test)]
// extern crate tokio;
// #![cfg_attr(test, deny(warnings))]

#[macro_use] extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate failure;

/// Handles all the communication with the bridge
pub mod bridge;
/// Structs mapping the different JSON-objects used with Hue API
pub mod json;
