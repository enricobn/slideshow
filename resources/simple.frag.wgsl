struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}
/*
struct Dim {
    rate: f32,
    center_x: f32,
    center_y: f32,
    radius: f32,
    aspectRatio: f32,
    refractiveIndex: f32
}

@group(3) @binding(0)
var<uniform> dim: Dim;
*/

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t, s, in.uv) * in.color;
}