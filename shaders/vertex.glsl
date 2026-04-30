#version 460

layout(location = 0) in vec3 position;

layout(location = 1) out vec3 pos;

void main() {
    pos = position;
    gl_Position = vec4(pos, 1.0);
}