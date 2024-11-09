extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window};
use crate::texture_manager::TextureManager;

pub enum TileState {
    EMPTY = 0,
    YELLOW = 1,
    RED = 2
}
impl Clone for TileState {
    fn clone(&self) -> Self {
        match self {
            TileState::EMPTY => TileState::EMPTY,
            TileState::RED  => TileState::RED,
            TileState::YELLOW => TileState::YELLOW,
        }
    }
}

pub struct BoardTiles {
    pub grid: Vec<Vec<TileState>>,
}

impl BoardTiles {
    pub fn new() -> Self {
        let grid = vec![vec![TileState::EMPTY; 7]; 6];
        BoardTiles { grid }
    }
}

pub struct Board {
    pub screen_area: Rect,
    pub clear_color: Color,
    pub tiles: BoardTiles,
}


impl Board {
    fn draw_tiles(&self, tex_manager: &TextureManager, canvas: &mut Canvas<Window>) {
        let mut position = (0,0);
        for row in &self.tiles.grid {
            for _ in row {
                tex_manager.render_texture(canvas, Rect::new(position.0, position.1, 100, 100));
                position.1 += 100;
            }
            position.0 += 100;
            position.1 = 0;
        }
    }
    pub fn renderer(&self, canvas: &mut Canvas<Window>, texture_manager: &TextureManager) {
        self.draw_tiles(texture_manager, canvas);
    }
}