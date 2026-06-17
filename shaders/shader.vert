#version 460



layout(location = 0)
in vec3 position;
layout(location = 1)
in vec3 normal;

layout(location = 0)
out vec3 v_normal;

layout(push_constant)
uniform PushConstantData {
    mat4 mvp;
    mat4 model;
    vec4 light_dir;
    vec4 model_colour;
    } pc;

void main() {
    gl_Position = pc.mvp * vec4(position, 1.0);

    v_normal = mat3(pc.model) * normal;
    }