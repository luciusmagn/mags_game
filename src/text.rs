use raylib::prelude::*;
use raylib::ease::Tween;

use std::fmt;

use crate::tick;
use crate::GAME;
use crate::component::Component;

pub struct Text {
	content:     String,
	color:       Color,
	x:           i32,
	y:           i32,
	font_size:   i32,
	alpha_tween: Option<Tween>,
	text_tween:  Option<Tween>,
	namespace:   &'static str,
	cursor:      bool,
	centered:    bool,
	next_ns:     &'static str,
	next_name:   String,
}

impl fmt::Debug for Text {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Text").finish()
	}
}
impl Text {
	pub fn new() -> Self {
		Self {
			content:     "<empty>".to_string(),
			color:       Color::WHITE,
			x:           10,
			y:           10,
			font_size:   20,
			alpha_tween: None,
			text_tween:  None,
			namespace:   "default",
			cursor:      false,
			centered:    false,
			next_ns:     "default",
			next_name:   String::new(),
		}
	}

	pub fn content<T: ToString>(mut self, src: T) -> Self {
		self.content = src.to_string();
		self
	}

	pub fn color(mut self, col: Color) -> Self {
		self.color = col;
		self
	}

	pub fn position(mut self, x: i32, y: i32) -> Self {
		self.x = x;
		self.y = y;
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

	pub fn centered(mut self, enabled: bool) -> Self {
		self.centered = enabled;
		self
	}

	pub fn next<T: ToString>(mut self, contents: T) -> Self {
		self.next_name = contents.to_string();
		self
	}
}

impl Component for Text {
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
	}

	fn update(&mut self, rl: &mut RaylibHandle) {
		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			let game = GAME.read().unwrap();
			game.swap_for(self.next_ns, &[&self.next_name]);
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

		if self.centered {
			x = self.x - width / 2;
			y = self.y - self.font_size / 2;
		} else {
			x = self.x;
			y = self.y;
		}

		d.draw_text(current_text, x, y, self.font_size, &cur_color);

		if self.cursor && tick() % 70 < 35 {
			d.draw_rectangle(
				self.x + width / 2 + 6,
				self.y - self.font_size / 2,
				self.font_size / 2,
				self.font_size,
				cur_color,
			);
		}
	}
}

pub(crate) mod preset {
	use raylib::prelude::*;
	use raylib::ease::{self, Tween};
	use super::Text;

	pub(crate) fn intro_style<T: ToString>(content: T) -> Text {
		let s = content.to_string();

		Text::new()
			.content(s)
			.font_size(20)
			.centered(true)
			.position(400, 300)
			.color(Color::WHITE)
			.text_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 1.5))
			.alpha_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 0.5))
			.cursor(true)
	}
}
