#[macro_use] pub mod util;
pub mod gba;
pub mod debug;

extern crate time;
#[macro_use] extern crate glium;
extern crate rustc_serialize;
extern crate bincode;

use std::io::prelude::*;
use std::env;
use std::fs::File;

use gba::core::memory::GbaMemory;
use gba::Gba;

const BIOS_PATH: &'static str = "roms/legal/gba.bin";

macro_rules! println_err {
	($($arg:tt)*) => (
		match writeln!(&mut ::std::io::stderr(), $($arg)*) {
			Ok(_) => {},
			Err(error) => panic!("Failed to write to stderr: {}", error)
		}
	)
}

pub fn load_bios(memory: &mut GbaMemory) {
	let mut f = match File::open(BIOS_PATH) {
		Ok(file) => file,
		Err(error) => panic!("Error while opening file '{}': {}", BIOS_PATH, error)
	};

	let mut bios_buffer = &mut memory.internal_data[0..0x40000]; // a slice exactly as large as the bios
	match f.read(bios_buffer) {
		Ok(bytes) => println!("Read {} bytes into BIOS region.", bytes),
		Err(error) => panic!("Error while reading from file '{}': {}", BIOS_PATH, error)
	}

	println!("Loaded BIOS at {}.", BIOS_PATH);
}

pub fn load_rom(gba: &mut Gba, rom_path: String) {
	let filepath = rom_path;
	let mut f = match File::open(filepath.clone()) {
		Ok(file) => file,
		Err(error) => panic!("Error while opening file '{}': {}", filepath, error)
	};

	let mut buffer = Vec::<u8>::new();
	match f.read_to_end(&mut buffer) {
		Ok(_) => {},
		Err(error) => panic!("Error while reading file `{}`: {}", filepath, error)
	}
	gba.load_cartridge(buffer);
	println!("Loaded ROM {}.", filepath);
}

pub fn load_memory(rom_path: String) -> GbaMemory {
	let mut memory = GbaMemory::new();
	let filepath = rom_path;
	let mut f = match File::open(filepath.clone()) {
		Ok(file) => file,
		Err(error) => panic!("Error while opening file '{}': {}", filepath, error)
	};

	let mut buffer = Vec::<u8>::new();
	match f.read_to_end(&mut buffer) {
		Ok(_) => {},
		Err(error) => panic!("Error while reading file `{}`: {}", filepath, error)
	}
	memory.rom = buffer;
	return memory;
}

pub fn run_gba(gba: &mut Gba) {
	gba.run();
}

pub fn disasm_gba_rom(memory: &mut GbaMemory, thumb_mode: bool) {
	debug::print_gba_rom_disasm(memory, thumb_mode);
}

const USAGE: &'static str = "
Pyrite

Usage:
	pyrite <rom>
	pyrite (-d | --disasm) [(-t | --thumb)] <rom>
	pyrite (-h | --help)
	pyrite (-v | --version)

Options:
	-d --disasm     Disassembles the ROM.
	-t --thumb      Will disassemble in thumb mode.
	-h --help       Show this screen.
	-v --version    Prints the version and exits.
";


#[derive(Default)]
struct Args {
	arg_rom: Option<String>,
	flag_version: bool,
	flag_disasm: bool,
	flag_thumb: bool,
	flag_help: bool
}

fn main() {
	let mut args: Args = Default::default();
	let args_list: Vec<String> = env::args().collect();

	for (index, arg) in args_list.iter().enumerate() {
		if index == 0 { continue; } // This is the executable.
		if arg.starts_with("-") {
			match arg.as_ref() {
				"-d" | "--disasm"	=> args.flag_disasm = true,
				"-t" | "--thumb"	=> args.flag_thumb = true,
				"-v" | "--version"	=> args.flag_version = true,
				"-h" | "--help"		=> args.flag_help = true,
				_ => { panic!("Unexpected option {}", arg) }
			}
		} else {
			args.arg_rom = Some(arg.clone());
		}
	}

	if args.flag_help || args_list.len() < 1{
		println!("{}", USAGE);
		return;
	}

	if args.flag_version {
		println!("pyrite version \"0.1.0\"");
		return;
	}

	if let Some(rom_file) = args.arg_rom {
		println!("Emulating ROM: {}", rom_file);
		if args.flag_disasm {
			let mut memory = load_memory(rom_file);
			// load_bios(&mut memory);
			disasm_gba_rom(&mut memory, args.flag_thumb);
		} else {
			let mut gba = Box::new(Gba::new());
			load_bios(&mut gba.cpu.memory);
			load_rom(&mut gba, rom_file);
			run_gba(&mut gba);
		}
	} else {
		println!("NO ROM FILE PROVIDED.");
		println!("{}", USAGE);
	}
}
