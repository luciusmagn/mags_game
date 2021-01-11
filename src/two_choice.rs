use raylib::prelude::*;
use raylib::ease::Tween;

use std::fmt;

use crate::tick;
use crate::GAME;
use crate::component::Component;

pub struct TwoChoice {
	content:  String,
	option1:  String,
	option2:  String,
	selected: bool,

	color:        Color,
	font_size:    i32,
	alpha_tween:  Option<Tween>,
	text_tween:   Option<Tween>,
	select_tween: Tween,
	namespace:    &'static str,
	cursor:       bool,

	next_ns1:   &'static str,
	next_name1: String,
	next_ns2:   &'static str,
	next_name2: String,
}

impl fmt::Debug for TwoChoice {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("TwoChoice").finish()
	}
}
impl TwoChoice {
	pub fn new() -> Self {
		Self {
			content:  "<empty>".to_string(),
			option1:  "yes".to_string(),
			option2:  "no".to_string(),
			selected: true,

			color:        Color::WHITE,
			font_size:    20,
			alpha_tween:  None,
			text_tween:   None,
			select_tween: Tween::new(ease::cubic_in, 0.0, 1.0, 1.5),
			namespace:    "default",
			cursor:       false,

			next_ns1:   "default",
			next_name1: String::new(),
			next_ns2:   "default",
			next_name2: String::new(),
		}
	}

	pub fn options<T: ToString>(mut self, first: T, second: T) -> Self {
		self.option1 = first.to_string();
		self.option2 = second.to_string();
		self
	}

	pub fn selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	pub fn next_one<T: ToString>(mut self, contents: T) -> Self {
		self.next_name1 = contents.to_string();
		self
	}

	pub fn next_two<T: ToString>(mut self, contents: T) -> Self {
		self.next_name2 = contents.to_string();
		self
	}

	pub fn content<T: ToString>(mut self, src: T) -> Self {
		self.content = src.to_string();
		self
	}

	pub fn color(mut self, col: Color) -> Self {
		self.color = col;
		self
	}

	pub fn font_size(mut self, size: i32) -> Self {
		self.font_size = size;
		self
	}

	pub fn alpha_tween(mut self, tween: Tween) -> Self {
		self.alpha_tween = Some(tween);
		self
	}

	pub fn text_tween(mut self, tween: Tween) -> Self {
		self.text_tween = Some(tween);
		self
	}

	pub fn cursor(mut self, enabled: bool) -> Self {
		self.cursor = enabled;
		self
	}
}

impl Component for TwoChoice {
	fn search(&self, namespace: &str, content: &str) -> bool {
		self.namespace == namespace && self.content == content
	}

	fn reset(&mut self) {
		if let Some(tween) = &mut self.alpha_tween {
			tween.reset();
		}
		if let Some(tween) = &mut self.text_tween {
			tween.reset();
		}
		self.select_tween.reset();
	}

	fn update(&mut self, rl: &mut RaylibHandle) {
		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			let game = GAME.read().unwrap();
			if self.selected {
				game.swap_for(self.next_ns1, &[&self.next_name1]);
			} else {
				game.swap_for(self.next_ns2, &[&self.next_name2]);
			}
		}

		if rl.is_key_pressed(KeyboardKey::KEY_LEFT)
			|| rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
		{
			self.selected = !self.selected;
		}
	}

	fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
		let current_text = match &mut self.text_tween {
			Some(ref mut t) =>
				&self.content[..(self.content.len() as f32 * t.apply(0.01)) as usize],
			None => &self.content,
		};

		let cur_color = match &mut self.alpha_tween {
			Some(ref mut t) => {
				let mut c = self.color;
				c.a = (c.a as f32 * t.apply(0.01)) as u8;
				c
			}
			None => self.color,
		};

		let (x, y);
		let width = measure_text(current_text, self.font_size);

		x = 400 - width / 2;
		y = 200 - self.font_size / 2;

		d.draw_text(current_text, x, y, self.font_size, &cur_color);

		d.draw_text(&self.option1, 200, 450, self.font_size, &cur_color);
		d.draw_text(&self.option2, 600, 450, self.font_size, &cur_color);

		if self.selected {
			d.draw_rectangle(
				200,
				450 + self.font_size + 3,
				measure_text(&self.option1, self.font_size),
				4,
				cur_color,
			);
		} else {
			d.draw_rectangle(
				600,
				450 + self.font_size + 3,
				measure_text(&self.option2, self.font_size),
				4,
				cur_color,
			);
		}

		if self.cursor && tick() % 70 < 35 {
			d.draw_rectangle(
				400 + width / 2 + 6,
				200 - self.font_size / 2,
				self.font_size / 2,
				self.font_size,
				cur_color,
			);
		}
	}
}
