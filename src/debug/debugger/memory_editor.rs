use rust_imgui as imgui;
use rust_imgui::ImVec4;
use rust_imgui::ImguiTextInputBuffer;
use glutin::VirtualKeyCode;
use ::gba::core::memory::*;
use ::gba::device::imgui_support::is_key_pressed;

const MEMORY_ADDR_COL: ImVec4 = ImVec4 {x: 0.914, y: 0.118, z: 0.388, w: 1.000}; // E91E63
const MEMORY_BYTE_COL: ImVec4 = ImVec4 {x: 1.000, y: 1.000, z: 1.000, w: 1.000}; // FFFFFF
const MEMORY_BYTE_SELECTED_COL: ImVec4 = ImVec4 {x: 1.000, y: 0.702, z: 0.000, w: 1.000}; // FFB300
const MEMORY_TEXT_COL: ImVec4 = ImVec4 {x: 0.012, y: 0.663, z: 0.957, w: 1.000}; // 03A9F4
const MEMORY_EDITING_COLUMNS: u32 = 16;
const MEMORY_EDITING_ROWS: u32 = 16;

pub struct MemoryEditor {
	display_addr_start: u32,
	cursor_addr: u32,
	addr_input_buffer: ImguiTextInputBuffer,
}

impl MemoryEditor {
	pub fn new() -> MemoryEditor {
		MemoryEditor {
			display_addr_start: 0,
			cursor_addr: 0,
			addr_input_buffer: ImguiTextInputBuffer::new(16),
		}
	}

	fn move_to(&mut self, addr: u32) {
		self.cursor_addr = addr;
		self.display_addr_start = addr;
	}

	pub fn render(&mut self, memory: &GbaMemory) {
		imgui::text_disabled(imstr!("[{:08X}]", self.cursor_addr)); // So for one frame you get the wrong cursor address :\
		let input_in_use = imgui::input_text(imstr!(""), self.addr_input_buffer.as_slice(), 
			imgui::ImGuiInputTextFlags_EnterReturnsTrue, None, None);

		if imgui::is_window_focused() {
			imgui::capture_keyboard_from_app(true);
			if !input_in_use {
				if is_key_pressed(VirtualKeyCode::Down, true) { self.cursor_addr += 16; }
				if is_key_pressed(VirtualKeyCode::Up, true) { self.cursor_addr -= 16; }
				if is_key_pressed(VirtualKeyCode::Right, true) { self.cursor_addr += 1; }
				if is_key_pressed(VirtualKeyCode::Left, true) { self.cursor_addr -= 1; }
			} else if is_key_pressed(VirtualKeyCode::Return, false) && input_in_use {
				if let Some(addr) = auto_radix_parse_u32(self.addr_input_buffer.as_str()) {
					self.move_to(addr)
				}
				self.addr_input_buffer.clear();
			}
		}

		imgui::same_line();
		if imgui::button_def(imstr!("Go To...")) {
			imgui::open_popup(imstr!("goto_memory_area"));
		}
		if imgui::begin_popup(imstr!("goto_memory_area")) {
			if imgui::menu_item(imstr!("00000000 - BIOS (1K)")) { self.move_to(0x00000000); }
			if imgui::menu_item(imstr!("02000000 - On-Board RAM (256K)")) { self.move_to(0x02000000); }
			if imgui::menu_item(imstr!("03000000 - On-Chip RAM (32K)")) { self.move_to(0x03000000); }
			if imgui::menu_item(imstr!("04000000 - IO registers")) { self.move_to(0x04000000); }
			if imgui::menu_item(imstr!("05000000 - Palette (1K)")) { self.move_to(0x05000000); }
			if imgui::menu_item(imstr!("06000000 - VRAM (96K)")) { self.move_to(0x06000000); }
			if imgui::menu_item(imstr!("07000000 - OAM (1K)")) { self.move_to(0x07000000); }
			if imgui::menu_item(imstr!("08000000 - ROM (32M)")) { self.move_to(0x08000000); }
			if imgui::menu_item(imstr!("0a000000 - ROM 1")) { self.move_to(0x0a000000); }
			if imgui::menu_item(imstr!("0c000000 - ROM 2")) { self.move_to(0x0c000000); }
			if imgui::menu_item(imstr!("0e000000 - SRAM (64K)")) { self.move_to(0x0e000000); }
			imgui::end_popup();
		}

		let mut display_addr_end = self.display_addr_start + (MEMORY_EDITING_ROWS * 16);
		if (self.cursor_addr & !0xf) < self.display_addr_start {
			self.display_addr_start = self.cursor_addr & !0xf;
		} else if (self.cursor_addr & !0xf) >= display_addr_end {
			display_addr_end = (self.cursor_addr & !0xf) + 16;
			self.display_addr_start = display_addr_end - (MEMORY_EDITING_ROWS * 16);
		}

		imgui::push_style_var_vec(imgui::ImGuiStyleVar::ItemSpacing, imgui::vec2(0.0, 0.0));

		let mut addr = self.display_addr_start;
		for _ in 0..MEMORY_EDITING_ROWS {
			self.render_line(memory, addr);
			addr += MEMORY_EDITING_COLUMNS;
		}

		imgui::pop_style_var(1);
	}

	pub fn render_line(&self, memory: &GbaMemory, start_addr: u32) {
		imgui::push_style_color(imgui::ImGuiCol::Text, MEMORY_ADDR_COL);
		imgui::text_unformatted(imstr!("{:08X}  ", start_addr));
		imgui::pop_style_color(1);

		imgui::push_style_color(imgui::ImGuiCol::Text, MEMORY_BYTE_COL);
		for addr in start_addr..(start_addr + MEMORY_EDITING_COLUMNS) {
			imgui::same_line();
			let _byte = memory.read8(addr);

			if addr == self.cursor_addr {
				imgui::push_style_color(imgui::ImGuiCol::Text, MEMORY_BYTE_SELECTED_COL);
				imgui::text_unformatted(imstr!("{:02X} ", _byte));
				imgui::pop_style_color(1);
				let dlist = imgui::get_window_draw_list().expect("Failed to get window draw list.");
				let mut _min = imgui::vec2(0.0, 0.0);
				let mut _max = imgui::vec2(0.0, 0.0);
				imgui::get_item_rect_min(&mut _min);
				imgui::get_item_rect_max(&mut _max);
				dlist.add_rect_filled_simple(_min, _max, 0x22FFB300);
				dlist.add_rect_simple(_min, _max, 0xFFFFB300);
			} else {
				imgui::text_unformatted(imstr!("{:02X} ", _byte));
			}
		}
		imgui::pop_style_color(1);

		imgui::same_line();
		imgui::text_unformatted(imstr!("  "));

		imgui::push_style_color(imgui::ImGuiCol::Text, MEMORY_TEXT_COL);
		for addr in start_addr..(start_addr + MEMORY_EDITING_COLUMNS) {
			imgui::same_line();
			let _byte = memory.read8(addr);
			if _byte < 32 || _byte > 126 {
				imgui::text_unformatted(imstr!("."));
			} else {
				imgui::text_unformatted(imstr!("{}", _byte as char));
			}
		}
		imgui::pop_style_color(1);
	}
}

pub fn auto_radix_parse_u32<'a>(s: &'a str) -> Option<u32> {
	let trimmed = s.trim();
	if trimmed.starts_with("0x") {
		if let Ok(ret) = u32::from_str_radix(&trimmed[2..], 16) {
			return Some(ret)
		}
	} else {
		if let Ok(ret) = u32::from_str_radix(trimmed, 10) {
			return Some(ret)
		}
	}
	return None
}