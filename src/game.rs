use std::marker::PhantomPinned;
use std::ptr::NonNull;
use std::sync::RwLock;

use fnv::FnvHashMap as HashMap;
use crate::component::Component;

use raylib::prelude::*;

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
	pub fn new() -> Self {
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

		world.iter().find(|(_, v)| v.search(namespace, contents)).and_then(|x| Some(*x.0))
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
