use super::super::core::cpu::ArmCpu;
use super::super::core::memory::*;
use super::super::device::audio::AudioDevice;
use ::util::measure::*;

// One cycle equals approx. 59.59ns (Closer to 45ns with the emulator running on my MacBook Pro) 
// But I imagine that this eventually evens out with the vsyncs and what not.

const DUTY_CYCLES: [f32; 4] = [
	0.125,
	0.25,
	0.5,
	0.75
];

#[derive(Default)]
struct GbaChannel1 {
	sweep_cycles_acc: u32, // cycles since the last sweep shift

	// 4000060h - SOUND1CNT_L (NR10) - Channel 1 Sweep register (R/W)
	sweep_shift_number: u16,   // 0-2   R/W  Number of sweep shift      (n=0-7)
	sweep_frequency_dec: bool, // 3     R/W  Sweep Frequency Direction  (0=Increase, 1=Decrease)
	sweep_time: u16,           // 4-6   R/W  Sweep Time; units of 7.8ms (0-7, min=7.8ms, max=54.7ms)
	// Sweep is disabled by setting Sweep Time to zero, if so, the direction bit should be set.
	// The change of frequency (NR13,NR14) at each shift is calculated by the following formula 
	// where X(0) is initial freq & X(t-1) is last freq:
	//  X(t) = X(t-1) +/- X(t-1)/2^n
	// ---- NOTE ----
	// The documentation is lying, it's not really the frequency but the rate which is 
	// that denominator under 131072 in register SOUND1CNT_X.
	// n is also refers to the sweep shift number above.

	// 4000062h - SOUND1CNT_H (NR11, NR12) - Channel 1 Duty/Len/Envelope (R/W)
	sound_length: u16,        // 0-5   W    Sound length; units of (64-n)/256s  (0-63)
	wave_pattern_duty: u16,   // 6-7   R/W  Wave Pattern Duty                   (0-3, see below)
	envelope_step_time: u16,  // 8-10  R/W  Envelope Step-Time; units of n/64s  (1-7, 0=No Envelope)
	envelope_inc: bool,       // 11    R/W  Envelope Direction                  (0=Decrease, 1=Increase)
	initial_volume: u16,      // 12-15 R/W  Initial Volume of envelope          (1-15, 0=No Sound)
	// Wave Duty:
	//   0: 12.5% ( -_______-_______-_______ )
	//   1: 25%   ( --______--______--______ )
	//   2: 50%   ( ----____----____----____ ) (normal)
	//   3: 75%   ( ------__------__------__ )
	// The Length value is used only if Bit 6 in NR14 is set.

	/// This is the volume that's actually chaning through the
	/// envelope function.
	current_volume: u16,
	envelope_cycles_acc: u32,

	/// Remaining cycles in sound length.
	sound_length_cycles_rem: u32,

	// 4000064h - SOUND1CNT_X (NR13, NR14) - Channel 1 Frequency/Control (R/W)
	frequency: u16,            // 0-10  W    Frequency; 131072/(2048-n)Hz  (0-2047)
	length_flag: bool,         // 14    R/W  Length Flag  (1=Stop output when length in NR11 expires)
	initial: bool,             // 15    W    Initial      (1=Restart Sound)

	dirty: bool
}

#[derive(Default)]
pub struct GbaAudio {
	c1: GbaChannel1,

	// The last number of cycles on the CPU's clock since the last call to tick.
	last_cpu_cycles: u64,
}

impl GbaAudio {
	pub fn tick(&mut self, device: &mut AudioDevice, cpu: &mut ArmCpu) {
		measure_start(MEASURE_AUDIO_TICK_TIME);
		self.load_channel1(cpu);

		// If we go 4 billion cycles without ever calling tick
		// then we have a bigger problem.
		let delta = (cpu.clock.cycles - self.last_cpu_cycles) as u32;

		self.tick_channel1(device, cpu, delta);

		// self.tick_channel1(device, cpu, delta);
		self.last_cpu_cycles = cpu.clock.cycles;
		measure_end(MEASURE_AUDIO_TICK_TIME);
	}

	fn load_channel1(&mut self, cpu: &mut ArmCpu) {
		if cpu.memory.internal_regs.sound_channel1_dirty {
			let sound1cnt_l = cpu.memory.get_reg(ioreg::SOUND1CNT_L);
			let sound1cnt_h = cpu.memory.get_reg(ioreg::SOUND1CNT_H);
			let sound1cnt_x = cpu.memory.get_reg(ioreg::SOUND1CNT_X);

			self.c1.sweep_shift_number = sound1cnt_l & 0x7;
			self.c1.sweep_frequency_dec = (sound1cnt_l & 0x8) != 0;
			self.c1.sweep_time = (sound1cnt_l >> 4) & 0x3;

			self.c1.sound_length = sound1cnt_h & 0x1f;
			self.c1.wave_pattern_duty = (sound1cnt_h >> 6) & 0x3;
			self.c1.envelope_step_time = (sound1cnt_h >> 8) & 0x7;
			self.c1.envelope_inc = (sound1cnt_h & 0x800) != 0;
			self.c1.initial_volume = (sound1cnt_h >> 12) & 0xf;

			self.c1.frequency = sound1cnt_x & 0x7ff;
			self.c1.length_flag = (sound1cnt_x & 0x4000) != 0;
			self.c1.initial = (sound1cnt_x & 0x8000) != 0;

			self.c1.dirty = true;
			cpu.memory.internal_regs.sound_channel1_dirty = false;
		}
	}

