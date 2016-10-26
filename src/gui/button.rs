use sdl2::render::Renderer;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use gui::Element;


pub struct Button {
	size_: (u32, u32),
}

impl Button {
	pub fn new() -> Button {
		Button { size_: (0, 0) }
	}
}

impl Element for Button {
	fn draw(&self, rend: &mut Renderer) {
		rend.set_draw_color(Color::RGB(255, 0, 0));
		let (w, h) = self.size_;
		rend.draw_rect(Rect::new(0, 0, w, h)).unwrap();
	}

	fn handle(&mut self, event: Event) {
		println!("clicked");
	}

	fn size(&self) -> (u32, u32) {
		return self.size_
	}
	fn resize(&mut self, w: u32, h: u32) {
		self.size_ = (w, h);
	}
}
