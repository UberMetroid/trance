// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use trance_runner::plugin_meta::DisplayMode;
use trance_runner::plugin_session::PluginSession;
use wayland_present::{OutputLayout, OverlayPresenter};

use super::layout::{monitor_cell_bounds, virtual_desktop};
use super::overlays::{black_frame_into, maybe_draw_overlays};
use crate::presentation::PresentationOptions;

pub fn run_frame_loop(
    presenter: &OverlayPresenter,
    stop: &AtomicBool,
    session: &mut PluginSession,
    layouts: &[OutputLayout],
    primary: OutputLayout,
    display_mode: DisplayMode,
    virtual_cols: usize,
    virtual_rows: usize,
    options: PresentationOptions,
    present_fps: f32,
    tick_hz: f32,
    frame_duration: Duration,
    last_frame: &mut Instant,
    frame_counter: &mut u64,
    fps_report: &mut Instant,
    achieved_fps: &mut f32,
    black_frames: &mut HashMap<(u32, u32), Vec<u8>>,
) -> Result<(), String> {
    let mut scratch_pixels = Vec::new();

    while !stop.load(Ordering::Relaxed) && presenter.is_visible() {
        let frame_start = Instant::now();
        let frame_dt = frame_start.saturating_duration_since(*last_frame);
        *last_frame = frame_start;
        session.tick(frame_dt);

        match display_mode {
            DisplayMode::Expand => {
                for layout in layouts {
                    let (cols, rows) = session.grid_for_pixels(layout.width, layout.height);
                    let mut pixels = session.render(cols, rows, layout.width, layout.height);
                    maybe_draw_overlays(
                        &mut pixels,
                        layout.width,
                        layout.height,
                        layout.id == primary.id,
                        options.show_fps_overlay,
                        *achieved_fps,
                    );
                    presenter.submit_frame(layout.id, layout.width, layout.height, pixels);
                }
            }
            DisplayMode::Mirror => {
                let (cols, rows) = session.grid_for_pixels(primary.width, primary.height);
                let base = session.render(cols, rows, primary.width, primary.height);

                for layout in layouts {
                    if layout.id == primary.id {
                        continue;
                    }
                    session.blit_to_monitor_into(
                        &base,
                        primary.width,
                        primary.height,
                        layout.width,
                        layout.height,
                        &mut scratch_pixels,
                    );
                    maybe_draw_overlays(
                        &mut scratch_pixels,
                        layout.width,
                        layout.height,
                        false,
                        options.show_fps_overlay,
                        *achieved_fps,
                    );
                    let cap = scratch_pixels.capacity();
                    presenter.submit_frame(
                        layout.id,
                        layout.width,
                        layout.height,
                        std::mem::replace(&mut scratch_pixels, Vec::with_capacity(cap)),
                    );
                }

                let mut primary_pixels = base;
                maybe_draw_overlays(
                    &mut primary_pixels,
                    primary.width,
                    primary.height,
                    true,
                    options.show_fps_overlay,
                    *achieved_fps,
                );
                presenter.submit_frame(
                    primary.id,
                    primary.width,
                    primary.height,
                    primary_pixels,
                );
            }
            DisplayMode::PrimaryOnly => {
                let (cols, rows) = session.grid_for_pixels(primary.width, primary.height);
                let mut primary_pixels =
                    session.render(cols, rows, primary.width, primary.height);
                maybe_draw_overlays(
                    &mut primary_pixels,
                    primary.width,
                    primary.height,
                    true,
                    options.show_fps_overlay,
                    *achieved_fps,
                );
                presenter.submit_frame(
                    primary.id,
                    primary.width,
                    primary.height,
                    primary_pixels,
                );

                for layout in layouts {
                    if layout.id == primary.id {
                        continue;
                    }
                    black_frame_into(black_frames, layout.width, layout.height, &mut scratch_pixels);
                    let cap = scratch_pixels.capacity();
                    presenter.submit_frame(
                        layout.id,
                        layout.width,
                        layout.height,
                        std::mem::replace(&mut scratch_pixels, Vec::with_capacity(cap)),
                    );
                }
            }
            DisplayMode::Span => {
                let (min_x, min_y, total_w, total_h) = virtual_desktop(layouts);
                let scanlines = session.draw_frame(virtual_cols, virtual_rows);
                for layout in layouts {
                    let bounds = monitor_cell_bounds(
                        *layout,
                        min_x,
                        min_y,
                        total_w,
                        total_h,
                        virtual_cols,
                        virtual_rows,
                        layout.id == primary.id,
                    );
                    let col_w = bounds.end_col.saturating_sub(bounds.start_col).max(1);
                    let row_h = bounds.end_row.saturating_sub(bounds.start_row).max(1);
                    let mut pixels = session.raster_viewport(
                        bounds.start_col,
                        bounds.start_row,
                        col_w,
                        row_h,
                        virtual_cols,
                        virtual_rows,
                        layout.width,
                        layout.height,
                        scanlines,
                    );
                    maybe_draw_overlays(
                        &mut pixels,
                        layout.width,
                        layout.height,
                        layout.id == primary.id,
                        options.show_fps_overlay,
                        *achieved_fps,
                    );
                    presenter.submit_frame(layout.id, layout.width, layout.height, pixels);
                }
            }
        }

        *frame_counter += 1;
        let elapsed = frame_start.elapsed();
        if fps_report.elapsed() >= Duration::from_secs(1) {
            *achieved_fps = *frame_counter as f32 / fps_report.elapsed().as_secs_f32();
            if *frame_counter >= present_fps as u64 || fps_report.elapsed() >= Duration::from_secs(5) {
                println!(
                    "trance-daemon: achieved {:.1} FPS (target {:.0}, tick {:.0})",
                    *achieved_fps, present_fps, tick_hz
                );
                *frame_counter = 0;
                *fps_report = Instant::now();
            }
        }

        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }

    Ok(())
}