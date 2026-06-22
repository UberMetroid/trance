// SPDX-License-Identifier: MIT

//! CPU upscaling for trance screensaver frames.

mod cpu;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

impl FilterMode {
    pub fn from_env() -> Self {
        match std::env::var("TRANCE_GPU_FILTER").as_deref() {
            Ok("nearest") => Self::Nearest,
            _ => Self::Linear,
        }
    }
}

/// Whether GPU upscaling should be attempted (always false since GPU upscaling is removed).
pub fn gpu_enabled() -> bool {
    false
}

/// Simulation grid scale factor in `(0, 1]`. Lower values render chunkier effects
/// that are upscaled to the monitor resolution.
pub fn render_scale() -> f32 {
    render_scale_for_gpu(gpu_enabled())
}

pub fn render_scale_for_gpu(use_gpu: bool) -> f32 {
    resolve_render_scale(use_gpu, None)
}

/// Effective simulation grid scale: env `TRANCE_RENDER_SCALE`, then config, then defaults.
pub fn resolve_render_scale(use_gpu: bool, configured: Option<f32>) -> f32 {
    if let Some(scale) = std::env::var("TRANCE_RENDER_SCALE").ok().and_then(|v| v.parse::<f32>().ok()) {
        return scale.clamp(0.25, 1.0);
    }
    if let Some(scale) = configured {
        return scale.clamp(0.25, 1.0);
    }
    if use_gpu { 1.0 } else { 0.5 }
}

/// Presentation frame-rate cap. `0` means match the detected monitor refresh rate.
pub fn max_fps() -> u32 {
    std::env::var("TRANCE_MAX_FPS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(0)
}

/// Physics / simulation tick rate (Hz). Independent of monitor refresh.
pub fn simulation_tick_hz() -> f32 {
    std::env::var("TRANCE_TICK_HZ")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .map(|hz| hz.clamp(15.0, 240.0))
        .unwrap_or(60.0)
}

pub fn target_fps(detected_refresh_hz: u32) -> f32 {
    let detected = detected_refresh_hz.max(60);
    let cap = max_fps();
    if cap == 0 {
        detected as f32
    } else {
        detected.min(cap) as f32
    }
}

pub use trance_api::GpuSpotlight;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuCell {
    pub ch: u32,
    pub fg: [u8; 4],
    pub bg: [u8; 4],
    pub bold: u32,
}

pub struct FrameUpscaler {
    filter: FilterMode,
    stretch_buf: Vec<u8>,
    stretch_dims: (u32, u32, u32, u32),
    stretch_cache: cpu::StretchCache,
    letterbox_buf: Vec<u8>,
    letterbox_dims: (u32, u32, u32, u32),
}

impl FrameUpscaler {
    pub fn new(_prefer_gpu: bool, filter: FilterMode) -> Self {
        Self {
            filter,
            stretch_buf: Vec::new(),
            stretch_dims: (0, 0, 0, 0),
            stretch_cache: cpu::StretchCache::new(),
            letterbox_buf: Vec::new(),
            letterbox_dims: (0, 0, 0, 0),
        }
    }

    fn ensure_stretch_buf(&mut self, src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) {
        let dims = (src_w, src_h, dst_w, dst_h);
        let needed = (dst_w * dst_h * 4) as usize;
        if self.stretch_dims != dims || self.stretch_buf.len() != needed {
            self.stretch_buf.resize(needed, 0);
            self.stretch_dims = dims;
        }
    }

    fn ensure_letterbox_buf(&mut self, src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) {
        let dims = (src_w, src_h, dst_w, dst_h);
        let needed = (dst_w * dst_h * 4) as usize;
        if self.letterbox_dims != dims || self.letterbox_buf.len() != needed {
            self.letterbox_buf.resize(needed, 0);
            self.letterbox_dims = dims;
        }
    }

    pub fn using_gpu(&self) -> bool {
        false
    }

    pub fn adapter_name(&self) -> Option<&str> {
        None
    }

    pub fn upscale_letterbox_into(
        &mut self,
        src: &[u8],
        src_w: u32,
        src_h: u32,
        dst_w: u32,
        dst_h: u32,
        out: &mut Vec<u8>,
    ) {
        self.ensure_letterbox_buf(src_w, src_h, dst_w, dst_h);
        cpu::upscale_letterbox_into(
            &mut self.letterbox_buf,
            src,
            src_w,
            src_h,
            dst_w,
            dst_h,
            self.filter,
        );
        out.resize(self.letterbox_buf.len(), 0);
        out.copy_from_slice(&self.letterbox_buf);
    }

    /// Stretch source to fill the destination (fullscreen presentation path).
    pub fn upscale_stretch_into(
        &mut self,
        src: &[u8],
        src_w: u32,
        src_h: u32,
        dst_w: u32,
        dst_h: u32,
        out: &mut Vec<u8>,
    ) {
        self.ensure_stretch_buf(src_w, src_h, dst_w, dst_h);
        cpu::upscale_stretch_into(
            &mut self.stretch_buf,
            src,
            src_w,
            src_h,
            dst_w,
            dst_h,
            &mut self.stretch_cache,
        );
        out.resize(self.stretch_buf.len(), 0);
        out.copy_from_slice(&self.stretch_buf);
    }
}