use controls::Controls;
use egui_macroquad::{macroquad, macroquad::prelude::*};
use renderer::Renderer;

mod controls;
mod helper;
mod renderer;

#[macroquad::main("graph")]
async fn main() {
    let mut renderer = Renderer::new();

    let mut controls_julia = Controls::new(vec2(0.0, 0.0), Rect { x: 0.5, y: 0.0, w: 0.5, h: 1.0 });
    let mut controls_mandelbrot = Controls::new(vec2(-0.6, 0.0), Rect { x: 0.0, y: 0.0, w: 0.5, h: 1.0 });
    
    loop {
        clear_background(color_u8!(30, 30, 30, 255));
        
        let available_width = renderer.ui();
        
        let bounds_mandelbrout = Rect{ x: 0.0, y: 0.0, w: available_width*0.5, h: screen_height() };
        let bounds_julia = Rect{ x: available_width*0.5, y: 0.0, w: available_width*0.5, h: screen_height() };

        controls_julia.update(&bounds_julia);
        controls_mandelbrot.update(&bounds_mandelbrout);
        renderer.update_uniforms(&controls_julia, &controls_mandelbrot, &bounds_mandelbrout, &bounds_julia);
        
        renderer.render(&controls_julia, &controls_mandelbrot, &bounds_mandelbrout, &bounds_julia);

		next_frame().await
    }
}
