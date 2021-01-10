use raylib::prelude::*;
use std::fmt::Debug;

pub trait Component: Debug {
	fn search(&self, namespace: &str, contents: &str) -> bool;
	fn draw(&mut self, d: &mut RaylibDrawHandle<'_>);
	fn update(&mut self, _rl: &mut RaylibHandle) {}
	fn reset(&mut self) {}
}
