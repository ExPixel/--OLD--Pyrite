use super::Gba;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs::create_dir_all;
use bincode::rustc_serialize::{encode, decode_from};
use bincode::SizeLimit;
use super::core::memory::MEM_BIOS;

/// Header for pyrite. It just says pyrite96
const PYRITE_HEADER: [u8; 8] = [0x70, 0x79, 0x72, 0x69, 0x74, 0x65, 0x39, 0x36];

/// The version of the current format.
const VERSION: u8 = 0;

pub trait BinarySerialization {
	fn serialize(&self, writer: &mut Write);
	fn deserialize<R: Read>(&mut self, reader: &mut R) -> Result<&'static str, &'static str>;

	// #TODO remove this temporary code.
	fn save_to_file<'a>(&self, file_path: &'a str) -> Result<&'static str, String>;
	fn load_from_file<'a>(&mut self, file_path: &'a str) -> Result<&'static str, String>;
}

impl BinarySerialization for Gba {
	// #TODO remove this temporary code.
	fn save_to_file<'a>(&self, file_path: &'a str) -> Result<&'static str, String> {
		let _save_path = Path::new(file_path);
		let _save_path_parent = _save_path.parent().expect("Getting save file path parent directory");
		create_dir_all(_save_path_parent).expect("Creating save file path parent directories");

		let mut f = match File::create(file_path) {
			Ok(file) => file,
			Err(_) => return Err(format!("Failed to open {}", file_path))
		};
		self.serialize(&mut f);
		return Ok("ok")
	}

	// #TODO remove this temporary code.
	fn load_from_file<'a>(&mut self, file_path: &'a str) -> Result<&'static str, String> {
		let mut f = match File::open(file_path) {
			Ok(file) => file,
			Err(_) => return Err(format!("Failed to open {}", file_path))
		};
		self.deserialize(&mut f).unwrap();
		return Ok("ok")
	}

	fn serialize(&self, w: &mut Write) {
		w.write(&PYRITE_HEADER).expect("Write pyrite header.");

		let data = [
			VERSION,
			if self.cpu.branched {1} else {0},
			self.cpu.prefetch_wait
		];
		w.write(&data).expect("Write version & CPU data.");

		let register_data: Vec<u8> = encode(&self.cpu.registers, SizeLimit::Infinite).expect("Encode CPU registers.");
		w.write(&register_data).expect("Write CPU registers.");

		let clock_data: Vec<u8> = encode(&self.cpu.clock, SizeLimit::Infinite).expect("Encode CPU clock.");
		w.write(&clock_data).expect("Write CPU clock data.");

		let joypad_data: Vec<u8> = encode(&self.joypad, SizeLimit::Infinite).expect("Encode Joypad");
		w.write(&joypad_data).expect("Write Joypad data.");

		let internal_regs: Vec<u8> = encode(&self.cpu.memory.internal_regs, SizeLimit::Infinite).expect("Encode internal registers.");
		w.write(&internal_regs).expect("Write internal registers.");

		let ram = &self.cpu.memory.internal_data[MEM_BIOS.size..];
		w.write(&ram).expect("Write RAM.");
	}

	fn deserialize<R: Read>(&mut self, r: &mut R) -> Result<&'static str, &'static str> {
		let mut buffer = [0u8; 16];

		if r.read_exact(&mut buffer[0..8]).is_ok() {
			if &buffer[0..8] != &PYRITE_HEADER {
				return Err("Header does not match Pyrite header.");
			}
		} else {
			return Err("Failed to read header.");
		}

		if r.read_exact(&mut buffer[0..3]).is_ok() {
			let version = buffer[0];
			if version != VERSION {
				return Err("Version did not match Pyrite's current serialization version.");
			}
			let branched = buffer[1];
			let prefetch_wait = buffer[2];
			self.cpu.branched = branched != 0;
			self.cpu.prefetch_wait = prefetch_wait;
		} else {
			return Err("Failed to read CPU and version information.");
		}

		if let Ok(cpu_registers) = decode_from(r, SizeLimit::Infinite) {
			self.cpu.registers = cpu_registers;
		} else {
			return Err("Failed to decode the CPU registers.");
		}

		if let Ok(cpu_clock) = decode_from(r, SizeLimit::Infinite) {
			self.cpu.clock = cpu_clock;
		} else {
			return Err("Failed to decode the CPU clock.");
		}

		if let Ok(joypad) = decode_from(r, SizeLimit::Infinite) {
			self.joypad = joypad;
		} else {
			return Err("Failed to decode the Joypad data.");
		}

		if let Ok(internal_regs) = decode_from(r, SizeLimit::Infinite) {
			self.cpu.memory.internal_regs = internal_regs;
		} else {
			return Err("Failed to decode the internal registers.");
		}

		{
			let ram = &mut self.cpu.memory.internal_data[MEM_BIOS.size..];
			if r.read_exact(ram).is_err() {
				return Err("Failed to read RAM");
			}
		}

		return Ok("OK")
	}
}