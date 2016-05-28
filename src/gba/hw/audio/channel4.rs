use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel4;
use super::{AudioState, apply_volume_stereo};
use std;

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	let channel: &mut GbaChannel4  = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel4  as *mut GbaChannel4 ) };
	
	if channel.initial {
		channel.freq_acc = 0.0;
		if channel.counter_width_7 {
			channel.lfsr = 0x40;
			channel.lfsr_mask = 0x7F;
			channel.lfsr_xor = 0x60;
			println!("7");
		} else {
			channel.lfsr = 0x4000;
			channel.lfsr_mask = 0x7FFF;
			channel.lfsr_xor = 0x6000;
			println!("15");
		}
		channel.initial = false;

		let r = if channel.dividing_ratio == 0 { 0.5 } else { channel.dividing_ratio as f32 };
		channel.intermediate_freq = 524_288.0 / r;

		println!("Initial!");
	}

	let freq = channel.intermediate_freq / ((1 << (channel.shift_clock_freq + 1)) as f32);
	channel.freq_inc = freq / device.sample_rate_f;

	// pyrite_debugging!({
	// 	println!("r: {} ({})", channel.dividing_ratio, if channel.dividing_ratio == 0 { 0.5 } else { channel.dividing_ratio as f32 });
	// 	println!("s: {} ({})", channel.shift_clock_freq, (1 << (channel.shift_clock_freq + 1)) as f32);
	// 	println!("i: {}", channel.intermediate_freq);
	// 	println!("freq: {} ({:+})", freq, channel.freq_inc);
	// });

	cpu.memory.internal_regs.audio_channel4.playing = !channel.length_flag || channel.sound_length_time_acc > 0;
}

pub fn tick(cpu: &mut ArmCpu, _: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
	let channel: &mut GbaChannel4  = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel4  as *mut GbaChannel4 ) };

	channel.freq_acc += channel.freq_inc;
	let mut out = channel.lfsr & 1;
	while channel.freq_acc >= 1.0 {
		if out != 0 {
			channel.lfsr = ((channel.lfsr >> 1) ^ channel.lfsr_xor) & channel.lfsr_mask;
		} else {
			channel.lfsr = (channel.lfsr >> 1) & channel.lfsr_mask;
		}
		channel.freq_acc -= 1.0;
		out = channel.lfsr & 1;
	}

	if out != 0 {
		(std::i16::MAX, std::i16::MAX)
	} else {
		(std::i16::MIN, std::i16::MIN)
	}
}