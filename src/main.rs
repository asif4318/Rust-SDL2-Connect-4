mod window;
mod gameloop;
mod texture_manager;
mod model;
mod textures;

extern crate sdl2;
use crate::gameloop::GameLoop;

fn main() {
    let mut game_loop = GameLoop::new().expect("Failed to initialize game loop");
    game_loop.run().expect("Startup Error!");
}
