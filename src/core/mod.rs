pub mod memory;
pub mod cpu;
pub mod lcd;
pub mod joypad;
pub mod device;

use self::memory::GbaMemory;

use self::cpu::registers;
use self::cpu::ArmCpu;
use self::device::GbaDevice;
use self::lcd::GbaLcd;

pub struct Gba<'a> {
	pub cpu: ArmCpu,
	pub lcd: GbaLcd,
	pub device: GbaDevice<'a>
}

impl<'a> Gba<'a> {
	pub fn new<'b>() -> Gba<'b> {
		Gba {
			cpu: ArmCpu::new(),
			lcd: GbaLcd::new(),
			device: GbaDevice::new()
		}
	}

	pub fn init(&mut self) {
		self.device.init();
	}

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.cpu.memory.rom = data;
	}

	pub fn run(&mut self) {
		match self.device.screen {
			Some(ref mut screen) => self.lcd.render_line(screen, &mut self.cpu.memory),
			None => unreachable!()
		}
		// self.cpu.registers.set(registers::REG_PC, 0x8000000);
		// // let mut x = 0;
		// while self.cpu.executable() {
		// 	self.cpu.tick();
		// 	// x += 1;
		// 	// if(x > 64) { break; }
		// }
	}
}