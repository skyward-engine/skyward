#version 140

in vec3 position;
in vec3 normal;

uniform mat4 matrix;

void main(){
    gl_Position=matrix*vec4(position,1.);
}