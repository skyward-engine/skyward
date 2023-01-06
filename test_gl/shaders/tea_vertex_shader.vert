#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 matrix;
uniform mat4 view;
uniform mat4 perspective;

void main(){
    mat4 modelview=view*matrix;
    
    gl_Position=perspective*modelview*vec4(position,1.);
    
    v_normal=transpose(inverse(mat3(modelview)))*normal;
    v_position=gl_Position.xyz/gl_Position.w;
}