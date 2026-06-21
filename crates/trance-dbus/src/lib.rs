// SPDX-License-Identifier: MIT

//! D-Bus API for the trance screensaver daemon (`com.local76.Trance`).
//!
//! The daemon exports configuration, preview, inhibit, and status signals on the
//! session bus. [`TranceClient`] wraps the typed methods for applets and CLI tools;
//! [`DaemonStatus`] is the canonical status snapshot shared with consumers.
//!
//! ## Constants
//!
//! - [`SERVICE_NAME`] — bus name (`com.local76.Trance`)
//! - [`OBJECT_PATH`] — object path (`/com/local76/Trance`)
//! - [`INTERFACE_NAME`] — interface name (same as service)
//!
//! Clients should prefer [`TranceClient`] over raw D-Bus for typed errors and
//! status decoding via [`DaemonStatus::from_map`].

pub mod client;
pub mod status;

pub use client::{daemon_available, TranceClient};
pub use status::DaemonStatus;

pub const SERVICE_NAME: &str = "com.local76.Trance";
pub const OBJECT_PATH: &str = "/com/local76/Trance";
pub const INTERFACE_NAME: &str = "com.local76.Trance";

// Status signals use HashMap payloads for forward-compatible applet parsing.