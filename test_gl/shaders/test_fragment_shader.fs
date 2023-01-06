#version 140

in vec2 v_tex_pos;
out vec4 color;

uniform sampler2D tex;

// in vec3 vColor;
// out vec4 f_color;

void main(){
    color=texture(tex,v_tex_pos);
    // f_color = vec4(vColor, 1.0);
}