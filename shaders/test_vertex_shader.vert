#version 140

in vec3 position;
in vec2 tex_pos;
out vec2 v_tex_pos;
uniform mat4 matrix;

// in vec3 color;
// out vec3 vColor;

void main(){
    v_tex_pos=tex_pos;
    gl_Position=matrix*vec4(position,1.);
    // vColor = color;
}