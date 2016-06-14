use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel2;
use super::{AudioState, apply_volume_stereo, get_freq_len_duty};
use std;

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	let channel: &mut GbaChannel2 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel2 as *mut GbaChannel2) };

	if channel.initial {
		channel.envelope_time_acc = 0;
		channel.current_volume = channel.initial_volume;
		channel.sound_length_time_acc = device.millis_to_frames(3, 9) * (64 - channel.sound_length as u32);
		state.c2_volume_multiplier = (channel.current_volume as f32) / 15.0;
		channel.initial = false;
	}

	state.c2_freq_len = device.sample_rate_f / max!(channel.frequency_f, 1.0);
	state.c2_freq_len_duty = get_freq_len_duty(state.c2_freq_len, channel.wave_pattern_duty);
	state.c2_volume_multiplier = (channel.current_volume as f32) / 15.0;

	channel.playing = !channel.length_flag || channel.sound_length_time_acc > 0;
	if !channel.playing {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !2);
	} else {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x | 2);
	}
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> usize {
	let channel: &mut GbaChannel2 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel2 as *mut GbaChannel2) };
	if !channel.length_flag || channel.sound_length_time_acc > 0 {
		// Envelope Function:
		if channel.envelope_step_time > 0 {
			channel.envelope_time_acc += 1;
			let envelope_time_frames = device.millis_to_frames(15, 6) * (channel.envelope_step_time as u32);
			if channel.envelope_time_acc >= envelope_time_frames {
				if channel.envelope_inc && channel.current_volume < 15 {
					channel.current_volume += 1;
					state.c2_volume_multiplier = (channel.current_volume as f32) / 15.0;
				} else if (!channel.envelope_inc) && channel.current_volume > 0 {
					channel.current_volume -= 1;
					state.c2_volume_multiplier = (channel.current_volume as f32) / 15.0;
				}
				channel.envelope_time_acc = 0;
			}
		}

		channel.frequency_step += 1.0; // 0.74303854875, 1.34582519531
		if channel.frequency_step > state.c2_freq_len {
			channel.frequency_step = 0.0;
		}

		if channel.length_flag {
			channel.sound_length_time_acc -= 1;
			if channel.sound_length_time_acc == 0 {
				let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
				cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !2); // turn the sound off.
			}
		}

		return if channel.frequency_step < state.c2_freq_len_duty {
			channel.current_volume as usize
		} else {
			channel.current_volume as usize + 16
		}
	}
	return channel.current_volume as usize + 16
}