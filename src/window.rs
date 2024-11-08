extern crate sdl2;
use sdl2::{Sdl};
use sdl2::render::Canvas;

pub struct Window {
    pub sdl_context: Sdl,
    // pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
}


impl Window {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("Rust Engine", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self { sdl_context, canvas })
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.clear();
    }
}