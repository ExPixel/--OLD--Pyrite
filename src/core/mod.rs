pub mod memory;
pub mod cpu;
pub mod lcd;
pub mod joypad;
pub mod device;

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
		self.cpu.registers.set(registers::REG_PC, 0x8000000);
		self.__debug_init_texture();
		self.device.render();
		'running: loop {
			self.frame();
			self.device.render();
			if self.poll_device_events() { break 'running; }
		}
		// self.cpu.registers.set(registers::REG_PC, 0x8000000);
		// // let mut x = 0;
		// while self.cpu.executable() {
		// 	self.cpu.tick();
		// 	// x += 1;
		// 	// if(x > 64) { break; }
		// }
	}

	fn frame(&mut self) {
		for _ in 0..160	{ self.do_vdraw_line(); }
		for _ in 0..68	{ self.do_vblank_line(); }
	}

	/*
	* Horizontal Dimensions
	* The drawing time for each dot is 4 CPU cycles.
	*   Visible     240 dots,  57.221 us,    960 cycles - 78% of h-time
	*   H-Blanking   68 dots,  16.212 us,    272 cycles - 22% of h-time
	*   Total       308 dots,  73.433 us,   1232 cycles - ca. 13.620 kHz
	* VRAM and Palette RAM may be accessed during H-Blanking. OAM can accessed only if "H-Blank Interval Free" bit in DISPCNT register is set.
	* 
	* Vertical Dimensions
	*   Visible (*) 160 lines, 11.749 ms, 197120 cycles - 70% of v-time
	*   V-Blanking   68 lines,  4.994 ms,  83776 cycles - 30% of v-time
	*   Total       228 lines, 16.743 ms, 280896 cycles - ca. 59.737 Hz
	* All VRAM, OAM, and Palette RAM may be accessed during V-Blanking.
	* Note that no H-Blank interrupts are generated within V-Blank period.
	*/

	fn do_vdraw_line(&mut self) {
		// #TODO set io registers
		self.do_hdraw();
		self.lcd.render_line(&mut self.device.gba_screen, &mut self.cpu.memory);
		self.do_hblank();
	}

	fn do_vblank_line(&mut self) {
		// #TODO set io registers
		self.run_cpu_cycles(1232);
	}

	fn do_hdraw(&mut self) {
		// #TODO set io registers
		self.run_cpu_cycles(960);
	}

	fn do_hblank(&mut self) {
		// #TODO set io registers
		self.run_cpu_cycles(272);
	}

	fn run_cpu_cycles(&mut self, cycles: u64) {
		let target = self.cpu.clock.cycles + cycles;
		while self.cpu.clock.cycles < target {
			self.cpu.tick();
		}
	}

	fn __debug_init_texture(&mut self) {
		self.device.gba_screen.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			for y in 0..160 {
				for x in 0..240 {
					let offset = y*pitch + x*3;
					buffer[offset + 0] = x as u8;
					buffer[offset + 1] = y as u8;
					buffer[offset + 2] = ((x + y) / 2) as u8;
				}
			}
		}).expect("Failed to aquire texture lock.");
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