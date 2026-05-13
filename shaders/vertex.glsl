#version 460

#include "include/lights.glsl"
#include "include/camera.glsl"

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec3 v_world_pos;
layout(location = 1) out vec3 v_normal;
layout(location = 2) out vec2 v_uv;
layout(location = 3) out vec4 v_light_space_pos[MAX_LIGHTS];

layout(push_constant) uniform PushConstants {
    mat4 model;
} pc;

layout(set = 2, binding = 0) uniform sampler2D heightmap;

void main() {
    v_uv = vec2(uv.x, 1.0 - uv.y);

    float height = texture(heightmap, v_uv).r;

    vec3 displaced_position = position + normalize(normal) * height;

    vec4 world_pos = pc.model * vec4(displaced_position, 1.0);

    for (uint i = 0; i < min(light_count, MAX_LIGHTS); i++) {
        v_light_space_pos[i] = lights[i].view_proj * world_pos;
    }

    v_world_pos = world_pos.xyz;
    v_normal = mat3(transpose(inverse(pc.model))) * normal;

    gl_Position = camera.view_proj * world_pos;
}