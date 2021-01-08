use raylib::prelude::*;
use raylib::ffi::{MouseButton, IsMouseButtonDown};

fn main() {
	let (w, h) = (800, 600);
	let mut tick = 0;
	let (mut rl, thread) = raylib::init()
		.size(w, h)
		.title("mag's Game")
		.build();
	rl.set_target_fps(60);

	while !rl.window_should_close() {
		tick += 1;

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);

		let width = measure_text("you awake in a strange land...", 20);
		d.draw_text("you awake in a strange land...",  w / 2 - width / 2, h / 2 - 5, 20, Color::WHITE);
	}
}
