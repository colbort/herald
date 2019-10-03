//! Core logic for herald client.

#![warn(missing_docs)]

/// Chainmail support
mod chainkeys;
/// User configuration
pub mod config;
/// Functions and data structures related to contacts.
pub mod contact;
/// Contact keys
mod contact_keys;
/// Conversations
pub mod conversation;
/// Wrapper around database.
pub mod db;
/// Errors
mod errors;
/// Image processing
pub(crate) mod image_utils;
/// Members of conversations
pub mod members;
/// Functions and data structures related to messages.
pub mod message;
/// message status
mod message_receipts;
/// Networking
pub mod network;
/// Pending out messages
pub mod pending;
/// Types
pub mod types;
/// Utils
pub mod utils;
pub use chrono;