use std::ops::{Mul, Add};

use egui_macroquad::macroquad::{color::Color, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

pub fn lerp<T>(a: T, b: T, t: f32) -> T 
where 
	f32 : Mul<T, Output = T>,
	T : Add<Output = T>,
{
	t * b + (1.0 - t) * a
}

pub fn draw_rect(rect: &Rect, color: Color) {
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

pub fn complex_pow(c: Vec2, power: f32) -> Vec2{
	let r = c.length_squared();
	let mut theta = f32::atan2(c.y, c.x);

	let r = r.powf(power/2.0);
	theta *= power;

	return vec2(r*theta.cos(), r*theta.sin());
}