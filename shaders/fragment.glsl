#version 460

layout(location = 0) out vec4 f_color;

layout(location = 1) in vec3 pos;

void main() {
    // f_color = vec4(1.0, 0.0, 0.0, 1.0);
    vec3 pos_new = pos + vec3(0.5, 0.5, 0.0);
    f_color = vec4(pos_new, 1.0);
}