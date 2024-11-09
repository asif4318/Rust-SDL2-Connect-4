use std::path::Path;
use sdl2::event::Event;
use sdl2::{EventPump};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator};
use sdl2::video::WindowContext;
use crate::texture_manager::TextureManager;
use crate::window::Window;
use crate::model::board::{Board, BoardTiles};
use crate::model::tilestate::TileState;

pub(crate) struct GameLoop {
    pub window: Window,
    pub board: Board,
}

impl GameLoop {
    pub fn new() -> Result<Self, String> {
        let window = Window::new()?;
        let board = Board {
            tiles: BoardTiles::new(),
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
            let _ = self.window.canvas.set_draw_color(Color::BLACK);
            let _ = self.window.canvas.fill_rect(Rect::new(0, 0, 800, 600));
            self.render(tex_creator);
            self.window.canvas.present();
        }
    }

    fn handle_input(&mut self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    println!("Pressed {key}!");
                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    let mouse_position = (x, y);
                    match mouse_btn {
                        MouseButton::Left => {
                            println!("x: {} y:{}", mouse_position.0, mouse_position.1);
                            self.board.insert_chip((mouse_position.0 / 100) as u32, TileState::RED);
                            //self.board.insert_chip(TileState::YELLOW, mouse_position.0/100);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        return true;
    }

    fn render(&mut self, tex_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        let mut tile_hidden = TextureManager::new(tex_creator);
        tile_hidden.load_texture(Path::new("./assets/tile_empty.png")).expect("Failed to load texture");
        let mut tile_red = TextureManager::new(tex_creator);
        tile_red.load_texture(Path::new("./assets/tile_red.png")).expect("Failed to load texture");
        let mut tile_yellow = TextureManager::new(tex_creator);
        tile_yellow.load_texture(Path::new("./assets/tile_yellow.png")).expect("Failed to load texture");

        self.board.renderer(&mut self.window.canvas, &tile_hidden, &tile_yellow, &tile_red);
        Ok(())
    }
}