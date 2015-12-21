#[allow(warnings)]
pub mod core;

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

pub fn run_rom(rom_path: String) {
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
}

const USAGE: &'static str = "
Pyrite

Usage:
	pyrite <rom>
	pyrite (-h | --help)
	pyrite (-v | --version)

Options:
	-h --help    Show this screen.
	-v --version    Prints the version and exits.
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_rom: Option<String>,
	flag_version: bool
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
		run_rom(rom_file);
	}
}
