use ::debug::profiler::Profiler;
use rust_imgui as imgui;

pub struct ProfilerGUI {
	show_as_tree: bool,
}

impl ProfilerGUI {
	pub fn new() -> ProfilerGUI {
		ProfilerGUI {
			show_as_tree: true,
		}
	}

	pub fn render(&mut self) {
		if self.show_as_tree {
			self.render_tree();
		} else {
			imgui::text(imstr!("Unsupported Display Format"));
		}
	}

	pub fn render_tree_node<'a>(&self, profiler: &mut Profiler<'a>, current: usize) -> bool {
		let node = profiler.get(current);
		let tree_expand_ret;
		if node.children.is_some() {
			tree_expand_ret = imgui::tree_node_str(imstr!("profiler_node_{}", current), imstr!("{}", node.name));
		} else {
			tree_expand_ret = false;
			imgui::bullet_text(imstr!("{}", node.name));
		}
		imgui::next_column();
		let (elapsed, units) = time_reduce(node.elapsed_time());
		imgui::text(imstr!("{:.2}{}", elapsed, units));
		imgui::next_column();

		let max = if let Some(idx) = node.parent {
			profiler_r!().get(idx).elapsed_time()
		} else {
			profiler_r!().elapsed_time()
		};
		imgui::text(imstr!("{:.2}%", percentage!(max, node.elapsed_time())));

		if imgui::is_item_hovered() {
			let frame_max = profiler_r!().elapsed_time();
			imgui::begin_tooltip();
			imgui::text(imstr!("{:.2}% of frame", percentage!(frame_max, node.elapsed_time())));
			imgui::end_tooltip();
		}

		imgui::next_column();
		return tree_expand_ret;
	}

	pub fn render_tree(&self) {
		let profiler = profiler_r!();
		let mut current = 0;
		// let mut depth = 0;

		imgui::columns_noid(3, true);
		imgui::text(imstr!("Operation"));
		imgui::next_column();
		imgui::text(imstr!("Time"));
		imgui::next_column();
		imgui::text(imstr!("Percentage"));
		imgui::next_column();

		'walk_loop: loop {
			let show_children = self.render_tree_node(profiler, current);

			if show_children {
				if let Some(index) = profiler.get(current).children {
					current = index;
					// depth += 1;
					continue 'walk_loop;
				}
			}

			if let Some(index) = profiler.get(current).next {
				current = index;
				continue 'walk_loop;
			}

			let mut parent = profiler.get(current).parent;
			while let Some(indexa) = parent {
				// depth -= 1;
				imgui::tree_pop();
				if let Some(index) = profiler.get(indexa).next {
					current = index;
					continue 'walk_loop;
				} else {
					parent = profiler.get(indexa).parent;
				}
			}

			break;
		}
		imgui::columns_noid(1, false);
	}
}


pub fn time_reduce(ns: u64) -> (f64, &'static str) {
	if ns > 1000000000 {
		return (ns as f64 / 1000000000.0, "s");
	} else if ns > 1000000 {
		return (ns as f64 / 1000000.0, "ms");
	} else {
		return (ns as f64, "ns");
	}
}