#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 matrix;
uniform mat4 view;
uniform mat4 perspective;

void main(){
    mat4 modelview=view*matrix;
    v_normal=transpose(inverse(mat3(modelview)))*normal;
    gl_Position=perspective*modelview*vec4(position,1.);
}