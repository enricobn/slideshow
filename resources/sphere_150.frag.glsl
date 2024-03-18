#version 150 core

// From https://www.objc.io/issues/21-camera-and-photos/gpu-accelerated-image-processing/#glsl-data-types-and-operations

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform Dim {
    float u_Rate;
    float center_x;
    float center_y;
    float radius;
    float aspectRatio;
    float refractiveIndex;
};

vec2 SineWave( vec2 p) {
    float pi = 3.14159;
    float A = 0.30;
    float w = 10.0 * pi;
    float t = 30.0 * pi / 180.0;
    float y = sin( w*p.x + t) * A * u_Rate;
    return vec2(p.x, p.y + y);
}

void main(){
    //vec2 textureCoordinateToUse = vec2(v_Uv.x, (v_Uv.y * aspectRatio + 0.5 - 0.5 * aspectRatio));
    vec2 textureCoordinateToUse = vec2((v_Uv.x * aspectRatio + 0.5 - 0.5 * aspectRatio), v_Uv.y);
    vec2 center = vec2(center_x, center_y);
    float distanceFromCenter = distance(center, textureCoordinateToUse);
    float real_radius = 0.5;
    float checkForPresenceWithinSphere = step(distanceFromCenter, real_radius);

    distanceFromCenter = distanceFromCenter / radius;

    float normalizedDepth = radius * sqrt(1.0 - distanceFromCenter * distanceFromCenter);
    vec3 sphereNormal = normalize(vec3(textureCoordinateToUse - center, normalizedDepth));

    vec3 refractedVector = refract(vec3(0.0, 0.0, -1.0), sphereNormal, refractiveIndex);

    // I recalculate normalizedDepth with real_radius to smooth the sphere
    distanceFromCenter = distance(center, textureCoordinateToUse);
    distanceFromCenter = distanceFromCenter / real_radius;
    normalizedDepth = sqrt(1.0 - distanceFromCenter * distanceFromCenter);

    Target0 = texture(t_Texture, (refractedVector.xy + 1.0) * 0.5) * checkForPresenceWithinSphere * normalizedDepth;
}