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

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.cpu.memory.rom = data;
	}

	pub fn run(&mut self) {
		'running: loop {
			self.lcd.render_line(&mut self.device.screen, &mut self.cpu.memory);
			if self.poll_device_events() {
				break 'running;
			}
		}
		// self.cpu.registers.set(registers::REG_PC, 0x8000000);
		// // let mut x = 0;
		// while self.cpu.executable() {
		// 	self.cpu.tick();
		// 	// x += 1;
		// 	// if(x > 64) { break; }
		// }
	}

	/// Polls for and handles events from the device.
	/// returns true if this should quit.
	fn poll_device_events(&mut self) -> bool {
		for event in self.device.event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					return true;
				},
				_ => {}
			}
		}
		return false;
	}
}