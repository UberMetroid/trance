// SPDX-License-Identifier: MIT

//! wgpu-backed frame upscaling for trance screensaver presentation.
//!
//! The GPU path converts BGRA plugin frames to RGBA, renders into a destination
//! texture with optional letterboxing, then readbacks BGRA for Wayland buffers.
//! [`FrameUpscaler`] falls back to the CPU implementation when no adapter is found.
//!
//! Internal modules:
//! - [`init`] — adapter/device/pipeline setup
//! - [`upscale`] — render pass and readback
//! - [`textures`] — pooled source/destination/readback slots
//! - [`pixel_format`] — BGRA↔RGBA conversion helpers
//! - [`shader`] — WGSL fullscreen blit shader
//!
//! Public consumers use [`FrameUpscaler`] only; internal `GpuUpscaler` details stay private.

mod frame_upscaler;
mod init;
mod pixel_format;
mod shader;
mod textures;
mod types;
mod upscale;

pub use frame_upscaler::FrameUpscaler;

// CPU fallback mirrors GPU letterbox math in trance-gpu/src/cpu.rs.