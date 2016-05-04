pub mod core;
pub mod hw;
pub mod device;
pub mod serialization;


use glium::glutin::{Event, ElementState, VirtualKeyCode};
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
use self::serialization::*;

use super::debug::debugger::GbaDebugger;

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

/// Keypad Interrupt
pub const INT_KEYPAD: u16 = 0x1000;

/// Game Pak (external IRQ source) Interrupt
pub const INT_GAMEPAK: u16 = 0x2000;

/// State passed to the GUI.
#[derive(Default)]
pub struct GbaExtras {
	paused: bool,
	request_pause: bool,

	pub request_debugger: bool,

	// #TODO remove temporary code.
	request_save_state: u8 // 0 - nothing, 1 - save, 2 - load
}

impl GbaExtras {
	pub fn new() -> GbaExtras {
		GbaExtras {
			paused: false,
			request_pause: false,
			request_debugger: false,

			// #TODO remove temporary code.
			request_save_state: 0
		}
	}
}

pub struct Gba {
	pub cpu: ArmCpu,
	pub lcd: GbaLcd,
	pub device: GbaDevice,
	pub joypad: GbaJoypad,
	pub request_exit: bool,
	pub extras: GbaExtras
}

impl Gba {
	pub fn new() -> Gba {
		Gba {
			cpu: ArmCpu::new(),
			lcd: GbaLcd::new(),
			device: GbaDevice::new(),
			joypad: GbaJoypad::new(),
			request_exit: false,
			extras: GbaExtras::new()
		}
	}

	pub fn load_cartridge(&mut self, data: Vec<u8>) {
		self.cpu.memory.rom = data;
	}

	pub fn init(&mut self) {
		self.cpu.registers.setf_f(); // The FIQ flag should always be high.

		if STARTUP_BIOS {
			self.cpu.registers.setf_i(); // Disables IRQ interrupts.
			self.cpu.registers.setf_f(); // Disables FIQ interrupts. (They are impossible on the GBA, but this is high by default.)
			self.cpu.set_pc(0x00000000);
			self.cpu.registers.set_mode(registers::MODE_SVC);
		} else {
			self.cpu.set_pc(0x08000000);
			self.cpu.registers.setf_i(); // Disables IRQ interrupts.
			self.cpu.registers.setf_f(); // Disables FIQ interrupts. (They are impossible on the GBA, but this is high by default.)
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

		// #TODO remove temporary code.
		// Because of the current way the Gba ticks,
		// this should be done in the end of a frame to preserve what
		// little timing we have.
		if self.extras.request_save_state != 0 {
			if self.extras.request_save_state == 1 {
				self.save_to_file();
				println!("Saved state to file.");
			} else if self.extras.request_save_state == 2 {
				self.load_from_file();
				println!("Loaded state from file.");
			}
			self.extras.request_save_state = 0;
		}

		if self.extras.request_debugger {
			self.extras.request_debugger = false;
			GbaDebugger::new(self).start();
		}
	}

	fn update_window_title(&mut self, _: u64, frame_start_time: f64, render_start_time: f64, end_time: f64) {
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
		if self.extras.paused != self.extras.request_pause {
			self.extras.paused = self.extras.request_pause;
			if self.extras.paused { println!("Paused."); }
			else { println!("Unpaused"); }
		}

		pyrite_debugging!({
			print_memory_table!(self.cpu.memory, 0x0600FB60, 0x0600FB60 + 63);
		});
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
			self.cpu.dma_check_started(timing, 0);
			self.cpu.dma_check_started(timing, 1);
			self.cpu.dma_check_started(timing, 2);
			self.cpu.dma_check_started(timing, 3);
			
			self.cpu.memory.internal_regs.dma_dirty = false;	
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
		self.wake_up_cpu(); // At this point the CPU can wake up.
		let mut reg_if = self.cpu.memory.get_reg(ioreg::IF);
		reg_if |= mask; // set the corresponding bit in IF.
		self.cpu.memory.set_reg(ioreg::IF, reg_if);
		if !self.cpu.allow_irq_interrupt() { return; }
		self.cpu.irq_interrupt();
	}

	/// Wakes up the CPU if it was halted.
	fn wake_up_cpu(&mut self) {
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
		if self.extras.paused { return }

		if self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped { return }
		let target = self.cpu.clock.cycles + cycles;

		//dma_ongoing(&mut self.cpu)

		'cpu_loop: while self.cpu.clock.cycles < target {
			if self.cpu.dma_ongoing() {
				let dma_int_mask = self.cpu.dma_tick();
				if dma_int_mask != 0 {
					self.hardware_interrupt(dma_int_mask);
				}
			} else {
				if self.cpu.executable() {
					self.cpu.tick();
					self.increment_timers();
					if self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped { return }
					self.check_dmas(DMA_TIMING_IMMEDIATE);
				} else {
					self.cpu.reg_dump_pretty();
					panic!("Attempting to execute at unexecutable address 0x{:08x}!", self.cpu.get_exec_address());
				}
			}
		}
	}

	fn increment_timers(&mut self) {
		let timer_inc = self.cpu.clock.timer_cycles;
		self.cpu.clock.timer_cycles = 0;
		let overflow_int_mask = self.cpu.memory.internal_regs.increment_timers(timer_inc);
		if overflow_int_mask != 0 {
			self.hardware_interrupt(overflow_int_mask);
		}
	}


	/// Polls for and handles events from the device.
	/// returns true if this should quit.
	fn poll_device_events(&mut self) {
		for event in self.device.display.poll_events() {
			match event {
				Event::Closed => self.request_exit = true,
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
					self.request_exit = true
				},

			// GENERAL DEBUGGING STUFF:
				Event::KeyboardInput(state, _, Some(VirtualKeyCode::D)) => {
					match state {
						ElementState::Pressed => set_pyrite_dyn_debug!(true),
						_ => set_pyrite_dyn_debug!(false)
					}
				},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::R)) => {
					self.cpu.reg_dump_pretty();
				},

				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::P)) => {
					self.extras.request_pause = !self.extras.paused;
				},
				
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Comma)) => { // The '<' key for me.
					self.extras.request_save_state = 1;
					println!("Requested Save...");
				},
				
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Period)) => { // The '>' key for me.
					self.extras.request_save_state = 2;
					println!("Requested Load...");
				},
				
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::H)) => { // The '>' key for me.
					self.extras.request_debugger = true;
					println!("Request Debugger...");
				},

			// DEBUGGING LAYERS IN GRAPHICS:
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key1)) => {debug_toggle_layer!(0);},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key2)) => {debug_toggle_layer!(1);},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key3)) => {debug_toggle_layer!(2);},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key4)) => {debug_toggle_layer!(3);},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key5)) => {debug_toggle_layer!(4);},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::L)) => {debug_turn_off_all_layers!();},
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::K)) => {debug_turn_on_all_layers!();},

			// ACTUAL GBA SHIT:
				Event::KeyboardInput(ElementState::Pressed, _, Some(keycode)) => {
					self.joypad.key_pressed(keycode);
				},
				Event::KeyboardInput(ElementState::Released, _, Some(keycode)) => {
					self.joypad.key_released(keycode);
				},
				_ => {}
			}
		}

		if self.joypad.tick(&mut self.cpu) {
			self.hardware_interrupt(INT_KEYPAD);
		}
	}
}

