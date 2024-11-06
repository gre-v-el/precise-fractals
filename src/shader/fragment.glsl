// per frame variables
uniform vec2 picked; 
uniform float juliaInterpolation;
uniform bool isJulia;
uniform int iterations;
uniform float power;
uniform vec2 topLeft;
uniform vec2 bottomRight;

// per pixel variables
varying vec2 UV;

float distSq(vec2 a, vec2 b){
	return (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y); 
}
float distSq(vec2 a){
	return a.x * a.x + a.y * a.y; 
}

vec2 complexPow(vec2 c, float power){
	float r = distSq(c);
	float theta = atan(c.y, c.x);

	r = pow(r, power/2.);
	theta *= power;

	return vec2(r*cos(theta), r*sin(theta));
}

void main() { 
	// define initial numbers
	// juliaInt == 0: mandelbrot | juliaInt == 1: julia | inbetween gives different results
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