use sdl2::render::Renderer;
use sdl2::event::Event;

pub trait Element {
	fn size(&self) -> (u32, u32);
	fn resize(&mut self, w: u32, h: u32);

	fn draw(&self, rend: &mut Renderer);
	fn handle(&mut self, event: Event);
}