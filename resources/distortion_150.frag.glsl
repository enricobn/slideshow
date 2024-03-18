#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform Dim {
    float u_Rate;
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
    Target0 = texture(t_Texture, SineWave(v_Uv)) * v_Color;
}
