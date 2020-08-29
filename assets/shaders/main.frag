#version 430 core

uniform float elapsed = 0;

out vec4 color;

void main()
{
    float slowElapse = elapsed * 0.4;
    float sinElapsed = max(sin(slowElapse), 0.05);
    float cosElapsed = max(cos(slowElapse), 0.05);
    color = vec4(sinElapsed, cosElapsed, sinElapsed, 1.0f);
}