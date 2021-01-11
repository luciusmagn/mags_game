extern crate lazy_static;
extern crate raylib;
extern crate fnv;

use raylib::prelude::*;
use raylib::ease::{self, Tween};

use lazy_static::lazy_static;

use std::sync::RwLock;

mod text;
use text::preset;

mod game;
use game::Game;

mod two_choice;
use two_choice::TwoChoice;

mod component;

lazy_static! {
	pub(crate) static ref TICK: RwLock<usize> = RwLock::new(0);
	pub(crate) static ref GAME: RwLock<Game> = RwLock::new(Game::new());
}

pub fn tick() -> usize {
	let t = TICK.read().unwrap();
	*t
}

pub fn add_tick() {
	let mut t = TICK.write().unwrap();
	*t += 1;
}

fn main() {
	let (w, h) = (800, 600);
	let (mut rl, thread) = raylib::init().size(w, h).title("mag's Game").build();

	rl.set_target_fps(60);

	let game = GAME.read().unwrap();
	game.component(
		preset::intro_style("you awake in a strange world...")
			.next("or at least that's how you feel..."),
	)
	.component(
		preset::intro_style("or at least that's how you feel...").next("exit bed?"),
	)
	.component(
		TwoChoice::new()
			.content("exit bed?")
			.text_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 1.5))
			.alpha_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 0.5))
			.next_one("you exited the bed, nothing of interest happened...")
			.next_two(
				"you rolled over and went back to sleep, nothing of interest happened...",
			),
	)
	.component(preset::intro_style("you exited the bed, nothing of interest happened..."))
	.component(preset::intro_style(
		"you rolled over and went back to sleep, nothing of interest happened...",
	));

	while !rl.window_should_close() {
		add_tick();

		let game = GAME.read().unwrap();
		game.update(&mut rl);

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		game.draw(&mut d);
	}
}
