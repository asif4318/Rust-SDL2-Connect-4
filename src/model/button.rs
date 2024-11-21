use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

pub struct Button<'a> {
    pub text: &'a str,
    pub rect: Rect,
    ttf_context: &'a Sdl2TtfContext,
}

impl Button<'_> {
    pub fn new<'a>(text: &'a str, rect: Rect, sdl2ttf_context: &'a Sdl2TtfContext) -> Box<Button<'a>> {
        let ttf_context = sdl2ttf_context;
        Box::new(Button {text: text.into(), rect, ttf_context})
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::CYAN);
        canvas.fill_rect(self.rect).expect("Couldn't draw button");
    }
}