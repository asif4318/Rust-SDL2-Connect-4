use std::time::Duration;
use sdl2::event::Event;
use sdl2::{EventPump};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::window::Window;
use crate::model::board::{Board};
use crate::model::button::Button;
use crate::model::gamestate::{GameCondition, GameState};
use crate::model::tilestate::TileState;
use crate::textures::GameTextures;

pub(crate) struct Engine {
    pub window: Window,
    pub board: Board,
    pub game_state: GameState,
}

impl Engine {
    pub fn new(name: &str) -> Result<Self, String> {
        let window = Window::new(name.into())?;
        let board = Board::new();
        let game_state = GameState::new();
        Ok(Engine { window, board, game_state })
    }

    pub fn reset(&mut self) {
        self.game_state = GameState::new();
        self.board = Board::new();
        println!("Game Reset!");
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.handle_loop();
        Ok(())
    }

    fn handle_loop(&mut self) {
        let tex_creator = &self.window.canvas.texture_creator();
        let game_textures = GameTextures::new(tex_creator);
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let mut event_pump = self.window.sdl_context.event_pump()
            .expect("SDL event pump failed.");

        self.window.clear_canvas();

        let reset_rect = Rect::new(710, 100, 80, 80);
        let reset_button = Button::new("Reset", reset_rect, &ttf_context);

        // TODO: Clean up this font section and use more clear names
        let font = ttf_context.load_font("./assets/freedom.ttf", 30)
            .expect("Failed to load font");
        let surface = font.render(reset_button.text).blended(Color::WHITE).unwrap();
        let font_texture = tex_creator.create_texture_from_surface(surface)
            .expect("Failed to render font");

        let game_won_text = font.render("GAME WON!").blended(Color::WHITE).unwrap();
        let game_won_texture = tex_creator.create_texture_from_surface(game_won_text)
            .expect("Failed to render game won text");

        'running: loop {
            self.window.clear_canvas();

            // Break the loop if the input loop returns false after the user closes the program
            if !self.handle_input(&mut event_pump, &reset_button) { break 'running; }

            // The rest of the game loop goes here...
            self.window.canvas.clear();

            match self.game_state.game_condition {
                GameCondition::PLAYING => {
                    let _ = self.window.canvas.set_draw_color(Color::BLACK);
                    let _ = self.window.canvas.fill_rect(Rect::new(0, 0, 800, 600));
                    let _ = self.render(&game_textures);
                }
                GameCondition::ENDED => {
                    let mut draw_color = Color::WHITE;
                    match self.game_state.turn {
                        TileState::YELLOW => { draw_color = Color::YELLOW }
                        TileState::RED => { draw_color = Color::RED }
                        _ => {}
                    }
                    self.window.canvas.set_draw_color(draw_color);
                    let _ = self.window.canvas.fill_rect(Rect::new(0, 0, 800, 600));
                    self.window.canvas.copy(
                        &game_won_texture, None,
                        Rect::new(800 / 2 - 100, 600 / 2 - 100, 200, 200),
                    ).unwrap();
                }
            }
            reset_button.render(&mut self.window.canvas);

            self.window.canvas.copy(
                &font_texture, None,
                reset_button.rect,
            ).unwrap();

            self.window.canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn handle_input(&mut self, event_pump: &mut EventPump, reset_button: &Button) -> bool {
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
                    self.handle_click(mouse_btn, x, y, reset_button);
                }
                _ => {}
            }
        }
        true
    }

    fn handle_click(&mut self, mouse_btn: MouseButton, x: i32, y: i32, reset_button: &Button) {
        match mouse_btn {
            MouseButton::Left => {
                println!("x: {} y:{}", x, y);

                // Check if reset button
                if reset_button.is_clicked(x, y) { self.reset() }

                match self.board.insert_chip((x / 100) as u32, self.game_state.turn) {
                    Some(_) => {
                        if self.board.has_win {
                            self.game_state.game_won()
                        } else {
                            self.game_state.next_turn()
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
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