#version 460



layout(location = 0)
in vec3 v_normal;

layout(location = 0)
out vec4 f_color;

layout(push_constant)
uniform PushConstantData {
    mat4 mvp;
    mat4 model;
    vec4 light_dir;
    vec4 model_colour;
    } pc;

void main() {
    vec3 normal = normalize(v_normal);

    vec3 light_dir = normalize(pc.light_dir.xyz);

    float diffuse = max(dot(normal, light_dir), 0.0);

    vec3 ambient = pc.model_colour.rgb * 0.1;

    vec3 final_color = ambient + (pc.model_colour.rgb * diffuse);

    f_color = vec4(final_color, pc.model_colour.a);
    }