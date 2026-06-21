// SPDX-License-Identifier: MIT

use crate::cpu;
use crate::FilterMode;

use super::types::GpuUpscaler;

pub struct FrameUpscaler {
    gpu: Option<GpuUpscaler>,
    filter: FilterMode,
    stretch_buf: Vec<u8>,
    stretch_dims: (u32, u32, u32, u32),
    stretch_cache: cpu::StretchCache,
    letterbox_buf: Vec<u8>,
    letterbox_dims: (u32, u32, u32, u32),
}

impl FrameUpscaler {
    pub fn new(prefer_gpu: bool, filter: FilterMode) -> Self {
        let gpu = if prefer_gpu {
            match GpuUpscaler::new() {
                Ok(gpu) => {
                    println!(
                        "trance-gpu: using {} ({})",
                        gpu.adapter_name(),
                        gpu.backend_label()
                    );
                    Some(gpu)
                }
                Err(error) => {
                    eprintln!("trance-gpu: GPU unavailable ({error}); using CPU upscale");
                    None
                }
            }
        } else {
            println!("trance-gpu: GPU disabled; using CPU upscale");
            None
        };

        Self {
            gpu,
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
        self.gpu.is_some()
    }

    pub fn adapter_name(&self) -> Option<&str> {
        self.gpu.as_ref().map(|gpu| gpu.adapter_name())
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
        if let Some(gpu) = self.gpu.as_mut() {
            match gpu.upscale_letterbox_into(src, src_w, src_h, dst_w, dst_h, self.filter, out) {
                Ok(()) => return,
                Err(error) => eprintln!("trance-gpu: frame upscale failed ({error}); CPU fallback"),
            }
        }

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
        if let Some(gpu) = self.gpu.as_mut() {
            match gpu.upscale_stretch_into(src, src_w, src_h, dst_w, dst_h, out) {
                Ok(()) => return,
                Err(error) => {
                    eprintln!("trance-gpu: stretch upscale failed ({error}); CPU fallback");
                }
            }
        }

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