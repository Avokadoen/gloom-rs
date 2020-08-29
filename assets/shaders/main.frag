#version 430 core

uniform float elapsed = 0;

out vec4 color;

void main()
{
    float slow_elapse = elapsed * 0.4;
    float sin_elapsed = max(sin(slow_elapse), 0.05);
    float cos_elapsed = max(cos(slow_elapse), 0.05);
    color = vec4(sin_elapsed, cos_elapsed, sin_elapsed, 1.0f);
}