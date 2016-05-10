use super::super::gba::Gba;
use super::super::gba::serialization::*;
use super::super::gba::core::memory::*;
use super::bitdesc::*;
use std::default::Default;
use rustbox::{Color, RustBox};
use rustbox::Key;
use rustbox::Style;
use rustbox;
use std::ascii::AsciiExt;
use std::time::Duration;
use ::util::measure::*;

use glium::glutin;

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
	"help",
	"frame",
	"frame-s",
	"save-state",
	"load-state",
	"measure"
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
			match self.rustbox.peek_event(Duration::from_millis(30), false) {
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
			for event in self.gba.device.video.display.poll_events() {
				match event {
					glutin::Event::Closed => {
						self.running = false;
						self.gba.request_exit = true;
					},
					_ => {} // We throw away most of the glutin events.
				}
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

	pub fn parse_command(command_string: &String) -> Option<(String, Vec<String>)> {
		let trimmed = command_string.trim();
		if trimmed.len() == 0 { return None }
		let mut command_name: Option<String> = None;
		let mut command_args: Vec<String> = Vec::new();
		let mut parsed_command_name = false;
		let mut in_string = false;
		let mut escaped = false;
		let mut buffer = String::new();
		for c in trimmed.chars() {
			if c == '"' {
				if escaped {
					buffer.push(c);
				} else {
					if in_string {
						in_string = false;
						if parsed_command_name { command_args.push(buffer.clone()); }
						else { command_name = Some(buffer.clone()); parsed_command_name = true; }
						buffer.clear();
					} else {
						in_string = true;
					}
				}
			} else {
				if (c as u32) <= 32 {
					if !in_string {
						if parsed_command_name { command_args.push(buffer.clone()); }
						else { command_name = Some(buffer.clone()); parsed_command_name = true; }
						buffer.clear();
					} else {
						buffer.push(c);
					}
				} else if c == '\\' {
					if escaped { buffer.push(c) }
					else { escaped = true; continue; }
				} else {
					buffer.push(c);
				}
			}
			escaped = false;
		}

		if buffer.len() > 0 {
			if parsed_command_name { command_args.push(buffer.clone()); }
			else { command_name = Some(buffer.clone()); }
		}
		
		if let Some(cname) = command_name {
			return Some((cname, command_args))
		} else {
			return None
		}
	}

	pub fn process_command(&mut self) {
		let history = self.command_buffer.clone();
		self.push_to_history(history);

		self.command_history_index = -1;
		self.clear_row(2); // Clears the error line.

		let (command_name, arguments) = match Self::parse_command(&self.command_buffer) {
			Some((c, a)) => (c, a),
			None => {
				self.write_error_line("No command provided.");
				return
			}
		};

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
				self.cmd_frame(false, &arguments);
				// self.running = false;
				// self.gba.extras.request_debugger = true;
			},

			"frame-s" => { // This will run one frame of the GBA.
				self.cmd_frame(true, &arguments);
				// self.running = false;
				// self.gba.extras.request_debugger = true;
			},

			"save-state" => {
				if arguments.len() < 1 {
					self.too_few_args(1, &command_name, &arguments);
					return;
				}
				self.cmd_save_state(&arguments);
			},

			"load-state" => {
				if arguments.len() < 1 {
					self.too_few_args(1, &command_name, &arguments);
					return;
				}
				self.cmd_load_state(&arguments);
			},

			"measure" => {
				self.cmd_measure(&arguments);
			},

			// #TODO remove this temporary command.
			"play-sound" => {
				self.gba.device.audio.channels.channel1.on = true;
				self.gba.device.audio.commit_channel1();
			},

			// #TODO remove this temporary command.
			"stop-sound" => {
				self.gba.device.audio.channels.channel1.on = false;
				self.gba.device.audio.commit_channel1();
			}

			_ => {
				self.write_error_line(&format!("Unrecognized command '{}'", command_name));
			}
		}
	}

	pub fn too_few_args(&self, arg_req: usize, cname: &String, cargs: &[String]) {
		self.write_error_line(&format!("Command {} requires {} arguments ({} provided)", cname, arg_req, cargs.len()));
	}

	pub fn cmd_measure(&mut self, args: &[String]) {
		if args.len() < 1 {
			self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Default, Color::Default,
				&format!("{} measures available.", AVAILABLE_MEASURES));
			return;
		}

		let measure_index = match auto_radix_parse_u32(&args[0]) {
			Some(mindex) => mindex,
			None => { self.write_error_line(&format!("{} is not a number.", args[0])); return }
		};

		let measure = get_measure_info(measure_index as usize);
		if measure.is_none() {
			self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Red, Color::Default,
				&format!("Measure #{} does not exist.", measure_index));
			return;
		}
		let measure = measure.unwrap();

		self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Default, Color::Default, "Measure Name:");
		self.rustbox.print(16, DSTART, rustbox::RB_NORMAL, Color::Yellow, Color::Default,
			&format!("{}", measure.name));

		self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Default, Color::Default, "Iterations:");
		self.rustbox.print(14, DSTART + 1, rustbox::RB_NORMAL, Color::Yellow, Color::Default,
			&format!("{}", measure.iterations));

		self.rustbox.print(1, DSTART + 2, rustbox::RB_BOLD, Color::Default, Color::Default, "Total Time:");
		self.rustbox.print(14, DSTART + 2, rustbox::RB_NORMAL, Color::Yellow, Color::Default,
			&format!("{}", measure.total_time));

		self.rustbox.print(1, DSTART + 3, rustbox::RB_BOLD, Color::Default, Color::Default, "Iteration Time(ns):");
		self.rustbox.print(22, DSTART + 3, rustbox::RB_NORMAL, Color::Yellow, Color::Default,
			&format!("{}", measure.iteration_time_ns()));

		self.rustbox.print(1, DSTART + 4, rustbox::RB_BOLD, Color::Default, Color::Default, "Iteration Time(ms):");
		self.rustbox.print(22, DSTART + 4, rustbox::RB_NORMAL, Color::Yellow, Color::Default,
			&format!("{}", measure.iteration_time_ms()));

		self.rustbox.print(1, DSTART + 5, rustbox::RB_BOLD, Color::Default, Color::Default, "Iteration Time( s):");
		self.rustbox.print(22, DSTART + 5, rustbox::RB_NORMAL, Color::Yellow, Color::Default,
			&format!("{}", measure.iteration_time_s()));
	}

	pub fn cmd_save_state(&mut self, args: &[String]) {
		let sfile = format!("data/sav/{}.psav", args[0]);
		self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Default, Color::Default, &format!("Saving state to {}...", sfile));
		if let Ok(_) = self.gba.save_to_file(&sfile) {
			self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Green, Color::Default, &format!("Successfully saved state to {}...", sfile));
		} else {
			self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Red, Color::Default, &format!("Failed to save state to {}...", sfile));
		}
	}

	pub fn cmd_load_state(&mut self, args: &[String]) {
		let sfile = format!("data/sav/{}.psav", args[0]);
		self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Default, Color::Default, &format!("Loading state from {}...", sfile));
		if let Ok(_) = self.gba.load_from_file(&sfile) {
			self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Green, Color::Default, &format!("Successfully loaded state from {}...", sfile));
		} else {
			self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Red, Color::Default, &format!("Failed to load state from {}...", sfile));
		}
	}

	pub fn cmd_frame(&mut self, silent: bool, args: &[String]) {
		let mut frames = 1;

		if args.len() > 0 {
			frames = match auto_radix_parse_u32(&args[0]) {
				Some(_frames) => _frames,
				None => { self.write_error_line(&format!("{} is not a number.", args[0])); return }
			};
		}

		self.rustbox.print(1, DSTART, rustbox::RB_BOLD, Color::Default, Color::Default, &format!("Running {} frames...", frames));
		self.rustbox.present();
		while frames > 0 {
			self.gba.tick(0);
			if self.gba.request_exit {
				self.running = false;
				self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Green, Color::Default, "Close requested while running frames.");
				self.rustbox.present();
				return
			} else if !silent {
				self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Yellow, Color::Default, &format!("{} frames remaining...", frames));
				self.rustbox.present();
			}
			frames -= 1;
		}
		self.rustbox.print(1, DSTART + 1, rustbox::RB_BOLD, Color::Green, Color::Default, "Finished running frames.");
		self.rustbox.present();
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

			"bg0hofs", BG0HOFS, RegBGHOFS
			"bg0vofs", BG0VOFS, RegBGVOFS
			"bg1hofs", BG1HOFS, RegBGHOFS
			"bg1vofs", BG1VOFS, RegBGVOFS
			"bg2hofs", BG2HOFS, RegBGHOFS
			"bg2vofs", BG2VOFS, RegBGVOFS
			"bg3hofs", BG3HOFS, RegBGHOFS
			"bg3vofs", BG3VOFS, RegBGVOFS

			"win0h", WIN0H, RegWinH
			"win1h", WIN1H, RegWinH
			"win0v", WIN0V, RegWinV
			"win1v", WIN1V, RegWinV

			"winin", WININ, RegWinIn
			"winout", WINOUT, RegWinOut
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

		self.rustbox.print(1, row, rustbox::RB_BOLD, Color::Default, Color::Default, "#value:");

		if desc.data_length() <= 8 {
			self.rustbox.print(10, row, rustbox::RB_BOLD, Color::Default, Color::Default, 
				&format!("0x{:02X}", desc.get_value()));
		} else if desc.data_length() <= 16 {
			self.rustbox.print(10, row, rustbox::RB_BOLD, Color::Default, Color::Default, 
				&format!("0x{:04X}", desc.get_value()));
		} else {
			self.rustbox.print(10, row, rustbox::RB_BOLD, Color::Default, Color::Default, 
				&format!("0x{:08X}", desc.get_value()));
		}

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