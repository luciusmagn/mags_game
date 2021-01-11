extern crate lazy_static;
extern crate raylib;
extern crate fnv;

use raylib::prelude::*;
use raylib::ease::{self, Tween};

use fnv::FnvHashMap as HashMap;
use lazy_static::lazy_static;

use std::marker::PhantomPinned;
use std::ptr::NonNull;
use std::sync::RwLock;

mod text;
use text::preset;

mod two_choice;
use two_choice::TwoChoice;

mod component;
use component::Component;

pub(crate) enum Command {
	SwapFor(usize, Vec<usize>),
	Remove(Vec<usize>),
	Add(Vec<usize>),
}

pub(crate) struct Game {
	world:      RwLock<HashMap<usize, Box<dyn Component>>>,
	stage:      RwLock<HashMap<usize, NonNull<Box<dyn Component>>>>,
	commands:   RwLock<Vec<Command>>,
	current_id: RwLock<usize>,
	count:      RwLock<usize>,
	_pin:       PhantomPinned,
}

unsafe impl Send for Game {}
unsafe impl Sync for Game {}

impl Game {
	fn new() -> Self {
		Self {
			world:      RwLock::new(HashMap::default()),
			stage:      RwLock::new(HashMap::default()),
			commands:   RwLock::new(vec![]),
			current_id: RwLock::new(0),
			count:      RwLock::new(0),
			_pin:       PhantomPinned,
		}
	}

	fn search(&self, namespace: &str, contents: &str) -> Option<usize> {
		let world = self.world.read().unwrap();

		world
			.iter()
			.find(|(_, v)| v.search(namespace, contents))
			.and_then(|x| Some(*x.0))
	}

	pub fn add(&self, namespace: &str, contents: &[&str]) -> Option<()> {
		let mut commands = self.commands.write().unwrap();

		let ids =
			contents.iter().filter_map(|x| self.search(namespace, x)).collect::<Vec<_>>();
		commands.push(Command::Add(ids));
		Some(())
	}

	pub fn remove(&self, namespace: &str, contents: &[&str]) -> Option<()> {
		let mut commands = self.commands.write().unwrap();
		let ids =
			contents.iter().filter_map(|x| self.search(namespace, x)).collect::<Vec<_>>();
		commands.push(Command::Remove(ids));
		Some(())
	}

	pub fn swap_for(&self, namespace: &str, contents: &[&str]) -> Option<()> {
		let current_id = self.current_id.read().unwrap();
		let mut commands = self.commands.write().unwrap();
		let ids =
			contents.iter().filter_map(|x| self.search(namespace, x)).collect::<Vec<_>>();
		commands.push(Command::SwapFor(*current_id, ids));
		Some(())
	}

	pub fn component<T: Component + 'static>(&self, component: T) -> &Self {
		let mut count = self.count.write().unwrap();
		let mut world = self.world.write().unwrap();

		world.insert(*count, Box::new(component));

		if *count == 0 {
			let mut commands = self.commands.write().unwrap();
			commands.push(Command::Add(vec![0]));
		}

		*count += 1;
		self
	}

	pub fn update(&self, rl: &mut RaylibHandle) {
		let mut stage = self.stage.write().unwrap();

		{
			let mut commands = self.commands.write().unwrap();
			let mut world = self.world.write().unwrap();
			eprintln!("update: {:?} {:?}", *stage, *world);

			for c in commands.drain(..) {
				match c {
					Command::Add(ids) => ids.iter().for_each(|x| {
						world.get_mut(x).and_then(|y| stage.insert(*x, NonNull::from(y)));
					}),
					Command::Remove(ids) => {
						ids.iter().for_each(|x| {
							stage.remove(x);
						});
					}
					Command::SwapFor(id, ids) => {
						stage.remove(&id);
						ids.iter().for_each(|x| {
							world
								.get_mut(x)
								.map(|y| {
									y.reset();
									y
								})
								.and_then(|y| stage.insert(*x, NonNull::from(y)));
						});
					}
				}
			}
		}

		for (id, comp) in stage.iter_mut() {
			{
				let mut current_id = self.current_id.write().unwrap();
				*current_id = *id;
			}
			let comp = unsafe { comp.as_mut() };
			comp.update(rl);
		}
	}

	pub fn draw(&self, d: &mut RaylibDrawHandle) {
		let mut stage = self.stage.write().unwrap();
		eprintln!("draw: {:?}", *stage);
		for (_, comp) in stage.iter_mut() {
			let comp = unsafe { comp.as_mut() };
			comp.draw(d);
		}
	}
}

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
	game
		.component(preset::intro_style("you awake in a strange world...").next("or at least that's how you feel..."))
		.component(preset::intro_style("or at least that's how you feel...").next("exit bed?"))
		.component(
			TwoChoice::new().content("exit bed?")
			.text_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 1.5))
			.alpha_tween(Tween::new(ease::cubic_in, 0.0, 1.0, 0.5))
			.next_one("you exited the bed, nothing of interest happened...")
			.next_two("you rolled over and went back to sleep, nothing of interest happened...")
		)
		.component(preset::intro_style("you exited the bed, nothing of interest happened..."))
		.component(preset::intro_style("you rolled over and went back to sleep, nothing of interest happened..."))
	;

	while !rl.window_should_close() {
		add_tick();

		let game = GAME.read().unwrap();
		game.update(&mut rl);

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		game.draw(&mut d);
	}
}

