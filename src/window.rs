extern crate sdl2;
use sdl2::{Sdl};
use sdl2::render::Canvas;

pub struct Window {
    pub sdl_context: Sdl,
    // pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
}


impl Window {
    pub fn new(name: &str) -> Result<Self, String> {
        let sdl_context = sdl2::init()
            .expect("Failed to init SDL");
        let video_subsystem = sdl_context.video()
            .expect("Failed to init video subsystem");

        let window = video_subsystem.window(name.into(), 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to build window");

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self { sdl_context, canvas})
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.clear();
    }
}