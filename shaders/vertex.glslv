#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in float v_light;

uniform mat4 model;
uniform mat4 projview;


out vec2 v_tex_coords;

out vec4 l_color;

void main() {
    v_tex_coords = tex_coords;
    l_color = vec4(v_light, v_light, v_light, 1.0);
    gl_Position = projview * model * vec4(position, 1.0);
}