use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel4;
use super::{AudioState};
use std;

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	let channel: &mut GbaChannel4  = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel4  as *mut GbaChannel4 ) };
	
	if channel.initial {
		channel.envelope_time_acc = 0;
		channel.current_volume = channel.initial_volume;
		state.c4_volume_multiplier = (channel.current_volume as f32) / 15.0;

		let r = if channel.dividing_ratio == 0 { 0.5 } else { channel.dividing_ratio as f32 };
		channel.intermediate_freq = 524_288.0 / r;

		channel.sound_length_time_acc = device.millis_to_frames(3, 9) * (64 - channel.sound_length as u32);

		channel.freq_acc = 0.0;
		if channel.counter_width_7 {
			channel.lfsr = 0x40;
			channel.lfsr_mask = 0x7F;
			channel.lfsr_xor = 0x60;
		} else {
			channel.lfsr = 0x4000;
			channel.lfsr_mask = 0x7FFF;
			channel.lfsr_xor = 0x6000;
		}
		channel.initial = false;
	}

	let freq = channel.intermediate_freq / ((1 << (channel.shift_clock_freq + 1)) as f32);
	channel.freq_inc = freq / device.sample_rate_f;

	state.c4_volume_multiplier = (channel.current_volume as f32) / 15.0;

	cpu.memory.internal_regs.audio_channel4.playing = !channel.length_flag || channel.sound_length_time_acc > 0;
	if !channel.playing {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !8);
	} else {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x | 8);
	}
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> usize {
	let channel: &mut GbaChannel4  = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel4  as *mut GbaChannel4 ) };

	if !channel.length_flag || channel.sound_length_time_acc > 0 {
		channel.freq_acc += channel.freq_inc;
		let mut out = channel.lfsr & 1;
		while channel.freq_acc >= 1.0 {
			if (channel.lfsr & 1) != 0 {
				channel.lfsr = ((channel.lfsr >> 1) ^ channel.lfsr_xor) & channel.lfsr_mask;
			} else {
				channel.lfsr = (channel.lfsr >> 1) & channel.lfsr_mask;
			}
			channel.freq_acc -= 1.0;
		}

		// Envelope Function:
		if channel.envelope_step_time > 0 {
			channel.envelope_time_acc += 1;
			let envelope_time_frames = device.millis_to_frames(15, 6) * (channel.envelope_step_time as u32);
			if channel.envelope_time_acc >= envelope_time_frames {
				if channel.envelope_inc && channel.current_volume < 15 {
					channel.current_volume += 1;
					state.c4_volume_multiplier = (channel.current_volume as f32) / 15.0;
				} else if (!channel.envelope_inc) && channel.current_volume > 0 {
					channel.current_volume -= 1;
					state.c4_volume_multiplier = (channel.current_volume as f32) / 15.0;
				}
				channel.envelope_time_acc = 0;
			}
		}

		if channel.length_flag {
			channel.sound_length_time_acc -= 1;
			if channel.sound_length_time_acc == 0 {
				let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
				cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !8); // turn the sound off.
			}
		}

		if out != 0 {
			channel.current_volume as usize
		} else {
			channel.current_volume as usize + 16
		}
	} else {
		return channel.current_volume as usize + 16
	}
}