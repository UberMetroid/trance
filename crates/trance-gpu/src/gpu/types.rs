// SPDX-License-Identifier: MIT

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Params {
    pub dst_size: [f32; 2],
    pub rect_origin: [f32; 2],
    pub rect_size: [f32; 2],
    pub _pad: [f32; 2],
}

pub struct TextureSlot {
    pub width: u32,
    pub height: u32,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

pub struct ReadbackSlot {
    pub width: u32,
    pub height: u32,
    pub buffer: wgpu::Buffer,
    pub bytes_per_row: u32,
}

pub struct GpuUpscaler {
    pub(crate) backend_label: String,
    pub(crate) adapter_name: String,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) bind_layout: wgpu::BindGroupLayout,
    pub(crate) sampler_linear: wgpu::Sampler,
    pub(crate) sampler_nearest: wgpu::Sampler,
    pub(crate) params_buffer: wgpu::Buffer,
    pub(crate) src_texture: Option<TextureSlot>,
    pub(crate) dst_texture: Option<TextureSlot>,
    pub(crate) readback: [Option<ReadbackSlot>; 2],
    pub(crate) readback_index: usize,
    pub(crate) src_rgba: Vec<u8>,
    pub(crate) src_rgba_dims: (u32, u32),
    pub(crate) output_bgra: Vec<u8>,
    pub(crate) output_dims: (u32, u32),
}