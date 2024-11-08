use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct TextureManager<'a> {
    pub texture: Option<Texture<'a>>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            texture: None,
            texture_creator,
        }
    }

    pub fn load_texture(&mut self, path: &Path) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(path)?;
        self.texture = Some(texture);
        Ok(())
    }

    pub fn render_texture(&self, canvas: &mut Canvas<sdl2::video::Window>, dest: sdl2::rect::Rect) -> Result<(), String> {
        if let Some(texture) = self.texture.as_ref() {
            canvas.copy(texture, None, dest)?;
            Ok(())
        } else {
            Err("Texture not found".to_string())
        }
    }
}