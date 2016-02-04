pub mod memory;
pub mod cpu;
pub mod lcd;
pub mod joypad;
pub mod device;

use glium;

use self::memory::*;
use self::cpu::registers;
use self::cpu::ArmCpu;
use self::device::GbaDevice;
use self::lcd::GbaLcd;
use self::joypad::GbaJoypad;
use ::util::frame_counter::FrameCounter;

/// delay for a 60fps frame in nanoseconds.
const FPS_60_DELTA_NS: u64 = 15670000; // 16670000

/// LCD V-Blank Interrupt
pub const INT_VBLANK: u16 = 0x01;

/// LCD H-Blank Interrupt
pub const INT_HBLANK: u16 = 0x02;

/// LCD V-Counter Match Interrupt
pub const INT_VCOUNT: u16 = 0x04;

/// Timer 0 Overflow Interrupt
pub const INT_TIMER0: u16 = 0x08;

/// Timer 1 Overflow Interrupt
pub const INT_TIMER1: u16 = 0x10;

/// Timer 2 Overflow Interrupt
pub const INT_TIMER2: u16 = 0x20;

/// Timer 3 Overflow Interrupt
pub const INT_TIMER3: u16 = 0x40;

/// Serial Communication Interrupt
pub const INT_SERIAL: u16 = 0x80;

/// DMA 0 Interrupt
pub const INT_DMA0: u16 = 0x100;

/// DMA 1 Interrupt
pub const INT_DMA1: u16 = 0x200;

/// DMA 2 Interrupt
pub const INT_DMA2: u16 = 0x400;

/// DMA 3 Interrupt
pub const INT_DMA3: u16 = 0x800;

/// Keypad Interrupt
pub const INT_KEYPAD: u16 = 0x1000;

/// Game Pak (external IRQ source) Interrupt
pub const INT_GAMEPAK: u16 = 0x2000;

pub struct Gba {
	pub cpu: ArmCpu,
	pub lcd: GbaLcd,
	pub device: GbaDevice,
	pub joypad: GbaJoypad,
	pub debug: GbaDebug,
	pub request_exit: bool
}

