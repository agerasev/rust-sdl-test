use sdl2::render::{Renderer, Texture};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2_ttf::Font;

use gui::Element;


pub struct Button {
	texture: Texture,
	size: (u32, u32),
}

impl Button {
	pub fn new(tex: Texture) -> Button {
		Button { texture: tex, size: (0, 0) }
	}
}

impl Element for Button {
	fn draw(&self, rend: &mut Renderer) {
		rend.set_draw_color(Color::RGB(255, 0, 0));
		let (w, h) = self.size;
		rend.draw_rect(Rect::new(0, 0, w, h)).unwrap();
		rend.copy(&self.texture, None, None);
	}

	fn handle(&mut self, event: Event) {
		println!("clicked");
	}

	fn size(&self) -> (u32, u32) {
		return self.size
	}
	fn resize(&mut self, w: u32, h: u32) {
		self.size = (w, h);
	}
}
