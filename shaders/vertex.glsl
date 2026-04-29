#version 460

layout(location = 0) in vec2 position;

layout(location = 1) out vec3 pos;

void main() {
    pos = vec3(position, 0.0);
    gl_Position = vec4(pos, 1.0);
}