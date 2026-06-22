//! Lightweight shared API definitions, traits, and math utilities for terminal
//! screensaver plugins. Host applications register callbacks for live system
//! queries; plugins depend only on this crate for portable drawing primitives.

mod callbacks;
mod caption;
mod color;
mod layout;
mod logo_block;
mod monitor;
mod palette;
mod rng;
mod screensaver;
mod system_info;
mod terminal_cell;

pub use callbacks::{get_system_info, query_current_palette, PALETTE_CALLBACK, SYSTEM_INFO_CALLBACK};
pub use caption::{caption_text, clear_caption, publish_caption};
pub use color::{hsl_to_rgb, lerp, percentage, rgb_to_hsl};
pub use layout::{is_span_layout, place_centered_logo, span_reach_scale, CenteredLogo};
pub use logo_block::render_logo_block;
pub use monitor::{
    clear_primary_bounds, get_primary_monitor_bounds, is_secondary_monitor, publish_primary_bounds,
    MonitorCellBounds, IS_SECONDARY_MONITOR_CALLBACK, MONITOR_BOUNDS_CALLBACK,
};
pub use palette::ScreenPalette;
pub use rng::LcgRng;
pub use screensaver::{Screensaver, ScreensaverInstance, ScreensaverState, GpuSpotlight};
pub use system_info::SystemInfo;
pub use terminal_cell::TerminalCell;

/// Compatibility module structures for minimal changes in screensaver ports.
pub mod core {
    pub use crate::{
        hsl_to_rgb, lerp, percentage, rgb_to_hsl, LcgRng, Screensaver, ScreensaverState,
        TerminalCell, GpuSpotlight,
    };
    pub mod screensaver {
        pub use crate::{Screensaver, ScreensaverState, GpuSpotlight};
    }
    pub mod logo_block {
        pub use crate::logo_block::render_logo_block;
    }
}

pub mod toolkit {
    pub mod sys_info {
        pub use crate::{
            caption_text, clear_caption, get_primary_monitor_bounds, get_system_info,
            is_secondary_monitor, is_span_layout, place_centered_logo, publish_caption,
            query_current_palette, span_reach_scale, CenteredLogo, MonitorCellBounds, SystemInfo,
        };
    }
}