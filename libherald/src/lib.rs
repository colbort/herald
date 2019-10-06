//! Client logic for herald.
#![deny(missing_docs)]
#![allow(clippy::all)]
/// Implementation of autogenerated traits.
pub mod implementation;

/// Autogenerated by `rust-qt-binding-generator`.
#[allow(missing_docs)]
pub mod interface;

/// FFI type aliases.
pub mod ffi;
/// Shared state.
pub mod shared;
/// Utilities.
pub mod utils;

/// App name on desktop, used for toasts
pub(crate) const DESKTOP_APP_NAME: &str = "heraldqtDesktop";
