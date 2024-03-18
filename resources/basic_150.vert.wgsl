struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct GgezDrawUniforms {
    color: vec4<f32>,
    src_rect: vec4<f32>,
    transform: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: GgezDrawUniforms;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = color;
    out.uv = uv * uniforms.src_rect.zw + uniforms.src_rect.xy;

    let instance_transform = mat4x4(uniforms.transform[0], uniforms.transform[1], uniforms.transform[2], uniforms.transform[3]) * vec4(position, 0.0, 1.0);
    let position = instance_transform * vec4<f32>(position, 0.0, 1.0);

    out.position = uniforms.transform * position; //uniforms.transform * my_uniforms.rotation * vec4<f32>(position, 0.0, 1.0);
    out.color = color;
    out.uv = uv;
    out.position = position;
    return out;
}