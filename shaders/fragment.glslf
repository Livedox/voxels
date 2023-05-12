#version 330 core

in vec2 v_tex_coords;
in vec4 l_color;
uniform vec4 a_color;
out vec4 f_color;

uniform sampler2D u_texture0;

void main() {
    f_color = texture(u_texture0, v_tex_coords) * l_color * a_color;
}