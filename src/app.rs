use egui_macroquad::{egui::{DragValue, SidePanel, Slider}, macroquad::prelude::*};

use crate::{controls::Controls, helper::{complex_pow, draw_rect, lerp}, materials::Materials};

pub struct App {
    materials: Materials,
    controls_julia: Controls,
    controls_mandelbrot: Controls,
    bounds_julia: Rect,
    bounds_mandelbrot: Rect,

    // sample path
    sample_mode: bool,
    sample_path_in_mandelbrot: bool,
    sample_path_start: Vec2,
    sample_path_iterations: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            materials: Materials::load("./src/shaders/shaders.ron"),
            controls_julia:      Controls::new(vec2( -0.0, 0.0), Rect { x: 0.45, y: 0.0, w: 0.45, h: 1.0 }),
            controls_mandelbrot: Controls::new(vec2(-0.6, 0.0), Rect { x: 0.0, y: 0.0, w: 0.45, h: 1.0 }),
            bounds_julia: Rect { x: 0.5, y: 0.0, w: 0.5, h: 1.0 },
            bounds_mandelbrot: Rect { x: 0.0, y: 0.0, w: 0.5, h: 1.0 },

            sample_mode: false,
            sample_path_in_mandelbrot: true,
            sample_path_iterations: 30,
            sample_path_start: vec2(-0.61, -0.1),
        }
    }

    pub fn update_uniforms(&mut self) {
        let camera_j = self.controls_julia.camera();
        let camera_m = self.controls_mandelbrot.camera();

        self.materials.top_left_j = camera_j.screen_to_world(vec2(self.bounds_julia.left(), self.bounds_julia.top())).into();
        self.materials.bottom_right_j = camera_j.screen_to_world(vec2(self.bounds_julia.right(), self.bounds_julia.bottom())).into();

        self.materials.top_left_m = camera_m.screen_to_world(vec2(self.bounds_mandelbrot.left(), self.bounds_mandelbrot.top())).into();
        self.materials.bottom_right_m = camera_m.screen_to_world(vec2(self.bounds_mandelbrot.right(), self.bounds_mandelbrot.bottom())).into();

        if is_mouse_button_down(MouseButton::Left) {
            if self.sample_mode {
                if self.bounds_mandelbrot.contains(mouse_position().into()) {
                    let p = self.controls_mandelbrot.mouse_world.into();
                    self.sample_path_in_mandelbrot = true;
                    self.sample_path_start = p; 
                }

                else if self.bounds_julia.contains(mouse_position().into()) {
                    let p = self.controls_julia.mouse_world.into();
                    self.sample_path_in_mandelbrot = false;
                    self.sample_path_start = p;
                }
            }
            else {
                if self.bounds_mandelbrot.contains(mouse_position().into()) {
                    self.materials.picked = self.controls_mandelbrot.mouse_world.into();
                }
            }

        }
    }

    pub fn ui(&mut self) -> f32 {
        let picked_drag_speed = 0.003 / self.controls_mandelbrot.camera().zoom.x;
        let sample_drag_speed = 0.003 / (if self.sample_path_in_mandelbrot {self.controls_mandelbrot.camera()} else {self.controls_julia.camera()}).zoom.x;

        let mut available_width = 100.0;
        egui_macroquad::ui(|ctx| {
            SidePanel::right("sidepanel").default_width(200.0).show(ctx, |ui| {
                ui.separator();

                ui.collapsing("Rendering", |ui| {
                    ui.label("Iterations:");
                    ui.add(Slider::new(&mut self.materials.iterations, 10..=1000).logarithmic(true));
    
                    ui.add_space(10.0);
                    ui.label("Power:");
                    ui.add(Slider::new(&mut self.materials.power, -5.0..=5.0));
    
                    ui.add_space(10.0);
                    ui.label("Julia interpolation:");
                    ui.add(Slider::new(&mut self.materials.julia_interpolation, 0.0..=1.0));

                    ui.add_space(10.0);
                    ui.label("Picked:");
                    ui.horizontal(|ui| {
                        ui.label("Re(z):");
                        ui.add(DragValue::new(&mut self.materials.picked[0]).speed(picked_drag_speed));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Im(z):");
                        ui.add(DragValue::new(&mut self.materials.picked[1]).speed(picked_drag_speed));
                    });
                });
                
                ui.collapsing("Sampling", |ui| {
                    ui.checkbox(&mut self.sample_mode, "Sample mode");

                    ui.add_space(10.0);
                    ui.label("Iterations:");
                    ui.add(Slider::new(&mut self.sample_path_iterations, 2..=50));

                    ui.add_space(10.0);
                    ui.label("Sample in:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.sample_path_in_mandelbrot, true, "Mandelbrot");
                        ui.selectable_value(&mut self.sample_path_in_mandelbrot, false, "Julia");
                    });

                    ui.add_space(10.0);
                    ui.label("Sample point:");
                    ui.horizontal(|ui| {
                        ui.label("Re(z):");
                        ui.add(DragValue::new(&mut self.sample_path_start.x).speed(sample_drag_speed));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Im(z):");
                        ui.add(DragValue::new(&mut self.sample_path_start.y).speed(sample_drag_speed));
                    });
                });

                ui.add_space(20.0);
                if ui.button("Match cameras").clicked() {
                    let mut new_camera = self.controls_mandelbrot.camera().clone();
                    let displacement = new_camera.screen_to_world(self.bounds_mandelbrot.center()) - new_camera.screen_to_world(self.bounds_julia.center());
                    new_camera.target.x += displacement.x;
                    self.controls_julia.set_camera(new_camera);
                }

            });
            available_width = ctx.available_rect().width();
        });
        available_width
    }

    pub fn render(&mut self) {
        let available_width = self.ui();
        
        self.bounds_mandelbrot = Rect{ x: 0.0, y: 0.0, w: available_width*0.5, h: screen_height() };
        self.bounds_julia = Rect{ x: available_width*0.5, y: 0.0, w: available_width*0.5, h: screen_height() };

        self.controls_julia.update(&self.bounds_julia);
        self.controls_mandelbrot.update(&self.bounds_mandelbrot);

        self.update_uniforms();
        
        if self.sample_mode {
            if self.sample_path_in_mandelbrot {
                self.materials.use_current();
                self.materials.activate_mandelbrot();
                draw_rect(&self.bounds_mandelbrot, WHITE);
                
                gl_use_default_material();
                self.draw_path(self.controls_mandelbrot.camera(), 0.0);

                self.materials.use_current();
                self.materials.activate_julia();
                draw_rect(&self.bounds_julia, WHITE);
            }
            else {
                self.materials.use_current();
                self.materials.activate_julia();
                draw_rect(&self.bounds_julia, WHITE);

                gl_use_default_material();
                self.draw_path(self.controls_julia.camera(), self.materials.julia_interpolation);

                self.materials.use_current();
                self.materials.activate_mandelbrot();
                draw_rect(&self.bounds_mandelbrot, WHITE);
            }
            gl_use_default_material();
        }
        else {
            self.materials.use_current();
            self.materials.activate_mandelbrot();
            draw_rect(&self.bounds_mandelbrot, WHITE);

            self.materials.activate_julia();
            draw_rect(&self.bounds_julia, WHITE);

            gl_use_default_material();
            let picked = self.controls_mandelbrot.camera().world_to_screen(self.materials.picked.into());
            draw_circle_lines(picked.x, picked.y, 5.0, 2.5, GRAY);
        }

        draw_rectangle(self.bounds_mandelbrot.right()-2.0, self.bounds_mandelbrot.y, 4.0, self.bounds_mandelbrot.h, color_u8!(30, 30, 30, 255));
        egui_macroquad::draw();
    }

    fn draw_path(&self, camera: &Camera2D, julia_interpolation: f32) {
        let start = self.sample_path_start;

        let mut c = start;
        c = lerp(c, self.materials.picked.into(), julia_interpolation); 
        let mut z = start;

        let mut prev_screen = camera.world_to_screen(z);
        for _ in 0..self.sample_path_iterations {
            z = complex_pow(z, self.materials.power);
            z += c;

            let screen = camera.world_to_screen(z.into());
            draw_line(prev_screen.x, prev_screen.y, screen.x, screen.y, 2.0, RED);
            prev_screen = screen;
        }
    }
}