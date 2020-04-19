precision mediump float;

uniform vec4 u_color;
uniform float u_fade;

void main() {
    gl_FragColor = u_color * u_fade;
}