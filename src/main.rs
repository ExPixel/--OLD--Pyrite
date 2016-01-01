// Warnings are off right now because of a lot
// of ununsed variables and dead code right now.
#[allow(warnings)]
pub mod core;
pub mod debug;

extern crate docopt;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use docopt::Docopt;

use core::Gba;

macro_rules! println_err {
    ($($arg:tt)*) => (
    	match writeln!(&mut ::std::io::stderr(), $($arg)*) {
    		Ok(_) => {},
    		Err(error) => panic!("Failed to write to stderr: {}", error)
    	}
    )
}

pub fn load_rom(rom_path: String) -> Gba {
	let mut gba = Gba::new();
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
	return gba
}

pub fn run_gba(gba: &mut Gba) {
	gba.run();
}

pub fn disasm_gba_rom(gba: &mut Gba, thumb_mode: bool) {
	debug::print_gba_rom_disasm(gba, thumb_mode);
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

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_rom: Option<String>,
	flag_version: bool,
	flag_disasm: bool,
	flag_thumb: bool
}

fn main() {
	let args: Args = Docopt::new(USAGE)
							.and_then(|d| d.decode())
							.unwrap_or_else(|e| e.exit());

	if args.flag_version {
		println!("pyrite version \"0.1.0\"");
		return;
	}

	if let Some(rom_file) = args.arg_rom {
		let mut gba = Box::new(load_rom(rom_file));
		if args.flag_disasm {
			disasm_gba_rom(&mut gba, args.flag_thumb);
		} else {
			run_gba(&mut gba);
		}
	}
}
