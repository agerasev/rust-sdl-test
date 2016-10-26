extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use sdl2::pixels::{Color};

mod gui;
use gui::{Element, Button};

fn main() {
	let ctx = sdl2::init().unwrap();
	let video_ctx = ctx.video().unwrap();
	
	let window  = match video_ctx.window("SDL2", 800, 600).position_centered().opengl().build() {
		Ok(window) => window,
		Err(err)   => panic!("failed to create window: {}", err)
	};

	let mut renderer = match window.renderer().build() {
		Ok(renderer) => renderer,
		Err(err) => panic!("failed to create renderer: {}", err)
	};

	let mut cont = gui::Container::new();
	cont.add(Box::<Button>::new(gui::Button::new()), (1.0, 1.0));
	cont.add(Box::<Button>::new(gui::Button::new()), (1.0, 1.5));
	cont.resize(400, 200);

	let mut events = ctx.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				Event::KeyDown{keycode, ..} => 
					if keycode.unwrap() == Keycode::Escape { break 'main; },
				_ => continue,
			}
		}

		renderer.set_draw_color(Color::RGB(0, 0, 0));
		renderer.clear();

		renderer.set_viewport(Some(Rect::new(200, 200, cont.size().0, cont.size().1)));
		cont.draw(&mut renderer);

		renderer.present();
	}
}
