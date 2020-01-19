use std::collections::HashMap;
use ::util::sync_unsafe_cell::SyncUnsafeCell;

static mut CURRENT_PROFILER: bool = false;
lazy_static! {
	pub static ref PROFILER0: SyncUnsafeCell<Profiler<'static>> = SyncUnsafeCell::new(Profiler::new());
	pub static ref PROFILER1: SyncUnsafeCell<Profiler<'static>> = SyncUnsafeCell::new(Profiler::new());
}

pub unsafe fn swap_profilers() {
	CURRENT_PROFILER = !CURRENT_PROFILER;
}

pub unsafe fn get_write_profiler() -> &'static mut Profiler<'static> {
	if CURRENT_PROFILER {
		::debug::profiler::PROFILER1.get_mut_fast()
	} else {
		::debug::profiler::PROFILER0.get_mut_fast()
	}
}

pub unsafe fn get_read_profiler() -> &'static mut Profiler<'static> {
	if CURRENT_PROFILER {
		::debug::profiler::PROFILER0.get_mut_fast()
	} else {
		::debug::profiler::PROFILER1.get_mut_fast()
	}
}

pub fn print_depth(d: usize) {
	for _ in 0..d { print!("\t"); }
}

pub fn display_node<'a>(profiler: &Profiler<'a>, index: usize, depth: usize) {
	print_depth(depth);
	println!("- {}", profiler.get(index).name);
}

pub fn walk_profiler_nonrecursive<'a>(profiler: &Profiler<'a>, index: usize) {
	let mut current = index;
	let mut depth = 0;
	'walk_loop: loop {
		display_node(profiler, current, depth);
		if let Some(index) = profiler.get(current).children {
			current = index;
			depth += 1;
			continue 'walk_loop;
		} else if let Some(index) = profiler.get(current).next {
			current = index;
			continue 'walk_loop;
		} else {
			let mut parent = profiler.get(current).parent;
			while let Some(indexa) = parent {
				depth -= 1;
				if let Some(index) = profiler.get(indexa).next {
					current = index;
					continue 'walk_loop;
				} else {
					parent = profiler.get(indexa).parent;
				}
			}
		}
		break;
	}
}

pub fn walk_profiler_recursive<'a>(profiler: &Profiler<'a>, index: usize, depth: usize) {
	display_node(profiler, index, depth);
	let mut _next = profiler.get(index).next;

	if let Some(_child_index) = profiler.get(index).children {
		walk_profiler_recursive(profiler, _child_index, depth + 1);
	}

	while let Some(_next_index) = _next {
		display_node(profiler, _next_index, depth);
		if let Some(_child_index) = profiler.get(_next_index).children {
			walk_profiler_recursive(profiler, _child_index, depth + 1);
		}
		_next = profiler.get(_next_index).next;
	}
}


#[inline]
fn profiler_time() -> u64 { // #TODO at some point I might want to use rdtsc
	use time;
	time::precise_time_ns()
}

pub struct Profiler<'a> {
	pub nodes: Vec<ProfilerNode<'a>>,
	last_begin_node: Option<usize>,
	last_end_node: Option<usize>,
	pub start: Option<u64>,
	pub end: Option<u64>,
	pub locked: bool,
	mapped_values: HashMap<&'a str, usize>,
}

#[derive(Debug)]
pub struct ProfilerNode<'a> {
	pub name: &'a str,
	pub start: u64,
	pub end: Option<u64>,
	pub last_timed: u64,
	pub parent: Option<usize>,
	pub children: Option<usize>,
	pub next: Option<usize>,
}

impl<'a> ProfilerNode<'a> {
	pub fn elapsed_time(&self) -> u64 {
		let _start = self.start;
		return self.end.unwrap_or(_start) - _start;
	}
}

impl<'a> Profiler<'a> {
	pub fn new() -> Profiler<'a> {
		Profiler {
			nodes: Vec::new(),
			mapped_values: HashMap::new(),
			last_begin_node: None,
			last_end_node: None,
			start: None,
			end: None,
			locked: false,
		}
	}

	pub fn clear(&mut self) {
		if self.locked { return; }
		self.nodes.clear();
		self.mapped_values.clear();
		self.last_begin_node = None;
		self.last_end_node = None;
		self.start = None;
		self.end = None;
	}

	pub fn create_mapped_id(&mut self, id: &'a str, name: &'a str) {
		if self.locked { return; }
		let next_index = self.nodes.len();
		self.begin(name);
		self.end();
		self.mapped_values.insert(id, next_index);
		let mut node = self.get_mut(next_index);
		let _start = node.start;
		node.last_timed = _start;
		node.end = Some(_start);
	}

	fn get_mapped(&mut self, id: &'a str) -> &mut ProfilerNode<'a> {
		let idx = {
			let _idx = self.mapped_values.get(id).expect("No profiler node with the given ID.");
			(*_idx).clone()
		};
		return self.get_mut(idx);
	}

	pub fn begin_mapped(&mut self, id: &'a str) {
		if self.locked { return; }
		let node = self.get_mapped(id);
		node.last_timed = profiler_time();
	}

	pub fn end_mapped(&mut self, id: &'a str) {
		if self.locked { return; }
		let node = self.get_mapped(id);
		let delta = profiler_time() - node.last_timed;
		let _start = node.start;
		let _end = node.end.unwrap_or(_start);
		node.end = Some(_end + delta);
	}

	pub fn get(&self, index: usize) -> &ProfilerNode<'a> {
		self.nodes.get(index).expect("Profiler node with index does not exist at given index.")
	}

	pub fn get_mut(&mut self, index: usize) -> &mut ProfilerNode<'a> {
		self.nodes.get_mut(index).expect("Profiler node with index does not exist at given index. (mut)")
	}

	pub fn begin(&mut self, name: &'a str) {
		if self.locked { return; }
		let current_index = self.nodes.len();

		let _parent = self.last_begin_node.take();
		if let Some(began_idx) = _parent {
			let node = self.get_mut(began_idx);
			if node.children.is_none() {
				node.children = Some(current_index);
			}
		}

		if let Some(ended_idx) = self.last_end_node.take() {
			let node = self.get_mut(ended_idx);
			node.next = Some(current_index);
		}

		let node = Self::create_node(name, _parent);
		if self.start.is_none() {
			self.start = Some(node.start);
		}
		self.nodes.push(node);
		self.last_begin_node = Some(current_index);
	}

	pub fn end(&mut self) {
		if self.locked { return; }
		if let Some(began_idx) = self.last_begin_node.take() {
			let _time = profiler_time();
			self.get_mut(began_idx).end = Some(_time);
			self.last_end_node = Some(began_idx);
			self.last_begin_node = self.get(began_idx).parent;
			self.end = Some(_time);
		}
	}

	pub fn elapsed_time(&self) -> u64 {
		let _start = self.start.unwrap_or(0);
		return self.end.unwrap_or(_start) - _start;
	}

	pub fn create_node(name: &'a str, parent: Option<usize>) -> ProfilerNode<'a> {
		ProfilerNode {
			name: name,
			last_timed: 0,
			start: profiler_time(),
			end: None,
			parent: parent,
			children: None,
			next: None,
		}
	}
}