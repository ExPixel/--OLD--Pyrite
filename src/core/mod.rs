pub mod memory;
pub mod cpu;
pub mod lcd;
use self::memory::GbaMemory;

use self::cpu::registers;
use self::cpu::ArmCpu;

pub struct Gba {
	pub cpu: ArmCpu
}

impl Gba {
	pub fn new() -> Gba {
		Gba {
			cpu: ArmCpu::new()
		}
	}

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.cpu.memory.rom = data;
	}

	pub fn run(&mut self) {
		self.cpu.registers.set(registers::REG_PC, 0x8000000);
		// let mut x = 0;
		while self.cpu.executable() {
			self.cpu.tick();
			// x += 1;
			// if(x > 64) { break; }
		}
	}
}