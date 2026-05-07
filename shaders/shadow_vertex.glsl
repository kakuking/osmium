#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(push_constant) uniform PushConstants {
    mat4 model;
    mat4 view_proj;
} pc;

layout(set = 2, binding = 0) uniform sampler2D heightmap;

void main() {
    vec2 v_uv = vec2(uv.x, 1.0 - uv.y);

    float height = texture(heightmap, v_uv).r;

    vec3 displaced_position = position + normalize(normal) * height;

    vec4 world_pos = pc.model * vec4(displaced_position, 1.0);

    gl_Position = pc.view_proj * world_pos;
}