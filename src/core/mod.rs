pub mod memory;
pub mod cpu;
pub mod lcd;

use self::memory::GbaMemory;

pub struct Gba {
	memory: GbaMemory
}

impl Gba {
	pub fn new() -> Gba {
		Gba {
			memory: GbaMemory::new()
		}
	}

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.memory.rom = data;
	}
}