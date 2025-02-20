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

	// orbit traps
	float nearest = 10e5;
	float average = 0.;

	// iterate
	int i = 0;
	while( (z.x*z.x + z.y*z.y < 4. && i < iterations) || i < 1){
		z = complexPow(z, power);
		z += c;
		i ++;

		nearest = min(nearest, min(abs(z.x), abs(z.y)));
		average += min(abs(z.x), abs(z.y));
	}
	average /= float(i);

	vec3 nearestColor = vec3(0.7804, 0.9216, 0.0) * vec3(2.*(1.-pow(10.*nearest, 0.1)));
	vec3 averageColor = vec3(0.0745, 0.4392, 0.0) * vec3(pow(0.5*average, 0.5));
	vec3 basicColor   = vec3(0.0,    0.8824, 1.0) * vec3(float(i)/float(iterations));

	vec3 color = (nearestColor + averageColor + basicColor) * vec3(0.5);

	gl_FragColor = vec4(color, 1.);
}