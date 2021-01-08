use raylib::prelude::*;
use raylib::ease::Tween;

pub struct Text {
	content: String,
	color: Color,
	x: i32,
	y: i32,
	font_size: i32,
	alpha_tween: Option<Tween>,
	text_tween: Option<Tween>,
	cursor: bool,
	tick: usize,
	centered: bool,
}

impl Text {
	pub fn new() -> Self {
		Self {
			content: "<empty>".to_string(),
			color: Color::WHITE,
			x: 10,
			y: 10,
			font_size: 20,
			alpha_tween: None,
			text_tween: None,
			cursor: false,
			tick: 0,
			centered: false,
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

	pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
		self.tick += 1;

		let current_text = match &mut self.text_tween {
			Some(ref mut t) => &self.content[..(self.content.len() as f32 * t.apply(0.01)) as usize],
			None => &self.content,
		};

		let cur_color = match &mut self.alpha_tween {
			Some(ref mut t) => {
				let mut c = self.color.clone();
				c.a = (c.a as f32 * t.apply(0.01)) as u8;
				c
			},
			None => self.color.clone(),
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

		if self.cursor && self.tick % 70 < 35 {
			d.draw_rectangle(self.x + width / 2 + 6, self.y - self.font_size / 2, self.font_size / 2, self.font_size, cur_color);
		}
	}
}