	fn tick_channel1(&mut self, device: &mut AudioDevice, cpu: &mut ArmCpu, delta: u32) {
		// So that the sound isn't restarted every tick.
		if self.c1.initial {
			self.c1.current_volume = self.c1.initial_volume;

			if self.c1.length_flag {
				// 1/256 seconds = 0.00390625 seconds
				// 0.00390625 seconds = 3906250 nanoseconds
				// 3906250 nanoseconds = 65552.1060581 cycles
				self.c1.sound_length_cycles_rem = 65552 * (64 - self.c1.sound_length as u32);
			}

			self.c1.initial = false;
			let mut sound1cnt_x = cpu.memory.get_reg(ioreg::SOUND1CNT_X);
			sound1cnt_x &= !0x8000; // clear the initial bit.
			cpu.memory.set_reg(ioreg::SOUND1CNT_X, sound1cnt_x);
		}

		if !self.c1.length_flag || self.c1.sound_length_cycles_rem > 0 {
			// Handling the sweep function:
			if self.c1.sweep_time > 0 {
				self.c1.sweep_cycles_acc += delta;
				let channel_sweep_cycle_delay = 128_000 * (self.c1.sweep_time as u32);
				if self.c1.sweep_cycles_acc >= channel_sweep_cycle_delay {
					let mut freq = self.c1.frequency as i32;
					if self.c1.sweep_frequency_dec {
						freq -= ((self.c1.frequency as u32) >> (self.c1.sweep_shift_number as u32)) as i32;
					} else {
						freq += ((self.c1.frequency as u32) >> (self.c1.sweep_shift_number as u32)) as i32;
					}

					// let sound1cnt_l = cpu.memory.get_reg(ioreg::SOUND1CNT_L);
					// self.c1.sweep_count += 1;

					// println!("n : {}", self.c1.sweep_shift_number);

					if freq > 0 {
						freq = min!(2047, freq);
						self.c1.frequency = freq as u16;
						self.c1.dirty = true;
					}

					self.c1.sweep_cycles_acc -= channel_sweep_cycle_delay;
				}
			}

			// Handling the envelope function:
			if self.c1.envelope_step_time > 0 {
				// 1/64 seconds = 0.015625
				// 0.015625 seconds = 15625000 nanoseconds
				// 15625000 nanoseconds = 262208.424232 cycles
				let channel_envelope_delay = 262208 * (self.c1.envelope_step_time as u32);
				self.c1.envelope_cycles_acc += delta;
				if self.c1.envelope_cycles_acc >= channel_envelope_delay {
					if self.c1.envelope_inc && self.c1.current_volume < 15 {
						self.c1.current_volume += 1;
					} else if !self.c1.envelope_inc && self.c1.current_volume > 0 {
						self.c1.current_volume -= 1;
					}
					self.c1.envelope_cycles_acc -= channel_envelope_delay;
				}
			} else {
				// No envelope so we play at full volume.
				self.c1.current_volume = 15;
			}

			// Handling sound length:
			if !self.c1.length_flag || self.c1.sound_length_cycles_rem < delta {
				self.c1.sound_length_cycles_rem = 0;
			} else {
				self.c1.sound_length_cycles_rem -= delta;
				// println!("length remaining: {}", self.c1.sound_length_cycles_rem);
			}
		}

		if self.c1.dirty {
			// let _f = device.channels.channel1.frequency;
			device.channels.channel1.frequency = 131_072.0 / ((2048 - self.c1.frequency) as f32);
			// if _f != device.channels.channel1.frequency {
			// 	println!("Playing at frequency: {} ({} = 0x{:04X})", device.channels.channel1.frequency, self.c1.frequency, self.c1.frequency);
			// }
			device.channels.channel1.duty_cycle = DUTY_CYCLES[self.c1.wave_pattern_duty as usize];
			device.channels.channel1.amplitude = (self.c1.current_volume as f32) / 15.0;

			// sound length is either off or it has more to go.
			let sound_length_continue = !self.c1.length_flag || self.c1.sound_length_cycles_rem > 0;

			// Shut the channel off if it goes any high to save my poor ears D:
			let sound_out_of_range = device.channels.channel1.frequency <= 20_000.0;

			device.channels.channel1.on = sound_length_continue & sound_out_of_range;
			device.commit_channel1();
		}
	}
}