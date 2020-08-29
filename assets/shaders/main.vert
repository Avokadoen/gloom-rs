#version 430 core

uniform mat4 projection;
uniform mat4 c_trans;

in vec3 position;

void main()
{
    gl_Position = projection * (c_trans * vec4(position, 1.0f));
}