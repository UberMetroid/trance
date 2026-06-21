// SPDX-License-Identifier: MIT

pub fn align_to(value: u32, alignment: u32) -> u32 {
    value.div_ceil(alignment) * alignment
}

pub fn bgra_to_rgba_into(
    dst: &mut Vec<u8>,
    dims: &mut (u32, u32),
    src: &[u8],
    width: u32,
    height: u32,
) {
    let needed = (width * height * 4) as usize;
    if *dims != (width, height) || dst.len() != needed {
        dst.resize(needed, 0);
        *dims = (width, height);
    }
    for (src_px, dst_px) in src.chunks_exact(4).zip(dst.chunks_exact_mut(4)) {
        dst_px[0] = src_px[2];
        dst_px[1] = src_px[1];
        dst_px[2] = src_px[0];
        dst_px[3] = src_px[3];
    }
}

pub fn rgba_to_bgra_inplace(pixels: &mut [u8]) {
    for px in pixels.chunks_exact_mut(4) {
        px.swap(0, 2);
    }
}