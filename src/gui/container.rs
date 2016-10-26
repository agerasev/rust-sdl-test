use sdl2::render::{Renderer};
use sdl2::event::{Event};
use sdl2::rect::{Rect};

use gui::Element;

struct Frame {
	bounds: Rect,
	stretch: (f64, f64),
	element: Box<Element>,
}

pub struct Container {
	elements: Vec<Frame>,
	size_: (u32, u32), 
}

impl Container {
	pub fn new() -> Container {
		Container { 
			elements: Vec::<Frame>::new(), 
			size_: (0, 0), 
		}
	}

	pub fn add(&mut self, e: Box<Element>, s: (f64, f64)) {
		self.elements.push((Frame { 
			bounds: Rect::new(0, 0, 0, 0),
			stretch: s,
			element: e, 
		}));
	}

	fn update_sizes(&mut self) {
		let (w, h) = self.size_;
		let mut th = 0.0;
		for frame in &self.elements {
			th += frame.stretch.1;
		}
		let mut dh = 0.0;
		for frame in &mut self.elements {
			let ref mut elem = frame.element;
			let fh = (frame.stretch.1/th)*(h as f64);
			let ndh = dh + fh;
			let rh = ndh as u32 - dh as u32;
			frame.bounds = Rect::new(0, dh as i32, w, rh);
			elem.resize(w, rh);
			dh = ndh;
		}
	}
}

impl Element for Container {
	fn draw(&self, rend: &mut Renderer) {
		let vp = rend.viewport();
		for frame in &self.elements {
			let mut fvp = frame.bounds;
			fvp.offset(vp.x(), vp.y());
			rend.set_viewport(Some(fvp));
			frame.element.draw(rend);
		}
	}

	fn handle(&mut self, event: Event) {
		println!("container event");
	}

	fn size(&self) -> (u32, u32) {
		self.size_
	}
	fn resize(&mut self, w: u32, h: u32) {
		self.size_ = (w, h);
		self.update_sizes();
	}
}