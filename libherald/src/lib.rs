//! Client logic for herald.
#![deny(missing_docs)]
#![allow(clippy::all)]
#![macro_use]
extern crate heraldcore;

/// Implementation of autogenerated traits.
pub mod implementation;

/// Autogenerated by `rust-qt-binding-generator`.
#[allow(missing_docs)]
pub mod interface;

/// FFI type aliases.
pub mod ffi;
/// Shared state.
pub mod shared;
/// Desktop push notifications
#[cfg(all(
    target_family = "unix",
    not(any(target_os = "android", target_os = "ios"))
))]
#[cfg_attr(
    all(target_family = "unix", not(target_os = "macos")),
    path = "toasts/xdg.rs"
)]
#[cfg_attr(target_os = "macos", path = "toasts/macos.rs")]
#[cfg_attr(not(target_family = "unix"), path = "toasts/other.rs")]
pub mod toasts;
/// Utilities.
pub mod utils;

/// App name on desktop, used for toasts
pub(crate) const DESKTOP_APP_NAME: &str = "heraldqtDesktop";
