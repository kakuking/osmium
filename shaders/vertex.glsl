#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec3 v_world_pos;
layout(location = 1) out vec2 v_uv;

void main() {
    v_world_pos = position;
    v_uv = uv;

    gl_Position = vec4(position, 1.0);
}