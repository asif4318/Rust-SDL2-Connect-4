use sdl2::event::Event;
use sdl2::{EventPump};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::window::Window;
use crate::model::board::{Board, BoardTiles};
use crate::model::gamestate::GameState;
use crate::textures::GameTextures;

pub(crate) struct GameLoop {
    pub window: Window,
    pub board: Board,
    pub game_state: GameState,
}

impl GameLoop {
    pub fn new() -> Result<Self, String> {
        let window = Window::new()?;
        let board = Board {
            tiles: BoardTiles::new(),
            has_win: false,
        };
        let game_state = GameState::new();
        Ok(GameLoop { window, board, game_state })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.handle_loop();
        Ok(())
    }

    fn handle_loop(&mut self) {
        let tex_creator = &self.window.canvas.texture_creator();
        let game_textures = GameTextures::new(tex_creator);

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
            let _ = self.render(&game_textures);
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
                            match self.board.insert_chip((mouse_position.0 / 100) as u32, self.game_state.turn) {
                                None => {}
                                Some(_) => { self.game_state.next_turn() }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        return true;
    }

    fn render(&mut self, game_textures: &GameTextures) -> Result<(), String> {
        self.board.renderer(
            &mut self.window.canvas,
            &game_textures.tile_hidden,
            &game_textures.tile_yellow,
            &game_textures.tile_red,
        );
        Ok(())
    }
}