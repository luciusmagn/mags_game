use raylib::prelude::*;
use raylib::ease::{self, Tween};
use raylib::ffi::{IsKeyPressed, KeyboardKey};

use std::cmp::min;

mod text;
use text::Text;

fn main() {
	let (w, h) = (800, 600);
	let mut tick = 0;
	let (mut rl, thread) = raylib::init()
		.size(w, h)
		.title("mag's Game")
		.build();

	rl.set_target_fps(60);

	let mut text_index = 0;
	let texts = [
		"you awake in a strange world...",
		"or at least that's how you feel...",
		"leave bed?",
	];

	let mut text = Text::new()
		.content(texts[text_index])
		.font_size(20)
		.centered(true)
		.position(w / 2, h / 2)
		.color(Color::WHITE)
		.text_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 1.5))
		.alpha_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 0.5))
		.cursor(true);

	while !rl.window_should_close() {
		tick += 1;

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		text.draw(&mut d);

		if unsafe { IsKeyPressed(KeyboardKey::KEY_SPACE as i32) } {
			text_index = min(texts.len() - 1, text_index + 1);

			text = Text::new()
				.content(texts[text_index])
				.font_size(20)
				.centered(true)
				.position(w / 2, h / 2)
				.color(Color::WHITE)
				.text_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 1.5))
				.alpha_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 0.5))
				.cursor(true);
		}
	}
}
