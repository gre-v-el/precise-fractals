use egui_macroquad::macroquad::prelude::*;
use crate::helper::*;

pub struct Controls {
	target: Camera2D,
	camera: Camera2D,
	pub mouse_world: Vec2,
	last_mouse_world: Vec2,
	pub drag: Vec2,
}

impl Controls {
	pub fn new(target: Vec2, bounds: Rect) -> Self {
		let mut camera = Camera2D {
			target,
			zoom: vec2(0.2, -0.2 * screen_width()/screen_height()),
			..Default::default()
		};
		let center = camera.screen_to_world(vec2(screen_width()*bounds.center().x, screen_height()*bounds.center().y));
		camera.target -= center;

		Controls {
			target: camera.clone(),
			camera,
			mouse_world: vec2(0.0, 0.0),
			last_mouse_world: vec2(0.0, 0.0),
			drag: vec2(0.0, 0.0),
		}
	}

	pub fn update(&mut self, bounds: &Rect) {

		let mouse_screen: Vec2 = mouse_position().into();
		self.mouse_world = self.target.screen_to_world(mouse_screen);
		
		let active = bounds.contains(mouse_screen);

		let (_, d_zoom) = 
			if !active {
				(0.0, 0.0)
			} else {
				mouse_wheel()
			};

		if d_zoom != 0.0 {
			self.target.target = self.mouse_world;

			self.target.zoom.x = self.target.zoom.x * 1.1f32.powf(d_zoom);
			self.target.zoom.y = -self.target.zoom.x * screen_width() / screen_height();

			let mouse_world = self.target.screen_to_world(mouse_screen);

			self.target.target += self.target.target - mouse_world;
		}
		else {
			self.target.zoom.y = -self.target.zoom.x * screen_width() / screen_height();
		}

		if active && is_mouse_button_down(MouseButton::Right) {
			self.target.target -= self.mouse_world - self.last_mouse_world;
		}
		
		self.camera.target = lerp(self.camera.target, self.target.target, 1.0 - 0.1f32.powf(10.0*get_frame_time()));
		self.camera.zoom =   lerp(self.camera.zoom,   self.target.zoom,   1.0 - 0.1f32.powf(10.0*get_frame_time()));

		self.drag = self.mouse_world - self.last_mouse_world;
        self.last_mouse_world = self.target.screen_to_world(mouse_screen);
	}

	pub fn camera(&self) -> &Camera2D {
		&self.camera
	}

	pub fn set_camera(&mut self, camera: Camera2D) {
		self.target = camera;
	}
}