impl Gba {
	pub fn new() -> Gba {
		Gba {
			cpu: ArmCpu::new(),
			lcd: GbaLcd::new(),
			device: GbaDevice::new(),
			joypad: GbaJoypad::new(),
			debug: GbaDebug::new(),
			request_exit: false
		}
	}

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.cpu.memory.rom = data;
	}

	pub fn init(&mut self) {
		self.cpu.set_pc(0x8000000);

		self.cpu.registers.set_mode(registers::MODE_SYS);

		// #FIXME I this these should be set by the BIOS but doing it here for now.
		self.cpu.registers.set_with_mode(registers::MODE_USR, registers::REG_SP, 0x03007F00); // Also System
		self.cpu.registers.set_with_mode(registers::MODE_IRQ, registers::REG_SP, 0x03007FA0);
		self.cpu.registers.set_with_mode(registers::MODE_SVC, registers::REG_SP, 0x03007FE0);

		self.cpu.memory.set_reg(ioreg::KEYINPUT, 0xffff); // make sure all keys are marked as released.
	}

	pub fn run(&mut self) {
		self.init();
		let mut delta = 0;
		let mut gba_fps_counter = FrameCounter::new();
		'running: loop {
			if delta >= FPS_60_DELTA_NS {
				self.frame();
				gba_fps_counter.record_frame();
				// println!("delta is {}, resetting.", delta);
				delta = 0;

				{
					let fps = self.device.fps_counter.avg_fps;
					let speed = ((fps as f64) / 60f64) * 100f64;
					let window = self.device.display.get_window().expect("Failed to get device window.");
					window.set_title(&format!("Pyrite - [GBA: {} FPS] [{} FPS ({}% GBA)]", gba_fps_counter.avg_fps, fps, speed as i64));
				}
			}
			if self.request_exit { break 'running; }
			self.device.render(&self.lcd.screen_buffer);
			delta += self.device.fps_counter.last_delta;
			// println!("delta is {} (+{})", delta, self.device.fps_counter.last_delta);
		}
		self.request_exit = false; // in case we don't actually close here.
		println!("-- Shutdown successfully.");
	}

	fn frame(&mut self) {
		// Clears the VBlank flag.
		{
			let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
			dispstat &= !0x1;
			self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);
		}

	
		for vcount in 0..160 {
			self.cpu.memory.set_reg(ioreg::VCOUNT, vcount);
			self.check_line_coincidence(vcount);
			self.do_vdraw_line(vcount);
		}

		// Sets the VBlank flag.
		{
			let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
			dispstat |= 0x1;
			self.cpu.memory.set_reg(ioreg::DISPSTAT, dispstat);
		}

		// We do the first iteration of vblank here in order
		// to fire the interrupt once.
		self.cpu.memory.set_reg(ioreg::VCOUNT, 160);
		self.check_line_coincidence(160);
		self.try_fire_vblank_int();
		self.do_vblank_line();

		for vcount in 161..228 {
			self.cpu.memory.set_reg(ioreg::VCOUNT, vcount);
			self.check_line_coincidence(vcount);
			self.do_vblank_line();
		}
	}

	/// Attempts to fire an vblank interrupt
	/// if it is enabled in DISPSTAT
	/// the CPU handles checking the IME and IE registers.
	fn try_fire_vblank_int(&mut self) {
		let dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		if ((dispstat >> 3) & 1) != 0 {
			self.hardware_interrupt(INT_VBLANK);
		}
	}

	/// Attempts to fire an hblank interrupt
	/// if it is enabled in DISPSTAT
	/// the CPU handles checking the IME and IE registers.
	fn try_fire_hblank_int(&mut self) {
		let dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		if ((dispstat >> 4) & 1) != 0 {
			self.hardware_interrupt(INT_HBLANK);
		}
	}

	/// Taken From TONC:
	/// There are three registers specifically for interrupts: REG_IE (0400:0200h), 
	/// REG_IF (0400:0202h) and REG_IME (0400:0208h). REG_IME is the master interrupt control; 
	/// unless this is set to ‘1’, interrupts will be ignored completely. 
	/// To enable a specific interrupt you need to set the appropriate bit in REG_IE. 
	/// When an interrupt occurs, the corresponding bit in REG_IF will be set.
	fn hardware_interrupt(&mut self, mask: u16) {
		if !self.cpu.allow_irq_interrupt() { return; }
		let reg_ime = self.cpu.memory.get_reg(ioreg::IME);
		if reg_ime != 1 { return; } // We just stop here if IME is not 1.
		let reg_ie = self.cpu.memory.get_reg(ioreg::IE);
		if (reg_ie & mask) == 0 { return; } // This specific interrupt is not enabled.
		let mut reg_if = self.cpu.memory.get_reg(ioreg::IF);
		reg_if |= mask; // set the corresponding bit in IF.
		self.cpu.memory.set_reg(ioreg::IF, reg_if);
		self.cpu.irq_interrupt();
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
	fn do_vdraw_line(&mut self, line: u16) {
		self.poll_device_events();
		self.do_hdraw();
		self.lcd.render_line(&mut self.cpu.memory, line);
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

		self.try_fire_hblank_int();

		self.run_cpu_cycles(272);
	}

	fn run_cpu_cycles(&mut self, cycles: u64) {
		let target = self.cpu.clock.cycles + cycles;
		while self.cpu.clock.cycles < target {
			if self.cpu.executable() {
				self.cpu.tick();
				// self.debug.on_tick(&mut self.cpu);
			} else {
				panic!("Attempting to execute at unexecutable address 0x{:08x}!", self.cpu.get_exec_address());
			}
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
	fn poll_device_events(&mut self) {
		for event in self.device.display.poll_events() {
			match event {
				glium::glutin::Event::Closed => self.request_exit = true,
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::D)) => {
					self.cpu.reg_dump_pretty();
				},
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(keycode)) => {
					self.joypad.key_pressed(keycode);
				},
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(keycode)) => {
					self.joypad.key_released(keycode);
				},
				_ => {}
			}
		}
		self.joypad.tick(&mut self.cpu);
	}
}

pub struct GbaDebug {
	waiting_for_io: bool
}

impl GbaDebug {
	pub fn new() -> GbaDebug {
		GbaDebug { waiting_for_io: false }
	}

	pub fn on_tick(&mut self, cpu: &mut ArmCpu) {
		if self.waiting_for_io {
			let siodata32_h = cpu.memory.read16(0x4000122);
			if siodata32_h == 0xbeef {
				let siodata32_l = cpu.memory.read16(0x4000120);
				print!("{}", (siodata32_l as u8) as char);
				self.waiting_for_io = false;
			}
		} else {
			let siodata32_h = cpu.memory.read16(0x4000122);
			if siodata32_h == 0xdead {
				self.waiting_for_io = true;
			}
		}
	}
}



