attribute vec3 position;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main() {
    gl_Position = u_model * u_view * u_model * vec4(position, 1.0);
}