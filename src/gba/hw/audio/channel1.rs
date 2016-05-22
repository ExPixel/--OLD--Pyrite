use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel1;
use super::{AudioState, apply_volume_stereo, get_freq_len_duty};
use std;

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	let channel: &mut GbaChannel1 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel1 as *mut GbaChannel1) };

	if channel.initial {
		channel.sweep_time_acc = 0;
		channel.envelope_time_acc = 0;
		channel.current_volume = channel.initial_volume;
		channel.sound_length_time_acc = device.millis_to_frames(3, 9) * (64 - channel.sound_length as u32);
		state.c1_volume_multiplier = (channel.current_volume as f32) / 15.0;
		channel.initial = false;
	}

	state.c1_freq_len = device.sample_rate_f / max!(channel.frequency_f, 1.0);
	state.c1_freq_len_duty = get_freq_len_duty(state.c1_freq_len, channel.wave_pattern_duty);
	state.c1_volume_multiplier = (channel.current_volume as f32) / 15.0;

	channel.playing = !channel.length_flag || channel.sound_length_time_acc > 0;
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
	let channel: &mut GbaChannel1 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel1 as *mut GbaChannel1) };
	if !channel.length_flag || channel.sound_length_time_acc > 0 {
		// Sweeps:
		if channel.sweep_time > 0 {
			channel.sweep_time_acc += 1;
			let sweep_time_frames = device.millis_to_frames(7, 8) * (channel.sweep_time as u32);
			if channel.sweep_time_acc >= sweep_time_frames {
				let mut f = channel.frequency;

				if channel.sweep_frequency_dec {
					if (channel.frequency >> channel.sweep_shift_number) <= f {
						// ^ we stop this from becoming 0 or "lower"
						f -= channel.frequency >> channel.sweep_shift_number;
					}
				} else {
					f += channel.frequency >> channel.sweep_shift_number;
					if f > 2047 {
						f = 2047;
						let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
						cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !1); // turn the sound off.
					}
				}
				
				channel.frequency = f;
				channel.frequency_f = 131072.0 / (2048.0 - channel.frequency as f32);

				state.c1_freq_len = device.sample_rate_f / channel.frequency_f;
				state.c1_freq_len_duty = get_freq_len_duty(state.c1_freq_len, channel.wave_pattern_duty);
				channel.sweep_time_acc = 0;
			}
		}

		// Envelope Function:
		if channel.envelope_step_time > 0 {
			channel.envelope_time_acc += 1;
			let envelope_time_frames = device.millis_to_frames(15, 6) * (channel.envelope_step_time as u32);
			if channel.envelope_time_acc >= envelope_time_frames {
				if channel.envelope_inc && channel.current_volume < 15 {
					channel.current_volume += 1;
					state.c1_volume_multiplier = (channel.current_volume as f32) / 15.0;
				} else if (!channel.envelope_inc) && channel.current_volume > 0 {
					channel.current_volume -= 1;
					state.c1_volume_multiplier = (channel.current_volume as f32) / 15.0;
				}
				channel.envelope_time_acc = 0;
			}
		}

		channel.frequency_step += 1.0; // 0.74303854875, 1.34582519531
		if channel.frequency_step > state.c1_freq_len {
			channel.frequency_step = 0.0;
		}

		if channel.length_flag {
			channel.sound_length_time_acc -= 1;
			if channel.sound_length_time_acc == 0 {
				let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
				cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !1); // turn the sound off.
			}
		}

		return if channel.frequency_step < state.c1_freq_len_duty {
			// Does the multiplication on a u16 and then converts back to i16
			// so that we can get a value in the range of -32,767 to 32,767
			// subtracts 1 because the highest number that can come out of the other end is actually
			// 32768 which we don't want.
			apply_volume_stereo(std::i16::MAX, state.c1_volume_multiplier)
		} else {
			apply_volume_stereo(std::i16::MIN, state.c1_volume_multiplier)
		}
	}
	return apply_volume_stereo(std::i16::MIN, state.c1_volume_multiplier); // Produce no sound because the channel is off.
}