use std::path::Path;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::texture_manager::TextureManager;
use crate::window::Window;

pub(crate) struct GameLoop {
    pub window: Window,
}

impl GameLoop {
    pub fn new() -> Result<Self, String> {
        let window = Window::new()?;
        Ok(GameLoop { window })
    }

    pub fn handle_loop(&mut self) {
        let tex_creator = &self.window.canvas.texture_creator();

        let mut event_pump = self.window.sdl_context.event_pump()
            .expect("SDL event pump failed.");

        self.window.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            self.window.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            self.window.clear_canvas();

            match self.handle_input(&mut event_pump) {
                true => {}
                false => {break 'running}
            };

            // The rest of the game loop goes here...
            self.render(tex_creator);

            self.window.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn handle_input(&self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    println!("Pressed {key}!");
                }
                _ => {}
            }
        }
        return true;
    }

    pub fn render(&mut self, tex_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        let mut texture_manager = TextureManager::new(tex_creator);
        texture_manager.load_texture(Path::new("./assets/tile_hidden.png")).expect("Failed to load texture");
        texture_manager.render_texture(&mut self.window.canvas, Rect::new(0, 0, 100, 100))?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.handle_loop();
        Ok(())
    }
}