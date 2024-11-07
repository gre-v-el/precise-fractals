use std::fs::read_to_string;

use egui_macroquad::macroquad::prelude::{gl_use_material, load_material, Material, MaterialParams, UniformType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MaterialsDescriptor {
    vertex_shader: String,
    fragments: Vec<(String, String)>, // name, path
    includes: Vec<(String, String)>,  // replace phrase, path
}

pub struct Materials {
    materials: Vec<(String, Material)>,
    current_material: usize,

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

impl Materials {
    pub fn load(path: &str) -> Self {
        let file = read_to_string(path).unwrap();
        let descriptor = ron::from_str::<MaterialsDescriptor>(&file).unwrap();

        let vertex_source = read_to_string(descriptor.vertex_shader).unwrap();

        // load includes
        let mut includes = Vec::new();
        for (name, path) in descriptor.includes {
            let source = read_to_string(path).unwrap();
            includes.push((name, source));
        }

        // load fragments and replace includes
        let mut fragments = Vec::new();
        for (name, path) in descriptor.fragments {
            let mut source = read_to_string(path).unwrap();
            
            for (phrase, lib) in &includes {
                if source.contains(phrase) {
                    source = source.replacen(phrase, lib, 1);
                    break;
                }
            }

            fragments.push((name, source));
        }

        // build materials
        let mut materials = Vec::new();
        for (name, source) in fragments {
            let material = load_material(
                &vertex_source, 
                &source, 
                MaterialParams {
                    uniforms: Materials::uniforms_descriptor(),
                    ..Default::default()
                }
            ).unwrap();
            materials.push((name, material));
        }

        Materials {
            materials,
            current_material: 0,

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

    fn uniforms_descriptor() -> Vec<(String, UniformType)> {
        vec![
            ("topLeft".into(), UniformType::Float2),
            ("bottomRight".into(), UniformType::Float2),
            ("picked".into(), UniformType::Float2),
            ("juliaInterpolation".into(), UniformType::Float1),
            ("iterations".into(), UniformType::Int1),
            ("power".into(), UniformType::Float1),
        ]
    }

    pub fn use_current(&self) {
        gl_use_material(self.materials[self.current_material].1);
    }

    pub fn activate_mandelbrot(&mut self) {
        let material = &mut self.materials[self.current_material].1;

        material.set_uniform("topLeft", self.top_left_m);
        material.set_uniform("bottomRight", self.bottom_right_m);
        material.set_uniform("picked", self.picked);
        material.set_uniform("juliaInterpolation", 0.0f32);
        material.set_uniform("iterations", self.iterations);
        material.set_uniform("power", self.power);
    }

    pub fn activate_julia(&mut self) {
        let material = &mut self.materials[self.current_material].1;

        material.set_uniform("topLeft", self.top_left_j);
        material.set_uniform("bottomRight", self.bottom_right_j);
        material.set_uniform("picked", self.picked);
        material.set_uniform("juliaInterpolation", self.julia_interpolation);
        material.set_uniform("iterations", self.iterations);
        material.set_uniform("power", self.power);
    }
}