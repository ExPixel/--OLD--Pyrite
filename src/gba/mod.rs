pub mod core;
pub mod hw;
pub mod device;

use glium;
use time;

use std::thread;
use std::time::Duration;
use self::core::memory::*;
use self::core::cpu::registers;
use self::core::cpu::ArmCpu;
use self::device::GbaDevice;
use self::hw::lcd::GbaLcd;
use self::hw::joypad::GbaJoypad;
use self::hw::dma::*;

// #TODO remove this debug code.
// I'm using vsync now and because my monitor's refresh rate is 60Hz
// I don't limit the FPS, this is bad to leave false though :P
const LIMIT_FPS: bool = false;

/// delay for a 60fps frame in nanoseconds.
const FPS_60_DELTA_NS: u64 = 16000000; // 16666667

/// #TODO remove this debug code.
/// true if the starting address should be 0 in SVC mode.
const STARTUP_BIOS: bool = false;

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
	pub dma_handler: DmaHandler,
	pub request_exit: bool
}

impl Gba {
	pub fn new() -> Gba {
		Gba {
			cpu: ArmCpu::new(),
			lcd: GbaLcd::new(),
			device: GbaDevice::new(),
			joypad: GbaJoypad::new(),
			dma_handler: DmaHandler::new(),
			request_exit: false
		}
	}

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.cpu.memory.rom = data;
	}

	pub fn init(&mut self) {
		self.cpu.registers.setf_f(); // The FIQ flag should always be high.

		if STARTUP_BIOS {
			self.cpu.set_pc(0x00000000);
			self.cpu.registers.set_mode(registers::MODE_SVC);
		} else {
			self.cpu.set_pc(0x08000000);
			self.cpu.registers.set_mode(registers::MODE_SYS);
			self.cpu.registers.set_with_mode(registers::MODE_USR, registers::REG_SP, 0x03007F00); // Also System
			self.cpu.registers.set_with_mode(registers::MODE_IRQ, registers::REG_SP, 0x03007FA0);
			self.cpu.registers.set_with_mode(registers::MODE_SVC, registers::REG_SP, 0x03007FE0);
			// #TODO some IO registers need to be set here.
		}

		self.cpu.memory.set_reg(ioreg::KEYINPUT, 0xffff); // make sure all keys are marked as released.
	}

	pub fn run(&mut self) {
		self.init();
		let mut frame: u64 = 0;
		'running: loop {
			if LIMIT_FPS {
				let start_time = time::precise_time_ns();
				self.tick(frame);
				let delta = time::precise_time_ns() - start_time;
				let sleep_time_millis = if delta > FPS_60_DELTA_NS { 0 } else { FPS_60_DELTA_NS - delta } / 1000000;
				thread::sleep(Duration::from_millis(sleep_time_millis));
			} else { // #TODO remove debug code.
				self.tick(frame);
			}
			if self.request_exit { break 'running; }
			frame += 1;
		}
		self.request_exit = false; // in case we don't actually close here.
		println!("-- Shutdown successfully.");
	}

	fn tick(&mut self, frame: u64) {
		let frame_start_time = time::precise_time_ns();
		self.frame();
		let render_start_time = time::precise_time_ns();
		self.device.render(&self.lcd.screen_buffer);
		let end_time = time::precise_time_ns();
		self.update_window_title(frame, frame_start_time as f64, render_start_time as f64, end_time as f64);
	}

	fn update_window_title(&mut self, frame: u64, frame_start_time: f64, render_start_time: f64, end_time: f64) {
		let frame_build_time = (render_start_time - frame_start_time) / 1000000.0;
		let render_to_screen_time = (end_time - render_start_time) / 1000000.0;
		let frame_build_and_render_time = (end_time - frame_start_time) / 1000000.0;
		let window = self.device.display.get_window().expect("Failed to get device window.");
		window.set_title(&format!("Pyrite - {:.1}ms [f: {:.1}ms, r: {:.1}ms]", frame_build_and_render_time, frame_build_time, render_to_screen_time));
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
		self.check_dmas(DMA_TIMING_VBLANK);
		self.do_vblank_line();

		for vcount in 161..228 {
			self.cpu.memory.set_reg(ioreg::VCOUNT, vcount);
			self.check_line_coincidence(vcount);
			self.do_vblank_line();
		}

		self.on_frame_end();
	}

	fn on_frame_end(&mut self) {
		self.cpu.memory.internal_regs.on_frame_end(
			&self.cpu.memory.internal_data[MEM_IOREG.local_addr..(MEM_IOREG.local_addr+MEM_IOREG.size)]
		);
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

	fn check_dmas(&mut self, timing: u16) {
		// We only check if the DMA registers are dirty if the timing is immediate
		// otherwise we try to start the DMA anyway.
		if timing != DMA_TIMING_IMMEDIATE || self.cpu.memory.internal_regs.dma_dirty {
			let mut interrupt = None;
			let mut dirty = false;

			if self.dma_handler.try_start_dma(&mut self.cpu, timing, 0) {
				let dma_cnt_h = self.cpu.memory.get_reg(ioreg::DMA0CNT_H);
				if ((dma_cnt_h >> 14) & 1) != 0 { // If DMA IRQ enabled.
					interrupt = Some(INT_DMA0);
				}
				if ((dma_cnt_h >> 9) & 1) != 0 { // If DMA repeat was enabled.
					dirty |= true;
				}
			}

			if self.dma_handler.try_start_dma(&mut self.cpu, timing, 1) {
				let dma_cnt_h = self.cpu.memory.get_reg(ioreg::DMA1CNT_H);
				if ((dma_cnt_h >> 14) & 1) != 0 { // If DMA IRQ enabled.
					interrupt = Some(INT_DMA1);
				}
				if ((dma_cnt_h >> 9) & 1) != 0 { // If DMA repeat was enabled.
					dirty |= true;
				}
			}

			if self.dma_handler.try_start_dma(&mut self.cpu, timing, 2) {
				let dma_cnt_h = self.cpu.memory.get_reg(ioreg::DMA2CNT_H);
				if ((dma_cnt_h >> 14) & 1) != 0 { // If DMA IRQ enabled.
					interrupt = Some(INT_DMA2);
				}
				if ((dma_cnt_h >> 9) & 1) != 0 { // If DMA repeat was enabled.
					dirty |= true;
				}
			}

			if self.dma_handler.try_start_dma(&mut self.cpu, timing, 3) {
				let dma_cnt_h = self.cpu.memory.get_reg(ioreg::DMA3CNT_H);
				if ((dma_cnt_h >> 14) & 1) != 0 { // If DMA IRQ enabled.
					interrupt = Some(INT_DMA3);
				}
				if ((dma_cnt_h >> 9) & 1) != 0 { // If DMA repeat was enabled.
					dirty |= true;
				}
			}

			self.cpu.memory.internal_regs.dma_dirty = dirty;

			if let Some(mask) = interrupt { self.hardware_interrupt(mask) }
		}
	}

	/// Taken From TONC:
	/// There are three registers specifically for interrupts: REG_IE (0400:0200h), 
	/// REG_IF (0400:0202h) and REG_IME (0400:0208h). REG_IME is the master interrupt control; 
	/// unless this is set to ‘1’, interrupts will be ignored completely. 
	/// To enable a specific interrupt you need to set the appropriate bit in REG_IE. 
	/// When an interrupt occurs, the corresponding bit in REG_IF will be set.
	fn hardware_interrupt(&mut self, mask: u16) {
		let reg_ime = self.cpu.memory.get_reg(ioreg::IME);
		if reg_ime != 1 { return; } // We just stop here if IME is not 1.
		let reg_ie = self.cpu.memory.get_reg(ioreg::IE);
		if (reg_ie & mask) == 0 { return; } // This specific interrupt is not enabled.
		self.wake_up_cpu(mask); // At this point the CPU can wake up.
		let mut reg_if = self.cpu.memory.get_reg(ioreg::IF);
		reg_if |= mask; // set the corresponding bit in IF.
		self.cpu.memory.set_reg(ioreg::IF, reg_if);
		if !self.cpu.allow_irq_interrupt() { return; }
		self.cpu.irq_interrupt();
	}

	/// Wakes up the CPU if it was halted.
	fn wake_up_cpu(&mut self, mask: u16) {
		// // #TODO remove testing code:
		// if self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped {
		// 	println!("WAKING UP CPU: 0x{:0x}", mask);
		// }
		self.cpu.memory.internal_regs.halted = false;
		self.cpu.memory.internal_regs.stopped = false; // Not sure if this is supposed to be here.
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
		self.check_dmas(DMA_TIMING_HBLANK);
		self.run_cpu_cycles(272);
	}

	fn run_cpu_cycles(&mut self, cycles: u64) {
		// #TODO have another look at this one.
		if self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped { return }

		let mut cycles = cycles;
		let mut target = self.cpu.clock.cycles + cycles;
		'cpu_loop: while self.cpu.clock.cycles < target {
			if self.cpu.executable() {
				if self.dma_handler.dma_cycles > 0 {
					if cycles < self.dma_handler.dma_cycles {
						self.dma_handler.dma_cycles -= cycles;
						break 'cpu_loop;
					} else {
						cycles -= self.dma_handler.dma_cycles;
						target = self.cpu.clock.cycles + cycles; // recalculate the target
						self.dma_handler.dma_cycles = 0;
					}
				} else {
					self.cpu.tick();
					// Now sure about this one either.
					if self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped { return }

					// #TODO I should check if the DMA is registers are dirty or something.
					// This loses me about 40-50 FPS. I could probably check by using something in the memory
					// to check if any of the ioregisters are dirty.
					self.check_dmas(DMA_TIMING_IMMEDIATE);
				}
			} else {
				self.cpu.reg_dump_pretty();
				panic!("Attempting to execute at unexecutable address 0x{:08x}!", self.cpu.get_exec_address());
			}
		}
	}


	/// Polls for and handles events from the device.
	/// returns true if this should quit.
	fn poll_device_events(&mut self) {
		for event in self.device.display.poll_events() {
			match event {
				glium::glutin::Event::Closed => self.request_exit = true,
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Escape)) => {
					self.request_exit = true
				},
				glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::D)) => {
					match state {
						glium::glutin::ElementState::Pressed => set_pyrite_dyn_debug!(true),
						_ => set_pyrite_dyn_debug!(false)
					}
				},
				glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::R)) => {
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

