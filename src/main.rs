mod window;
mod gameloop;
mod texture_manager;
mod model;

extern crate sdl2;
use crate::gameloop::GameLoop;

fn main() {
    let mut game_loop = GameLoop::new().unwrap();
    game_loop.run().expect("Startup Error!");
}
