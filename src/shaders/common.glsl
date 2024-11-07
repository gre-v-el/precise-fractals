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