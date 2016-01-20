pub mod memory;
pub mod cpu;
pub mod lcd;
pub mod joypad;
pub mod device;

use self::memory::*;
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

	pub fn init(&mut self) {
		self.cpu.registers.set(registers::REG_PC, 0x8000000);

		self.cpu.registers.set_mode(registers::MODE_SYS);

		// #FIXME I this these should be set by the BIOS but doing it here for now.
		self.cpu.registers.set_with_mode(registers::REG_SP, registers::MODE_USR, 0x03007F00); // Also System
		self.cpu.registers.set_with_mode(registers::REG_SP, registers::MODE_IRQ, 0x03007FA0);
		self.cpu.registers.set_with_mode(registers::REG_SP, registers::MODE_SVC, 0x03007FE0);
	}

	pub fn run(&mut self) {
		self.init();
		self.device.render();
		'running: loop {
			self.frame();
			self.device.render();
			if self.poll_device_events() { break 'running; }

			{
				let mut window = self.device.renderer.window_mut().expect("Failed to get mutable window reference.");
				let title = format!("Pyrite - {} FPS", self.device.fps_counter.fps);
				window.set_title(&title);
			}
		}
		println!("-- Shutdown successfully.");
		// self.cpu.registers.set(registers::REG_PC, 0x8000000);
		// // let mut x = 0;
		// while self.cpu.executable() {
		// 	self.cpu.tick();
		// 	// x += 1;
		// 	// if(x > 64) { break; }
		// }
	}

	fn frame(&mut self) {
		// Clears the VBlank flag.
		{
			let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
			dispstat &= !0x1;
			self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);
		}

		unsafe {
			// #TODO The borrow checker got annoying, will fix this at some point.
			let dd = &mut self.device as *mut GbaDevice;
			(*dd).gba_screen.with_lock(None, |buffer: &mut [u8], pitch: usize| {		
				for vcount in 0..160 {
					self.cpu.memory.set_reg(ioreg::VCOUNT, vcount);
					self.check_line_coincidence(vcount);

					let line_data_off = (vcount as usize) * pitch;

					self.do_vdraw_line(vcount, &mut buffer[line_data_off..(line_data_off + pitch)]);
				}
			}).expect("Failed to aquire texture lock.");
		}

		// Sets the VBlank flag.
		{
			let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
			dispstat |= 0x1;
			self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);
		}

		for vcount in 160..228 {
			self.cpu.memory.set_reg(ioreg::VCOUNT, vcount);
			self.check_line_coincidence(vcount);
			self.do_vblank_line();
		}
	}

	fn check_line_coincidence(&mut self, vcount: u16) {
		let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		if ((dispstat >> 8) & 0xf) == vcount {
			dispstat |= 0x4; // Sets the V-Counter flag
		} else {
			dispstat &= !0x4; // Clears the V-Counter flag
		}
		self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);
	}

	/// Horizontal Dimensions
	/// The drawing time for each dot is 4 CPU cycles.
	///   Visible     240 dots,  57.221 us,    960 cycles - 78% of h-time
	///   H-Blanking   68 dots,  16.212 us,    272 cycles - 22% of h-time
	///   Total       308 dots,  73.433 us,   1232 cycles - ca. 13.620 kHz
	/// VRAM and Palette RAM may be accessed during H-Blanking. OAM can accessed only if "H-Blank Interval Free" bit in DISPCNT register is set.
	/// 
	/// Vertical Dimensions
	///   Visible (*) 160 lines, 11.749 ms, 197120 cycles - 70% of v-time
	///   V-Blanking   68 lines,  4.994 ms,  83776 cycles - 30% of v-time
	///   Total       228 lines, 16.743 ms, 280896 cycles - ca. 59.737 Hz
	/// All VRAM, OAM, and Palette RAM may be accessed during V-Blanking.
	/// Note that no H-Blank interrupts are generated within V-Blank period.
	fn do_vdraw_line(&mut self, line: u16, line_buffer: &mut [u8]) {
		self.do_hdraw();
		self.lcd.render_line(&mut self.cpu.memory, line, line_buffer);
		self.do_hblank();
	}

/*
4000004h - DISPSTAT - General LCD Status (Read/Write)
Display status and Interrupt control. The H-Blank conditions are generated once per scanline, including for the 'hidden' scanlines during V-Blank.
  Bit   Expl.
  0     V-Blank flag   (Read only) (1=VBlank) (set in line 160..226; not 227)
  1     H-Blank flag   (Read only) (1=HBlank) (toggled in all lines, 0..227)
  2     V-Counter flag (Read only) (1=Match)  (set in selected line)     (R)
  3     V-Blank IRQ Enable         (1=Enable)                          (R/W)
  4     H-Blank IRQ Enable         (1=Enable)                          (R/W)
  5     V-Counter IRQ Enable       (1=Enable)                          (R/W)
  6     Not used (0) / DSi: LCD Initialization Ready (0=Busy, 1=Ready)   (R)
  7     Not used (0) / NDS: MSB of V-Vcount Setting (LYC.Bit8) (0..262)(R/W)
  8-15  V-Count Setting (LYC)      (0..227)                            (R/W)
*/

	fn do_vblank_line(&mut self) {
		self.run_cpu_cycles(1232);
	}

	fn do_hdraw(&mut self) {
		// Clears the HBlank flag:
		let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		dispstat &= !0x2;
		self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);

		self.run_cpu_cycles(960);
	}

	fn do_hblank(&mut self) {
		// Sets the HBlank flag:
		let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		dispstat |= 0x2;
		self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);

		self.run_cpu_cycles(272);
	}

	fn run_cpu_cycles(&mut self, cycles: u64) {
		let target = self.cpu.clock.cycles + cycles;
		while self.cpu.clock.cycles < target {
			if !self.cpu.executable() {
				panic!("Attempting to execute at unexecutable address 0x{:08x}!", self.cpu.get_exec_address());
			}
			self.cpu.tick();
		}
	}

	// fn __debug_init_texture(&mut self) {
	// 	self.device.gba_screen.with_lock(None, |buffer: &mut [u8], pitch: usize| {
	// 		for y in 0..160 {
	// 			for x in 0..240 {
	// 				let offset = y*pitch + x*3;
	// 				buffer[offset + 0] = x as u8;
	// 				buffer[offset + 1] = y as u8;
	// 				buffer[offset + 2] = ((x + y) / 2) as u8;
	// 			}
	// 		}
	// 	}).expect("Failed to aquire texture lock.");
	// }

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