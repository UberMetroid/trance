//! Core shared types and primitives. Vendored from `runner::core`.
//! Source: /home/jeryd/library/src/core/mod.rs (and included submodules).

pub mod logo_block;
pub mod screen_palette;
pub mod screensaver;

pub use trance_api::{TerminalCell, LcgRng, hsl_to_rgb, rgb_to_hsl, percentage, lerp};
