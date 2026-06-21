// SPDX-License-Identifier: MIT

use crate::FilterMode;

use super::pixel_format::{bgra_to_rgba_into, rgba_to_bgra_inplace};
use super::types::{GpuUpscaler, Params};

impl GpuUpscaler {
    pub fn upscale_letterbox_into(
        &mut self,
        src: &[u8],
        src_w: u32,
        src_h: u32,
        dst_w: u32,
        dst_h: u32,
        filter: FilterMode,
        out: &mut Vec<u8>,
    ) -> Result<(), String> {
        if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
            out.resize((dst_w * dst_h * 4) as usize, 0);
            return Ok(());
        }

        let expected = (src_w * src_h * 4) as usize;
        if src.len() < expected {
            return Err("source pixel buffer is too small".into());
        }

        let scale = (dst_w as f32 / src_w as f32).min(dst_h as f32 / src_h as f32);
        let display_w = (src_w as f32 * scale).floor().max(1.0);
        let display_h = (src_h as f32 * scale).floor().max(1.0);
        let offset_x = ((dst_w as f32 - display_w) * 0.5).floor();
        let offset_y = ((dst_h as f32 - display_h) * 0.5).floor();

        self.upscale_rect_into(
            src,
            src_w,
            src_h,
            dst_w,
            dst_h,
            [offset_x, offset_y],
            [display_w, display_h],
            filter,
            out,
        )
    }

    pub fn upscale_stretch_into(
        &mut self,
        src: &[u8],
        src_w: u32,
        src_h: u32,
        dst_w: u32,
        dst_h: u32,
        out: &mut Vec<u8>,
    ) -> Result<(), String> {
        if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
            out.resize((dst_w * dst_h * 4) as usize, 0);
            return Ok(());
        }

        let expected = (src_w * src_h * 4) as usize;
        if src.len() < expected {
            return Err("source pixel buffer is too small".into());
        }

        self.upscale_rect_into(
            src,
            src_w,
            src_h,
            dst_w,
            dst_h,
            [0.0, 0.0],
            [dst_w as f32, dst_h as f32],
            FilterMode::Nearest,
            out,
        )
    }

    fn upscale_rect_into(
        &mut self,
        src: &[u8],
        src_w: u32,
        src_h: u32,
        dst_w: u32,
        dst_h: u32,
        rect_origin: [f32; 2],
        rect_size: [f32; 2],
        filter: FilterMode,
        out: &mut Vec<u8>,
    ) -> Result<(), String> {
        bgra_to_rgba_into(&mut self.src_rgba, &mut self.src_rgba_dims, src, src_w, src_h);
        self.ensure_src_texture(src_w, src_h);
        self.ensure_dst_texture(dst_w, dst_h);
        self.ensure_readback(dst_w, dst_h);

        let src_slot = self.src_texture.as_ref().expect("source texture");
        let dst_slot = self.dst_texture.as_ref().expect("destination texture");
        let readback_idx = self.readback_index;
        let readback = self.readback[readback_idx]
            .as_ref()
            .expect("readback buffer");

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &src_slot.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.src_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(src_w * 4),
                rows_per_image: Some(src_h),
            },
            wgpu::Extent3d {
                width: src_w,
                height: src_h,
                depth_or_array_layers: 1,
            },
        );

        let params = Params {
            dst_size: [dst_w as f32, dst_h as f32],
            rect_origin,
            rect_size,
            _pad: [0.0, 0.0],
        };
        self.queue
            .write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&params));

        let sampler = match filter {
            FilterMode::Linear => &self.sampler_linear,
            FilterMode::Nearest => &self.sampler_nearest,
        };

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("trance-upscale-bind-group"),
            layout: &self.bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&src_slot.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("trance-upscale-encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("trance-upscale-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &dst_slot.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &dst_slot.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(readback.bytes_per_row),
                    rows_per_image: Some(dst_h),
                },
            },
            wgpu::Extent3d {
                width: dst_w,
                height: dst_h,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(Some(encoder.finish()));

        let slice = readback.buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        loop {
            self.device.poll(wgpu::Maintain::Poll);
            if let Ok(result) = receiver.try_recv() {
                result.map_err(|error| format!("GPU readback failed: {error}"))?;
                break;
            }
            std::thread::yield_now();
        }

        let mapped = slice.get_mapped_range();
        let row_bytes = (dst_w * 4) as usize;
        let needed = (dst_w * dst_h * 4) as usize;
        if self.output_dims != (dst_w, dst_h) || self.output_bgra.len() != needed {
            self.output_bgra.resize(needed, 0);
            self.output_dims = (dst_w, dst_h);
        }
        for row in 0..dst_h as usize {
            let src_start = row * readback.bytes_per_row as usize;
            let src_end = src_start + row_bytes;
            let dst_start = row * row_bytes;
            let dst_end = dst_start + row_bytes;
            if src_end <= mapped.len() && dst_end <= self.output_bgra.len() {
                self.output_bgra[dst_start..dst_end]
                    .copy_from_slice(&mapped[src_start..src_end]);
            }
        }
        drop(mapped);
        readback.buffer.unmap();
        self.readback_index = 1 - readback_idx;

        rgba_to_bgra_inplace(&mut self.output_bgra);
        out.resize(needed, 0);
        out.copy_from_slice(&self.output_bgra);
        Ok(())
    }
}