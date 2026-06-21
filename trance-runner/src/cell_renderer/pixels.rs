// SPDX-License-Identifier: MIT

pub fn letterbox_into(
    content: &[u8],
    content_w: u32,
    content_h: u32,
    width: u32,
    height: u32,
    offset_x: usize,
    offset_y: usize,
) -> Vec<u8> {
    let mut framed = vec![0u8; (width * height * 4) as usize];
    for row in 0..content_h as usize {
        let src_start = row * content_w as usize * 4;
        let src_end = src_start + content_w as usize * 4;
        let dst_row = offset_y + row;
        if dst_row >= height as usize {
            break;
        }
        let dst_start = (dst_row * width as usize + offset_x) * 4;
        let dst_end = dst_start + content_w as usize * 4;
        if src_end <= content.len() && dst_end <= framed.len() {
            framed[dst_start..dst_end].copy_from_slice(&content[src_start..src_end]);
        }
    }
    framed
}

pub fn fill_rect(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: (u8, u8, u8),
) {
    for row in y..y.saturating_add(h).min(height as usize) {
        for col in x..x.saturating_add(w).min(width as usize) {
            write_pixel(pixels, width, col, row, color, 0xFF);
        }
    }
}

pub fn dim_rect(pixels: &mut [u8], width: u32, height: u32, x: usize, y: usize, w: usize, h: usize) {
    for row in y..y.saturating_add(h).min(height as usize) {
        for col in x..x.saturating_add(w).min(width as usize) {
            let offset = pixel_offset(width, col, row);
            if offset + 2 < pixels.len() {
                pixels[offset] = pixels[offset] / 2;
                pixels[offset + 1] = pixels[offset + 1] / 2;
                pixels[offset + 2] = pixels[offset + 2] / 2;
            }
        }
    }
}

pub fn blit_bitmap(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    x: usize,
    y: usize,
    bitmap: &[u8],
    bitmap_w: usize,
    bitmap_h: usize,
    color: (u8, u8, u8),
) {
    for row in 0..bitmap_h {
        for col in 0..bitmap_w {
            let alpha = *bitmap.get(row * bitmap_w + col).unwrap_or(&0);
            if alpha == 0 {
                continue;
            }
            let px = x + col;
            let py = y + row;
            if px >= width as usize || py >= height as usize {
                continue;
            }
            write_pixel(pixels, width, px, py, color, alpha);
        }
    }
}

pub fn write_pixel(pixels: &mut [u8], width: u32, x: usize, y: usize, color: (u8, u8, u8), alpha: u8) {
    let offset = pixel_offset(width, x, y);
    if offset + 3 >= pixels.len() {
        return;
    }

    if alpha == 0xFF {
        pixels[offset] = color.2;
        pixels[offset + 1] = color.1;
        pixels[offset + 2] = color.0;
        pixels[offset + 3] = 0xFF;
        return;
    }

    let src_a = alpha as f32 / 255.0;
    let dst_a = pixels[offset + 3] as f32 / 255.0;
    let out_a = src_a + dst_a * (1.0 - src_a);
    if out_a <= 0.0 {
        return;
    }

    let blend = |src: u8, dst: u8| {
        let src_f = src as f32 / 255.0;
        let dst_f = dst as f32 / 255.0;
        ((src_f * src_a + dst_f * dst_a * (1.0 - src_a)) / out_a * 255.0) as u8
    };

    pixels[offset] = blend(color.2, pixels[offset]);
    pixels[offset + 1] = blend(color.1, pixels[offset + 1]);
    pixels[offset + 2] = blend(color.0, pixels[offset + 2]);
    pixels[offset + 3] = (out_a * 255.0) as u8;
}

fn pixel_offset(width: u32, x: usize, y: usize) -> usize {
    ((y as u32 * width + x as u32) * 4) as usize
}