use egui_macroquad::{egui::{DragValue, SidePanel, Slider}, macroquad::prelude::*};

use crate::{controls::Controls, helper::{draw_rect, grow}};

pub struct Renderer {
    materials: Vec<(String, Material)>,
    current_material: usize,

    // sample path
    sample_mode: bool,
    sample_path_in_mandelbrot: bool,
    sample_path_start: Vec2,
    sample_path_iterations: usize,

    // uniforms
    pub top_left_m: [f32; 2],
    pub bottom_right_m: [f32; 2],
    pub top_left_j: [f32; 2],
    pub bottom_right_j: [f32; 2],

    pub picked: [f32; 2],
    pub julia_interpolation: f32,
    pub iterations: i32,
    pub power: f32,
}

impl Renderer {
    pub fn new() -> Self {
        let materials = vec![
            ("Orbit Traps".into(), load_material(
                include_str!("shader/vertex.glsl"),
                include_str!("shader/fragment.glsl"),
                MaterialParams {
                    uniforms: Renderer::uniforms_descriptor(),
                    ..Default::default()
                },
            ).unwrap())
        ];

        Renderer {
            materials,
            current_material: 0,

            sample_mode: false,
            sample_path_in_mandelbrot: true,
            sample_path_iterations: 30,
            sample_path_start: vec2(-0.61, -0.1),

            top_left_m: [-2.0, 2.0],
            bottom_right_m: [2.0, -2.0],
            top_left_j: [-2.0, 2.0],
            bottom_right_j: [2.0, -2.0],
            picked: [-1.0, 0.0],
            julia_interpolation: 1.0,
            iterations: 200,
            power: 2.0,
        }
    }

    pub fn uniforms_descriptor() -> Vec<(String, UniformType)> {
        vec![
            ("topLeft".into(), UniformType::Float2),
            ("bottomRight".into(), UniformType::Float2),
            ("picked".into(), UniformType::Float2),
            ("juliaInterpolation".into(), UniformType::Float1),
            ("isJulia".into(), UniformType::Float1),
            ("iterations".into(), UniformType::Int1),
            ("power".into(), UniformType::Float1),
        ]
    }

    pub fn activate_mandelbrot(&mut self) {
        let material = &mut self.materials[self.current_material].1;

        material.set_uniform("topLeft", self.top_left_m);
        material.set_uniform("bottomRight", self.bottom_right_m);
        material.set_uniform("picked", self.picked);
        material.set_uniform("juliaInterpolation", 0.0f32);
        material.set_uniform("iterations", self.iterations);
        material.set_uniform("power", self.power);
        material.set_uniform("isJulia", 0.0f32);
    }

    pub fn activate_julia(&mut self) {
        let material = &mut self.materials[self.current_material].1;

        material.set_uniform("topLeft", self.top_left_j);
        material.set_uniform("bottomRight", self.bottom_right_j);
        material.set_uniform("picked", self.picked);
        material.set_uniform("juliaInterpolation", self.julia_interpolation);
        material.set_uniform("iterations", self.iterations);
        material.set_uniform("power", self.power);
        material.set_uniform("isJulia", 1.0f32);
    }

    pub fn update_uniforms(&mut self, controls_julia: &Controls, controls_mandelbrot: &Controls, bounds_mandelbrot: &Rect, bounds_julia: &Rect) {
        let camera_j = controls_julia.camera();
        let camera_m = controls_mandelbrot.camera();

        self.top_left_j = camera_j.screen_to_world(vec2(bounds_julia.left(), bounds_julia.top())).into();
        self.bottom_right_j = camera_j.screen_to_world(vec2(bounds_julia.right(), bounds_julia.bottom())).into();

        self.top_left_m = camera_m.screen_to_world(vec2(bounds_mandelbrot.left(), bounds_mandelbrot.top())).into();
        self.bottom_right_m = camera_m.screen_to_world(vec2(bounds_mandelbrot.right(), bounds_mandelbrot.bottom())).into();

        if is_mouse_button_down(MouseButton::Left) {
            if self.sample_mode {
                if bounds_mandelbrot.contains(mouse_position().into()) {
                    let p = controls_mandelbrot.mouse_world.into();
                    self.sample_path_in_mandelbrot = true;
                    self.sample_path_start = p; 
                }

                else if bounds_julia.contains(mouse_position().into()) {
                    let p = controls_julia.mouse_world.into();
                    self.sample_path_in_mandelbrot = false;
                    self.sample_path_start = p;
                }
            }
            else {
                if bounds_mandelbrot.contains(mouse_position().into()) {
                    self.picked = controls_mandelbrot.mouse_world.into();
                }
            }

        }
    }

    pub fn ui(&mut self) -> f32 {
        let mut available_width = 100.0;
        egui_macroquad::ui(|ctx| {
            SidePanel::right("sidepanel").default_width(200.0).show(ctx, |ui| {
                ui.separator();

                ui.collapsing("Rendering", |ui| {
                    ui.label("Iterations:");
                    ui.add(Slider::new(&mut self.iterations, 10..=1000).logarithmic(true));
    
                    ui.add_space(10.0);
                    ui.label("Julia interpolation:");
                    ui.add(Slider::new(&mut self.julia_interpolation, 0.0..=1.0));
                });
                
                ui.collapsing("Sampling", |ui| {
                    ui.checkbox(&mut self.sample_mode, "Sample mode");

                    ui.add_space(10.0);
                    ui.label("Iterations:");
                    ui.add(Slider::new(&mut self.iterations, 2..=50));

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
                        ui.add(DragValue::new(&mut self.sample_path_start.x));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Im(z):");
                        ui.add(DragValue::new(&mut self.sample_path_start.y));
                    });
                });


            });
            available_width = ctx.available_rect().width();
        });
        available_width
    }

    pub fn render(&mut self, controls_julia: &Controls, controls_mandelbrot: &Controls, bounds_mandelbrot: &Rect, bounds_julia: &Rect) {
        gl_use_material(self.materials[self.current_material].1);

        self.activate_mandelbrot();
        draw_rect(bounds_mandelbrot, WHITE);

        self.activate_julia();
        draw_rect(bounds_julia, WHITE);

        gl_use_default_material();

        if self.sample_mode {
            let camera = if self.sample_path_in_mandelbrot {
                controls_mandelbrot.camera()
            }
            else {
                controls_julia.camera()
            };

            let start = camera.world_to_screen(self.sample_path_start.into());

        }
        else {
            let picked = controls_mandelbrot.camera().world_to_screen(self.picked.into());
            draw_circle_lines(picked.x, picked.y, 5.0, 2.5, GRAY);
        }
        egui_macroquad::draw();
    }
}