#version 460

layout(set = 0, binding = 0) uniform CameraData {
    mat4 view;
    mat4 proj;
    mat4 view_proj;
    vec4 camera_pos;
} camera;

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec3 v_world_pos;
layout(location = 1) out vec2 v_uv;

layout(push_constant) uniform PushConstants {
    mat4 model;
} pc;

void main() {
    v_world_pos = position;
    v_uv = vec2(uv.x, 1.0 - uv.y);

    vec4 pp = camera.view_proj * vec4(position, 1.0);

    gl_Position = camera.view_proj * pc.model * vec4(position, 1.0);
}