// SPDX-License-Identifier: MIT

pub const SHADER: &str = r#"
struct Params {
    dst_size: vec2<f32>,
    rect_origin: vec2<f32>,
    rect_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    let pos = positions[vertex_index];
    var out: VertexOutput;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = (pos + vec2<f32>(1.0, 1.0)) * 0.5;
    return out;
}

@fragment
fn fs(input: VertexOutput) -> @location(0) vec4<f32> {
    let pixel = input.uv * params.dst_size;
    let rect_min = params.rect_origin;
    let rect_max = params.rect_origin + params.rect_size;
    if (pixel.x < rect_min.x || pixel.y < rect_min.y || pixel.x >= rect_max.x || pixel.y >= rect_max.y) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    let local = (pixel - rect_min) / params.rect_size;
    let uv = vec2<f32>(local.x, 1.0 - local.y);
    return textureSample(src_tex, src_sampler, uv);
}
"#;