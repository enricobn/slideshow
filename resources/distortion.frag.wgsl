struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Dim {
    rate: f32
}

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

@group(3) @binding(0)
var<uniform> dim: Dim;

fn SineWave( p: vec2<f32>) -> vec2<f32> {
    let pi = 3.14159;
    let A = 0.30;
    let w = 10.0 * pi;
    let t = 30.0 * pi / 180.0;
    let y = sin( w*p.x + t) * A * dim.rate;
    return vec2(p.x, p.y + y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t, s, SineWave(in.uv)) * in.color;
}
