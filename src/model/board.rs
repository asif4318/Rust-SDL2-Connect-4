extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use crate::model::tilestate::TileState;
use crate::texture_manager::TextureManager;

pub struct BoardTiles {
    pub grid: Vec<Vec<TileState>>,
}

impl BoardTiles {
    pub fn new() -> Self {
        let grid = vec![vec![TileState::EMPTY; 7]; 6];
        BoardTiles { grid }
    }
    fn size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }
}

pub struct Board {
    pub tiles: BoardTiles,
    pub has_win: bool,
}

impl Board {
    fn draw_tiles(&self, tile_hidden: &TextureManager, tile_red: &TextureManager, tile_yellow: &TextureManager, canvas: &mut Canvas<Window>) {
        let (x, y) = self.tiles.size();
        for i in 0..x {
            for j in 0..y {
                match self.tiles.grid[i][j] {
                    TileState::EMPTY => {
                        let _ = tile_hidden.render_texture(canvas, Rect::new((j * 100) as i32, (i * 100) as i32, 100, 100));
                    }
                    TileState::YELLOW => {
                        let _ = tile_yellow.render_texture(canvas, Rect::new((j * 100) as i32, (i * 100) as i32, 100, 100));
                    }
                    TileState::RED => {
                        let _ = tile_red.render_texture(canvas, Rect::new((j * 100) as i32, (i * 100) as i32, 100, 100));
                    }
                }
            }
        }
    }

    fn get_grid(&mut self) -> &mut Vec<Vec<TileState>> {
        &mut self.tiles.grid
    }

    fn count_in_direction(&self, position: (usize, usize), dr: isize, dc: isize, color: TileState) -> usize {
        let mut count = 0;
        let (mut row, mut col) = position;

        // Keeps counting unless we hit the end of the board or a tile not of the desired color
        loop {
            let next_row = row as isize + dr;
            let next_col = col as isize + dc;

            if next_row < 0
                || next_row >= self.tiles.grid.len() as isize
                || next_col < 0
                || next_col >= self.tiles.grid[0].len() as isize
            {
                break;
            }

            row = next_row as usize;
            col = next_col as usize;

            if self.tiles.grid[row][col] == color {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    fn check_win(&mut self, tile_position: (usize, usize), color: TileState) {
        let directions = [
            (0, 1),  // Horizontal right
            (1, 0),  // Vertical down
            (1, 1),  // Diagonal down-right
            (1, -1), // Diagonal down-left
        ];

        for &(dr, dc) in &directions {
            let mut count = 1;

            // Check one direction
            count += self.count_in_direction(tile_position, dr, dc, color);
            // Check the opposite direction
            count += self.count_in_direction(tile_position, -dr, -dc, color);

            if count >= 4 {
                self.has_win = true; // Found a winning line
                return;
            }
        }

        self.has_win = false;
        return;
    }

    pub fn insert_chip(&mut self, column: u32, color: TileState) -> Option<(usize, usize)> {
        let (rows, cols) = self.tiles.size();

        // Check if col is within bounds
        if column >= cols as u32 {
            println!("Out of bounds!");
            return None;
        }

        let ref_grid: &mut Vec<Vec<TileState>> = self.get_grid();

        let mut i = rows;
        while i != 0 {
            i -= 1;
            match ref_grid[i][column as usize] {
                TileState::EMPTY => {
                    println!("This row {} and this column {} is empty", i, column);
                    ref_grid[i][column as usize] = color;
                    let coordinates = (i, column as usize);
                    self.check_win(coordinates, color);
                    println!("Has win: {}", self.has_win);
                    return Some(coordinates);
                }
                _ => {}
            }
        }

        None
    }

    pub fn renderer(&self, canvas: &mut Canvas<Window>, tile_hidden: &TextureManager, tile_yellow: &TextureManager, tile_red: &TextureManager) {
        self.draw_tiles(tile_hidden, tile_red, tile_yellow, canvas);
    }
}