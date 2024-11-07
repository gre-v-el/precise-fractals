use egui_macroquad::{macroquad, macroquad::prelude::*};
use app::App;

mod materials;
mod controls;
mod helper;
mod app;

#[macroquad::main("graph")]
async fn main() {
    let mut app = App::new();
    
    loop {
        clear_background(color_u8!(30, 30, 30, 255));
        
        app.render();

		next_frame().await
    }
}
