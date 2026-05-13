#version 460

#include "include/lights.glsl"
#include "include/camera.glsl"
#include "include/material.glsl"

layout(location = 0) in vec3 v_world_pos;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_uv;
layout(location = 3) in vec4 v_light_space_pos[MAX_LIGHTS];

layout(location = 0) out vec4 f_color;

void main() {
    vec4 albedo = material.base_color;
    vec3 normal_sample = texture(normal_tex, v_uv).rgb;
    float roughness_sample = texture(roughness_tex, v_uv).r;
    float metallic_sample = texture(metallic_tex, v_uv).r;

    float shadow_factor = calculate_shadow(v_light_space_pos);

    albedo = sample_albedo(albedo, v_uv, material.texture_flags);

    albedo.rgb *= shadow_factor;

    f_color = albedo;
}