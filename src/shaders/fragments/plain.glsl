uniform vec2 picked; 
uniform float juliaInterpolation;
uniform int iterations;
uniform float power;

varying vec2 UV;

$include_lib

void main() { 
	vec2 c = UV;
	c = mix(c, picked, juliaInterpolation); 
	vec2 z = UV;

	// iterate
	int i = 0;
	while(z.x*z.x + z.y*z.y < 4. && i < iterations){
		z = complexPow(z, power);
		z += c;
		i ++;
	}

	gl_FragColor = vec4(vec3(float(i)/float(iterations)), 1.);
}