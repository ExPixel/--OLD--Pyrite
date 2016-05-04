use super::super::gba::Gba;
use super::super::gba::core::memory::*;
use super::bitdesc::*;
use std::default::Default;
use rustbox::{Color, RustBox};
use rustbox::Key;
use rustbox::Style;
use rustbox;
use std::ascii::AsciiExt;

const HEADER_LINE: usize = 0;
const COMMAND_LINE: usize = 1;
const ERROR_LINE: usize = 2;
const DSTART: usize = 3; // display start

const MAX_HISTORY_SIZE: usize = 32;

const IMPLEMENTED_COMMANDS: &'static [&'static str] = &[
	"exit",
	"kill-emulator",
	"print-memory",
	"ioreg",
	"help"
];

pub struct GbaDebugger<'a> {
	gba: &'a mut Gba,
	rustbox: RustBox,
	running: bool,
	command_buffer: String,

	command_history_index: isize,
	command_history: Vec<String>
}

impl<'a> GbaDebugger<'a> {
	pub fn new<'b>(gba: &'b mut Gba) -> GbaDebugger<'b> {
		let rustbox = match RustBox::init(Default::default()) {
			Ok(v) => v,
			Err(e) => panic!("Failed to open rustbox {}", e)
		};

		GbaDebugger {
			gba: gba,
			rustbox: rustbox,
			running: true,
			command_buffer: String::new(),
			command_history_index: -1,
			command_history: Vec::new()
		}
	}

	pub fn start(&mut self) {
		self.reset_screen();

		while self.running {
			self.rustbox.present();
			match self.rustbox.poll_event(false) {
				Ok(rustbox::Event::KeyEvent(key)) => {
					match key {
						Key::Char(c) => { self.char_typed(c) },
						Key::Backspace => { self.backspace() },
						Key::Enter => { self.execute_command() },
						Key::Tab => { self.complete_command() },
						Key::Up => { self.command_history_move_up() },
						Key::Down => { self.command_history_move_down() },
						_ => {}
					}
				},
				Err(e) => panic!("Failed to read rustbox event: {}", e),
				_ => {}
			}
		}
	}

	pub fn reset_screen(&mut self) {
		self.rustbox.clear();
		self.rustbox.print(1, HEADER_LINE, rustbox::RB_BOLD, Color::White, Color::Black, "--- Pyrite Debugger ---");
		self.display_command();
	}

	pub fn clear_display_area(&self) {
		if self.rustbox.height() > DSTART {
			for y in 3..self.rustbox.height() {
				self.clear_row(y);
			}
		}
	}

	pub fn process_command(&mut self) {
		let history = self.command_buffer.clone();
		self.push_to_history(history);

		self.command_history_index = -1;
		self.clear_row(2); // Clears the error line.

		let command_name;
		let mut arguments: Vec<String> = Vec::new();

		{
			let mut split = self.command_buffer.split_whitespace();
			if let Some(name) = split.next() {
				command_name = String::from(name);
			} else {
				self.write_error_line("No command provided.");
				return
			}

			while let Some(arg) = split.next() {
				arguments.push(String::from(arg));
			}
		}

		self.clear_display_area();
		match command_name.to_ascii_lowercase().as_ref() {
			"exit" => { self.running = false },

			"kill-emulator" => {
				self.running = false;
				self.gba.request_exit = true;
			},

			"print-memory" => {
				if arguments.len() < 1 {
					self.too_few_args(1, &command_name, &arguments);
					return;
				}
				self.cmd_print_memory_table(&arguments);
			},

			"help" => {
				self.print_help();
			},

			"ioreg" => {
				if arguments.len() < 1 {
					self.too_few_args(1, &command_name, &arguments);
					return;
				}
				self.cmd_ioreg(&arguments);
			},

			"frame" => { // This will run one frame of the GBA.
				self.running = false;
				self.gba.extras.request_debugger = true;
			},

			_ => {
				self.write_error_line(&format!("Unrecognized command '{}'", command_name));
			}
		}
	}

	pub fn too_few_args(&self, arg_req: usize, cname: &String, cargs: &[String]) {
		self.write_error_line(&format!("Command {} requires {} arguments ({} provided)", cname, arg_req, cargs.len()));
	}

	pub fn cmd_ioreg(&self, args: &[String]) {
		macro_rules! generate_reg_descriptions {
			($($cmd_arg: expr, $reg_name: ident, $reg_desc_type: ident)+) => (
				match args[0].to_ascii_lowercase().as_ref() {
					// "dispcnt" => ("DISPCNT", RegDispcntDesc::from(self.gba.cpu.memory.get_reg(ioreg::DISPCNT) as u32)),
					$(
						$cmd_arg => (stringify!($reg_name), Box::new($reg_desc_type::from(self.gba.cpu.memory.get_reg(ioreg::$reg_name) as u32))),
					)+
					_ => {
						self.write_error_line(&format!("Unsupported IO register `{}`.", args[0]));
						return;
					}
				};
			)
		}

		let data: (&str, Box<BitDescriptor>) = generate_reg_descriptions!(
			"dispcnt", DISPCNT, RegDispcntDesc
			"dispstat", DISPSTAT, RegDispStat
			"bg0cnt", BG0CNT, RegBGCnt
			"bg1cnt", BG1CNT, RegBGCnt
			"bg2cnt", BG2CNT, RegBGCnt
			"bg3cnt", BG3CNT, RegBGCnt
		);

		self.print_bitdesc_data(data.0, data.1);
	}

	pub fn cmd_print_memory_table(&self, args: &[String]) {
		let start = match auto_radix_parse_u32(&args[0]) {
			Some(_start) => _start & !(0xF),
			None => { self.write_error_line(&format!("{} is not a number.", args[0])); return }
		};


		let end = if args.len() > 1 {
			match auto_radix_parse_u32(&args[1]) {
				Some(_end) => _end,
				None => { self.write_error_line(&format!("{} is not a number.", args[1])); return }
			}
		} else {
			start + 2048
		};

		self.print_memory_table(start, end);
	}

	pub fn print_memory_table(&self, start: u32, end: u32) {
		if self.rustbox.width() < 56 { // for the address and 16 bytes
			self.write_error_line("Requires at least 56 columns to run!");
			return;
		}

		let mut address = start;
		let mut row = DSTART;
		let _height = self.rustbox.height();
		let _width = self.rustbox.width();
		let mut column;

		while address <= end && row < _height {
			self.rustbox.print(1, row, rustbox::RB_BOLD, Color::Default, Color::Default, &format!("{:08X}", address));
			column = 9;
			while column < (_width - 3) && column < 56 {
				let byte = self.gba.cpu.memory.read8(address);
				self.rustbox.print(column, row, rustbox::RB_NORMAL, Color::Default, Color::Default,
					&format!(" {:02X}", byte));
				column += 3;
				address += 1;
			}
			row += 1;
		}
	}

	pub fn print_bitdesc_data<'l>(&self, desc_name: &'l str, desc: Box<BitDescriptor>) {
		let mut row = DSTART;

		self.rustbox.print(1, row, rustbox::RB_BOLD, Color::Default, Color::Default, "REGISTER:");
		self.rustbox.print(12, row, rustbox::RB_BOLD, Color::Yellow, Color::Default, desc_name);
		row += 1;

		for i in 0..desc.property_count() {
			let property_name_len = desc.get_property_name(i).len();
			let property_value = desc.get_property_value(i).to_string();
			self.rustbox.print(1, row, rustbox::RB_BOLD, Color::Default, Color::Default, desc.get_property_name(i));
			self.rustbox.print(property_name_len + 1, row, rustbox::RB_BOLD, Color::Default, Color::Default, ": ");
			self.rustbox.print(property_name_len + 3, row, rustbox::RB_BOLD, Color::Yellow, Color::Default, &property_value);
			row += 1;
		}
	}

	pub fn print_help(&self) {
		self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Default, Color::Default, "Available Commands:");
		let mut row = DSTART + 1;
		let mut column = 1;
		let mut max_column_width = 0;
		for i in 0..IMPLEMENTED_COMMANDS.len() {
			let cmd = IMPLEMENTED_COMMANDS[i];
			self.rustbox.print(column, row, rustbox::RB_BOLD, Color::Green, Color::Default, cmd);

			if cmd.len() > max_column_width {
				max_column_width = cmd.len();
			}

			row += 1;

			if row > self.rustbox.height() {
				row = DSTART + 1;
				column += max_column_width + 1;
				max_column_width = 0;
			}
		}
	}

	pub fn write_error_line<'l>(&self, err: &'l str) {
		self.clear_row(2);
		self.rustbox.print(1, ERROR_LINE, rustbox::RB_BOLD, Color::Red, Color::Default, err);
	}

	pub fn execute_command(&mut self) {
		self.process_command();
		self.command_buffer.clear();
		self.display_command();
	}

	fn clear_row(&self, y: usize) {
		// let sty = rustbox::RB_NORMAL;
		let fg = Style::from_color(Color::Default);
		let bg = Style::from_color(Color::Default);
		let w = self.rustbox.width();
		for x in 0..w {
			// change_cell(&self, x: usize, y: usize, ch: u32, fg: u16, bg: u16)
			unsafe {
				self.rustbox.change_cell(x, y, ' ' as u32, fg.bits(), bg.bits());
			}
		}
	}

	pub fn complete_command(&mut self) {
		let mut set_command = false;
		let mut create_space = false;

		for i in 0..IMPLEMENTED_COMMANDS.len() {
			let test = IMPLEMENTED_COMMANDS[i];
			if test.starts_with(&self.command_buffer.to_ascii_lowercase()) {
				if set_command {
					create_space = false;
					break
				} else {
					self.command_buffer = String::from(test);
					set_command = true;
					create_space = true;
				}
			}
		}

		if create_space {
			self.command_buffer.push(' ');
		}

		if set_command {
			self.display_command();
		}
	}

	pub fn backspace(&mut self) {
		let clen = self.command_buffer.len();
		if clen > 0 {
			let nlen = self.command_buffer.len() - 1;
			self.command_buffer.truncate(nlen);
			self.display_command();
		}
	}

	pub fn char_typed(&mut self, c: char) {
		self.command_buffer.push(c);
		self.command_history_index = -1;
		self.display_command();
	}

	pub fn display_command(&mut self) {
		self.clear_row(1);
		self.rustbox.print(1, COMMAND_LINE, rustbox::RB_BOLD, Color::Yellow, Color::Default, "> ");
		self.rustbox.print(3, COMMAND_LINE, rustbox::RB_NORMAL, Color::White, Color::Default, &self.command_buffer);
		let clen = self.command_buffer.len();
		self.rustbox.print(3 + clen, COMMAND_LINE, rustbox::RB_BOLD, Color::Yellow, Color::Default, "_");
	}

	pub fn command_history_move_up(&mut self) {
		if self.command_history.len() < 1 { return }

		if self.command_history_index < 0 {
			self.command_history_index = (self.command_history.len() - 1) as isize;
		} else if self.command_history_index == 0 {
			return
		} else {
			self.command_history_index -= 1;
		}

		self.command_buffer = self.command_history[self.command_history_index as usize].clone();
		self.display_command();
	}

	pub fn command_history_move_down(&mut self) {
		if self.command_history_index < 0 { return }
		if self.command_history.len() < 1 { return }
		if self.command_history_index == (self.command_history.len() - 1) as isize {
			self.command_history_index = -1;
			self.command_buffer.clear();
		} else {
			self.command_history_index += 1;
			self.command_buffer = self.command_history[self.command_history_index as usize].clone();
		}
		self.display_command();
	}

	pub fn push_to_history(&mut self, command: String) {
		if self.command_history.len() > 0 {
			let _end = self.command_history.len() - 1;
			if self.command_history[_end].eq_ignore_ascii_case(&command) {
				return;
			}
		}

		self.command_history.push(command);
		if self.command_history.len() > MAX_HISTORY_SIZE {
			self.command_history.remove(0);
		}
	}
}

pub fn auto_radix_parse_u32<'a>(s: &'a str) -> Option<u32> {
	if s.starts_with("0x") {
		if let Ok(ret) = u32::from_str_radix(&s[2..], 16) {
			return Some(ret)
		}
	} else {
		if let Ok(ret) = u32::from_str_radix(s, 10) {
			return Some(ret)
		}
	}
	return None
}