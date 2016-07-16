use rust_imgui as imgui;
use rust_imgui::ImVec4;
use rust_imgui::imstr::ImStr;
use std::collections::VecDeque;

pub struct ConsoleLine {
	color: ImVec4,
	text: String,
}

pub struct ImGuiConsole {
	max_entries: usize,
	scroll_to_bottom: bool,
	entries: VecDeque<ConsoleLine>,
	entry_count: usize,
}

impl ImGuiConsole {
	pub fn new(max_entries: usize, scroll_to_bottom: bool) -> ImGuiConsole {
		ImGuiConsole {
			max_entries: max_entries,
			scroll_to_bottom: scroll_to_bottom,
			entries: VecDeque::new(),
			entry_count: 0,
		}
	}

	pub fn render(&self) {
		let mut entry_num = self.entry_count - self.entries.len() + 1;
		for idx in 0..self.entries.len() {
			imgui::text_unformatted(imstr!("{}.", entry_num));
			imgui::same_line();
			let entry = &self.entries[idx];
			imgui::push_style_color(imgui::ImGuiCol::Text, entry.color);
			imgui::text_unformatted(ImStr::from_bytes_unchecked(entry.text.as_bytes()));
			imgui::pop_style_color(1);
			entry_num += 1;
		}

		if self.scroll_to_bottom {
			imgui::set_scroll_here(0.5);
		}
	}

	pub fn log(&mut self, color: ImVec4, mut text: String) {
		text.push('\0');
		let cline = ConsoleLine {
			color: color,
			text: text
		};
		self.entries.push_back(cline);
		if self.entries.len() > self.max_entries {
			self.entries.pop_front().unwrap();
		}
		self.entry_count += 1;
	}
}