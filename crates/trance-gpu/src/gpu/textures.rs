// SPDX-License-Identifier: MIT

use super::pixel_format::align_to;
use super::types::{GpuUpscaler, ReadbackSlot, TextureSlot};

impl GpuUpscaler {
    pub(crate) fn ensure_src_texture(&mut self, width: u32, height: u32) {
        if self
            .src_texture
            .as_ref()
            .is_some_and(|slot| slot.width == width && slot.height == height)
        {
            return;
        }

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("trance-src"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        self.src_texture = Some(TextureSlot {
            width,
            height,
            texture,
            view,
        });
    }

    pub(crate) fn ensure_dst_texture(&mut self, width: u32, height: u32) {
        if self
            .dst_texture
            .as_ref()
            .is_some_and(|slot| slot.width == width && slot.height == height)
        {
            return;
        }

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("trance-dst"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        self.dst_texture = Some(TextureSlot {
            width,
            height,
            texture,
            view,
        });
    }

    pub(crate) fn ensure_readback(&mut self, width: u32, height: u32) {
        let bytes_per_row = align_to(width * 4, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        for slot in &mut self.readback {
            if slot
                .as_ref()
                .is_some_and(|s| s.width == width && s.height == height)
            {
                continue;
            }

            let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("trance-readback"),
                size: (bytes_per_row * height) as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            *slot = Some(ReadbackSlot {
                width,
                height,
                buffer,
                bytes_per_row,
            });
        }
    }
}