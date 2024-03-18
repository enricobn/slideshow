// From https://www.objc.io/issues/21-camera-and-photos/gpu-accelerated-image-processing/#glsl-data-types-and-operations

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Dim {
    rate: f32,
    center_x: f32,
    center_y: f32,
    radius: f32,
    aspectRatio: f32,
    refractiveIndex: f32
}

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

@group(3) @binding(0)
var<uniform> dim: Dim;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //vec2 textureCoordinateToUse = vec2(in.uv.x, (in.uv.y * dim.aspectRatio + 0.5 - 0.5 * dim.aspectRatio));
    let textureCoordinateToUse = vec2((in.uv.x * dim.aspectRatio + 0.5 - 0.5 * dim.aspectRatio), in.uv.y);
    let center = vec2(dim.center_x, dim.center_y);
    let distanceFromCenter = distance(center, textureCoordinateToUse);
    let real_radius = 0.5;
    let checkForPresenceWithinSphere = step(distanceFromCenter, real_radius);

    let distanceFromCenter0 = distanceFromCenter / dim.radius;

    let normalizedDepth = dim.radius * sqrt(1.0 - distanceFromCenter0 * distanceFromCenter0);
    let sphereNormal = normalize(vec3(textureCoordinateToUse - center, normalizedDepth));

    let refractedVector = refract(vec3(0.0, 0.0, -1.0), sphereNormal, dim.refractiveIndex);

    // I recalculate normalizedDepth with real_radius to smooth the sphere
    let distanceFromCenter1 = distance(center, textureCoordinateToUse);
    let distanceFromCenter2 = distanceFromCenter1 / real_radius;
    let normalizedDepth0 = sqrt(1.0 - distanceFromCenter2 * distanceFromCenter2);

    //return textureSample(t, s, (refractedVector.xy + 1.0) * 0.5) * checkForPresenceWithinSphere * normalizedDepth0;
    return in.position;
}