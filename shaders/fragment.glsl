#version 460

layout(location = 0) in vec3 v_world_pos;
layout(location = 1) in vec2 v_uv;

layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform MaterialData {
    vec4 base_color;

    // x = roughness
    // y = metallic
    // z/w unused
    vec4 pbr_params;

    // x = has_albedo
    // y = has_normal
    // z = has_roughness
    // w = has_metallic
    uvec4 texture_flags;
} material;

layout(set = 1, binding = 1) uniform sampler2D albedo_tex;
layout(set = 1, binding = 2) uniform sampler2D normal_tex;
layout(set = 1, binding = 3) uniform sampler2D roughness_tex;
layout(set = 1, binding = 4) uniform sampler2D metallic_tex;

void main() {
    vec4 albedo = material.base_color;
    albedo.a = 1.0;

    // if (material.has_texture_albedo == 1) {
    if (material.texture_flags.x == 1) {
        albedo *= texture(albedo_tex, v_uv);
    }

    vec3 normal_sample = texture(normal_tex, v_uv).rgb;
    float roughness_sample = texture(roughness_tex, v_uv).r;
    float metallic_sample = texture(metallic_tex, v_uv).r;

    // if (material.has_texture_normal == 1) {
    if (material.texture_flags.y == 1) {
        albedo.rgb *= normal_sample * 0.0 + 1.0;
    }

    // if (material.has_texture_roughness == 1) {
    if (material.texture_flags.z == 1) {
        albedo.rgb *= roughness_sample * 0.0 + 1.0;
    }

    // if (material.has_texture_metallic == 1) {
    if (material.texture_flags.w == 1) {
        albedo.rgb *= metallic_sample * 0.0 + 1.0;
    }

    f_color = albedo;

    // f_color = vec4(0.0, 0.0, 0.0, 1.0);
}