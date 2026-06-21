// SPDX-License-Identifier: MIT

//! Fullscreen Wayland overlays using [`zwlr_layer_shell_v1`].
//!
//! [`OverlayPresenter`] draws layer-shell surfaces above application windows.
//! Solid-color fills and screensaver frames share the same presenter thread and
//! output registry so multi-monitor layouts stay consistent across configure
//! events and refresh-rate reporting.
//!
//! Consumers submit per-output BGRA frames via [`OverlayPresenter::submit_frame`];
//! the overlay thread attaches SHM buffers and marks damage per monitor.
//!
//! [`zwlr_layer_shell_v1`]: https://wayland.app/protocols/wlr-layer-shell-unstable-v1
//!
//! Requires a compositor that implements wlr-layer-shell (COSMIC, Sway, Hyprland, etc.).

mod appearance;
mod output;
mod overlay;
mod presenter;

pub use appearance::OverlayAppearance;
pub use output::OutputLayout;
pub use presenter::OverlayPresenter;

// Presenter commands are processed on a dedicated Wayland thread.