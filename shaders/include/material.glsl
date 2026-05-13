layout(set = 1, binding = 0) uniform MaterialData {
    vec4 base_color;
    vec4 pbr_params; // x - roughness, y - metallic, z/w - unused
    uvec4 texture_flags; // x - has_albedo, y - has_normal, z - has_roughness, w - has_metallic
} material;

layout(set = 1, binding = 1) uniform sampler2D albedo_tex;
layout(set = 1, binding = 2) uniform sampler2D normal_tex;
layout(set = 1, binding = 3) uniform sampler2D roughness_tex;
layout(set = 1, binding = 4) uniform sampler2D metallic_tex;

vec4 sample_albedo(
    vec4 in_color,
    vec2 uv,
    uvec4 flags
) {
    if (flags.x == 1) {
        return in_color * texture(albedo_tex, uv);
    }

    return in_color;
}

vec4 sample_normal(
    vec4 in_color,
    vec2 uv,
    uvec4 flags
) {
    if (flags.y == 1) {
        return in_color * texture(normal_tex, uv);
    }

    return in_color;
}

vec4 sample_roughness(
    vec4 in_color,
    vec2 uv,
    uvec4 flags
) {
    if (flags.z == 1) {
        return in_color * texture(roughness_tex, uv);
    }

    return in_color;
}

vec4 sample_metallic(
    vec4 in_color,
    vec2 uv,
    uvec4 flags
) {
    if (flags.w == 1) {
        return in_color * texture(metallic_tex, uv);
    }

    return in_color;
}