#version 460

#define MAX_LIGHTS 16

struct Light {
    mat4 view_proj;
    vec3 color;
    float pad;
};

layout(location = 0) in vec3 v_world_pos;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_uv;
layout(location = 3) in vec4 v_light_space_pos[MAX_LIGHTS];

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 1) uniform LightCounts {
    uint light_count;
};

layout(set = 0, binding = 2) readonly buffer Lights {
    Light lights[MAX_LIGHTS];
};

layout(set = 0, binding = 3) uniform sampler2D shadow_maps[MAX_LIGHTS];

layout(set = 1, binding = 0) uniform MaterialData {
    vec4 base_color;
    vec4 pbr_params; // x - roughness, y - metallic, z/w - unused
    uvec4 texture_flags; // x - has_albedo, y - has_normal, z - has_roughness, w - has_metallic
} material;

layout(set = 1, binding = 1) uniform sampler2D albedo_tex;
layout(set = 1, binding = 2) uniform sampler2D normal_tex;
layout(set = 1, binding = 3) uniform sampler2D roughness_tex;
layout(set = 1, binding = 4) uniform sampler2D metallic_tex;

float calculate_shadow(uint light_index, vec4 light_space_pos) {
    vec3 proj_coords = light_space_pos.xyz / light_space_pos.w;
    vec3 depth_map_coords = proj_coords * 0.5 + 0.5;

    if (
        depth_map_coords.x < 0.0 || depth_map_coords.x > 1.0 ||
        depth_map_coords.y < 0.0 || depth_map_coords.y > 1.0 ||
        depth_map_coords.z < 0.0 || depth_map_coords.z > 1.0
    ) {
        return 1.0;
    }

    float closest_depth = texture(
        shadow_maps[light_index],
        depth_map_coords.xy
    ).r;

    float current_depth = proj_coords.z;
    float bias = 0.005;
    bias = 0.000;

    return current_depth - bias > closest_depth ? 0.25 : 1.0;
}

void main() {
    vec4 albedo = material.base_color;
    albedo.a = 1.0;

    float total_shadow = 1.0;

    for (uint i = 0; i < min(light_count, MAX_LIGHTS); i++) {
        total_shadow *= calculate_shadow(i, v_light_space_pos[i]);
    }

    if (material.texture_flags.x == 1) {
        albedo *= texture(albedo_tex, v_uv);
    }

    vec3 normal_sample = texture(normal_tex, v_uv).rgb;
    float roughness_sample = texture(roughness_tex, v_uv).r;
    float metallic_sample = texture(metallic_tex, v_uv).r;

    if (material.texture_flags.y == 1) {
        albedo.rgb *= normal_sample * 0.0 + 1.0;
    }

    if (material.texture_flags.z == 1) {
        albedo.rgb *= roughness_sample * 0.0 + 1.0;
    }

    if (material.texture_flags.w == 1) {
        albedo.rgb *= metallic_sample * 0.0 + 1.0;
    }

    albedo.rgb *= total_shadow;

    f_color = albedo;
}