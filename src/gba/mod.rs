pub mod core;
pub mod hw;
pub mod device;
pub mod serialization;
use time;

use std::thread;
use std::time::Duration;
use ::util::measure::*;
use self::core::memory::*;
use self::core::cpu::registers;
use self::core::cpu::ArmCpu;
use self::device::GbaDevice;
use self::hw::lcd::GbaLcd;
use self::hw::joypad::GbaJoypad;
use self::hw::dma;
use self::hw::audio;
use self::hw::timers;

use super::debug::debugger;
// use super::debug::debugger::GbaDebugger;

// #TODO remove this debug code.
// I'm using vsync now and because my monitor's refresh rate is 60Hz
// I don't limit the FPS, this is bad to leave false though :P
const LIMIT_FPS: bool = false;

/// The number of CPU ticks per audio tick.
/// The higher this value is, the better the audio will sound for some games
/// that frequenty change the value in the sound registers (e.g. Fire Emblem).
const AUDIO_TICK_RATE: u32 = 6144;

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
}

impl GbaExtras {
	pub fn new() -> GbaExtras {
		GbaExtras {
			paused: false,
			request_pause: false,
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
			extras: GbaExtras::new(),
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
		'running: loop {
			if LIMIT_FPS {
				let start_time = time::precise_time_ns();
				self.tick();
				let delta = time::precise_time_ns() - start_time;
				let sleep_time_millis = if delta > FPS_60_DELTA_NS { 0 } else { FPS_60_DELTA_NS - delta } / 1000000;
				thread::sleep(Duration::from_millis(sleep_time_millis));
			} else { // #TODO remove debug code.
				self.tick();
			}
			if self.request_exit { break 'running; }
		}
		self.request_exit = false; // in case we don't actually close here.

		// It's important that we do this so that we don't leak things
		// beyond our comprehension.
		self.device.close();

		debug_info!("-- Shutdown successfully.");
	}

	pub fn tick(&mut self) {
		let frame_start_time = time::precise_time_ns();
		if !self.extras.paused {
			self.frame();
		} else {
			self.poll_device_events();
		}

		// IMGUI:
		self.device.video.prepare_imgui();

		let render_start_time = time::precise_time_ns();
		debugger::render_debugger(self);
		self.device.video.render(&self.lcd.screen_buffer);
		let render_end_time = time::precise_time_ns();

		let mut debugger = debugger::get_debugger();
		debugger.frame_build_time = (render_start_time - frame_start_time) as f64 / 1000000.0;
		debugger.frame_render_time = (render_end_time - render_start_time) as f64 / 1000000.0;
		debugger.full_frame_time = (render_end_time - frame_start_time) as f64 / 1000000.0;

		if self.extras.paused != self.extras.request_pause {
			self.extras.paused = self.extras.request_pause;
			if self.extras.paused { console_warn!("Paused"); }
			else { console_warn!("Unpaused"); }
		}
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
		self.check_dmas(dma::DMA_TIMING_VBLANK);
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

		// pyrite_debugging!({
		// 	use std::sync::atomic::Ordering;
		// 	let _write_misses = self.device.audio.ring_buffer._stat_write_misses.load(Ordering::Relaxed);
		// 	let _read_misses = self.device.audio.ring_buffer._stat_read_misses.load(Ordering::Relaxed);

		// 	let _wd = _write_misses as u64 - pyrite_counter_get!(4);
		// 	let _rd = _read_misses as u64 - pyrite_counter_get!(5);

		// 	pyrite_counter_set!(4, _write_misses);
		// 	pyrite_counter_set!(5, _read_misses);

		// 	println!("Audio Write Misses: {} (+{})", _write_misses, _wd);
		// 	println!("Audio Read Misses: {} (+{})", _read_misses, _rd);
		// });
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

	fn try_fire_vcounter_int(&mut self) {
		let dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		if ((dispstat >> 5) & 1) != 0 {
			self.hardware_interrupt(INT_VCOUNT);
		}
	}

	fn check_dmas(&mut self, timing: u16) {
		// We only check if the DMA registers are dirty if the timing is immediate
		// otherwise we try to start the DMA anyway.
		if timing != dma::DMA_TIMING_IMMEDIATE || self.cpu.memory.internal_regs.dma_dirty {
			dma::check_started(&mut self.cpu, timing, 0);
			dma::check_started(&mut self.cpu, timing, 1);
			dma::check_started(&mut self.cpu, timing, 2);
			dma::check_started(&mut self.cpu, timing, 3);
			
			self.cpu.memory.internal_regs.dma_dirty = false;	
		}
	}

	fn hardware_interrupt(&mut self, mask: u16) {
		self.cpu.hardware_interrupt(mask);
	}

	fn check_line_coincidence(&mut self, vcount: u16) {
		let mut dispstat = self.cpu.memory.get_reg(ioreg::DISPSTAT);
		if ((dispstat >> 8) & 0xf) == vcount {
			dispstat |= 0x4; // Sets the V-Counter flag
			self.try_fire_vcounter_int();
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
		self.check_dmas(dma::DMA_TIMING_HBLANK);
		self.run_cpu_cycles(272);
	}

	fn run_cpu_cycles(&mut self, cycles: u64) {
		if self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped { return }
		let target = self.cpu.clock.cycles + cycles;

		//dma_ongoing(&mut self.cpu)

		measure_start(MEASURE_CPU_TICKS_TIME);
		measure_start(MEASURE_DMA_TICKS_TIME);

		'cpu_loop: while self.cpu.clock.cycles < target  && !(self.cpu.memory.internal_regs.halted || self.cpu.memory.internal_regs.stopped) {
			if dma::ongoing(&self.cpu) {
				measure_iteration(MEASURE_DMA_TICKS_TIME);
				dma::tick(&mut self.cpu);
			} else {
				if self.cpu.executable() {
					measure_iteration(MEASURE_CPU_TICKS_TIME);
					self.cpu.tick();
					self.increment_timers();
					if self.cpu.clock.audio_clock > AUDIO_TICK_RATE {
						audio::tick(&mut self.cpu, &mut self.device.audio);
						self.cpu.clock.audio_clock = 0;
					}
					self.check_dmas(dma::DMA_TIMING_IMMEDIATE);
				} else {
					self.cpu.reg_dump_pretty();
					panic!("Attempting to execute at unexecutable address 0x{:08x}!", self.cpu.get_exec_address());
				}
			}
		}

		measure_end(MEASURE_CPU_TICKS_TIME);
		measure_end(MEASURE_DMA_TICKS_TIME);
	}

	fn increment_timers(&mut self) {
		let timer_inc = self.cpu.clock.timer_cycles;
		self.cpu.clock.timer_cycles = 0;
		timers::increment(&mut self.cpu, timer_inc);
		// let overflow_int_mask = self.cpu.memory.internal_regs.increment_timers(timer_inc);
		// if overflow_int_mask != 0 {
		// 	self.hardware_interrupt(overflow_int_mask);
		// }
	}
}

pub trait GbaEventPoll {
	fn poll_device_events(&mut self);
}

