attribute vec3 position;
attribute vec2 texcoord;

uniform vec2 topLeft;
uniform vec2 bottomRight;

varying vec2 UV;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
	UV = mix(topLeft, bottomRight, texcoord);
	gl_Position = Projection * Model * vec4(position.xy, 0.0, 1.0);
}