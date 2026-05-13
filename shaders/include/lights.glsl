
#define MAX_LIGHTS 16

struct Light {
    mat4 view_proj;
    vec3 color;
    float pad;
};

layout(set = 0, binding = 1) uniform LightCounts {
    uint light_count;
};

layout(set = 0, binding = 2) readonly buffer Lights {
    Light lights[MAX_LIGHTS];
};

layout(set = 0, binding = 3) uniform sampler2D shadow_maps[MAX_LIGHTS];

float calculate_shadow_one_light(uint light_index, vec4 light_space_pos) {
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

float calculate_shadow(vec4 light_space_positions[MAX_LIGHTS]) {
    float total_shadow = 1.0;

    for (uint i = 0; i < min(light_count, MAX_LIGHTS); i++) {
        total_shadow *= calculate_shadow_one_light(i, light_space_positions[i]);
    }

    return total_shadow;
}