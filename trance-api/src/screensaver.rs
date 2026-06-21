use std::time::Duration;

use crate::terminal_cell::TerminalCell;

pub trait Screensaver: ScreensaverState {
    fn init(&mut self, _cols: usize, _rows: usize) {}
    fn update(&mut self, dt: Duration, cols: usize, rows: usize);
    fn update_frame_time(&mut self, _dt: Duration) {}
    fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize);
    fn has_scanlines(&self) -> bool {
        false
    }
}

/// FFI-safe wrapper around the Screensaver trait object.
pub struct ScreensaverInstance {
    pub inner: Box<dyn Screensaver>,
}

pub trait ScreensaverState {
    fn active(&self) -> bool;
    fn set_active(&mut self, active: bool);
    fn focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
}

impl<T: Screensaver + ?Sized> ScreensaverState for T {
    fn active(&self) -> bool {
        true
    }
    fn set_active(&mut self, _active: bool) {}
    fn focused(&self) -> bool {
        true
    }
    fn set_focused(&mut self, _focused: bool) {}
}