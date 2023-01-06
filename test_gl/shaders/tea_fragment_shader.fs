#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light;

const vec3 ambient_color=vec3(.2,0.,0.);
const vec3 diffuse_color=vec3(.6,0.,0.);
const vec3 specular_color=vec3(1.,1.,1.);

void main(){
    float diffuse=max(dot(normalize(v_normal),normalize(u_light)),0.);
    
    vec3 camera_dir=normalize(-v_position);
    vec3 half_direction=normalize(normalize(u_light)+camera_dir);
    float specular=pow(max(dot(half_direction,normalize(v_normal)),0.),16.);
    
    color=vec4(ambient_color+diffuse*diffuse_color+specular*specular_color,1.);
}