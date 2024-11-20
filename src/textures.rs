use std::path::Path;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::texture_manager::TextureManager;

pub struct GameTextures<'a> {
    pub(crate) tile_hidden: TextureManager<'a>,
    pub(crate) tile_yellow: TextureManager<'a>,
    pub(crate) tile_red: TextureManager<'a>,
}


impl<'a> GameTextures<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut tile_hidden = TextureManager::new(texture_creator);
        tile_hidden.load_texture(Path::new("./assets/tile_empty.png")).expect("Failed to load texture");
        let mut tile_red = TextureManager::new(texture_creator);
        tile_red.load_texture(Path::new("./assets/tile_red.png")).expect("Failed to load texture");
        let mut tile_yellow = TextureManager::new(texture_creator);
        tile_yellow.load_texture(Path::new("./assets/tile_yellow.png")).expect("Failed to load texture");

        Self { tile_hidden, tile_yellow, tile_red }
    }
}