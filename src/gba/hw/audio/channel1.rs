use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel1;
use super::get_freq_len_duty;
use std;

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice) {
	let channel: &mut GbaChannel1 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel1 as *mut GbaChannel1) };

	if channel.initial {
		channel.sweep_time_acc = 0;
		channel.envelope_time_acc = 0;
		channel.current_volume = channel.initial_volume;
		channel.sound_length_time_acc = device.millis_to_frames(3, 9) * (64 - channel.sound_length as u32);
		channel.initial = false;
	}

	channel.freq_len = device.sample_rate_f / max!(channel.frequency_f, 1.0);
	channel.freq_len_duty = get_freq_len_duty(channel.freq_len, channel.wave_pattern_duty);

	channel.playing = !channel.length_flag || channel.sound_length_time_acc > 0;
	if !channel.playing {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !1);
	} else {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x | 1);
	}
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice) -> usize {
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

				channel.freq_len = device.sample_rate_f / channel.frequency_f;
				channel.freq_len_duty = get_freq_len_duty(channel.freq_len, channel.wave_pattern_duty);
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
				} else if (!channel.envelope_inc) && channel.current_volume > 0 {
					channel.current_volume -= 1;
				}
				channel.envelope_time_acc = 0;
			}
		}

		channel.frequency_step += 1.0; // 0.74303854875, 1.34582519531
		if channel.frequency_step > channel.freq_len {
			channel.frequency_step = 0.0;
		}

		if channel.length_flag {
			channel.sound_length_time_acc -= 1;
			if channel.sound_length_time_acc == 0 {
				let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
				cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !1); // turn the sound off.
			}
		}

		return if channel.frequency_step < channel.freq_len_duty {
			channel.current_volume as usize
		} else {
			channel.current_volume as usize + 16
		}
	}
	return channel.current_volume as usize + 16
}