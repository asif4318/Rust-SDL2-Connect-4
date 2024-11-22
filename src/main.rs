mod window;
mod engine;
mod texture_manager;
mod model;
mod textures;

extern crate sdl2;
use crate::engine::Engine;

fn main() {
    let mut game_engine = Engine::new("Connect 4 - Rust SDL2")
        .expect("Failed to initialize game loop");
    game_engine.run().expect("Startup Error!");
}
