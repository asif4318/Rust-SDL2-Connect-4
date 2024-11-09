use std::path::Path;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::texture_manager::TextureManager;
use crate::window::Window;
use crate::model::board::{Board, BoardTiles};

pub(crate) struct GameLoop {
    pub window: Window,
    pub board: Board,
}

impl GameLoop {
    pub fn new() -> Result<Self, String> {
        let window = Window::new()?;
        let board = Board {
            screen_area: Rect::new(0, 0, 800, 400),
            clear_color: Color::BLUE,
            tiles: BoardTiles::new()
        };
        Ok(GameLoop { window, board })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.handle_loop();
        Ok(())
    }

    fn handle_loop(&mut self) {
        let tex_creator = &self.window.canvas.texture_creator();

        let mut event_pump = self.window.sdl_context.event_pump()
            .expect("SDL event pump failed.");

        self.window.clear_canvas();

        'running: loop {
            self.window.clear_canvas();

            match self.handle_input(&mut event_pump) {
                true => {}
                false => { break 'running }
            };

            // The rest of the game loop goes here...
            self.window.canvas.set_draw_color(Color::BLACK);
            self.window.canvas.fill_rect(Rect::new(0, 0, 800, 600));
            self.render(tex_creator);
            self.window.canvas.present();
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

    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        let mut texture_manager = TextureManager::new(tex_creator);
        texture_manager.load_texture(Path::new("./assets/tile_empty.png")).expect("Failed to load texture");
        self.board.renderer(&mut self.window.canvas, &texture_manager);
        Ok(())
    }
}