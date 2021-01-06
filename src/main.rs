use raylib::prelude::*;
use raylib::ffi::{MouseButton, IsMouseButtonDown};

fn main() {
	let (mut cells, mut tick) = ([[false; 60]; 80], 0);
	let (mut rl, thread) = raylib::init().size(800, 600).title("Borisův sand simulator").build();

	while !rl.window_should_close() {
		tick += 1;
		let (x, y) = ((rl.get_mouse_x() - (rl.get_mouse_x() % 10)) / 10, (rl.get_mouse_y() - (rl.get_mouse_y() % 10)) / 10);

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);

		cells.iter().enumerate()
			.map(|(i, x)| x.iter().map(move |y| (i, y)).enumerate())
			.flatten()
			.for_each(|(row, (col, c))| if *c {
				d.draw_rectangle((col * 10) as i32, (row * 10) as i32, 10,	10, Color::WHITE);
			});

		if unsafe { IsMouseButtonDown(MouseButton::MOUSE_LEFT_BUTTON as i32) } {
			cells[x as usize][y as usize] = true;
		}

		if tick % 20 == 0 {
			cells.clone().iter().enumerate()
				.map(|(i, x)| x.iter().map(move |y| (i, y)).enumerate())
				.flatten()
				.for_each(|(row, (col, c))| if *c && col < 79 && row < 59 {
					cells[col][row + 1] = true;
					cells[col][row] = false;
				});
		}
	}
}
