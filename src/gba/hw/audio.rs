use super::super::core::cpu::ArmCpu;
use super::super::core::memory::*;
use super::super::core::memory::ioreg::GbaChannel1;
use super::super::core::memory::ioreg::GbaChannel2;
use super::super::device::audio::AudioDevice;
use ::util::measure::*;
use std;

// There is a lot of stuff I don't want to be calculating
// over and over because a lot of division is involved,
// so I cache the results in this struct and just pass this
// around instead.
#[derive(Default)]
struct AudioState {
	// Channel 1:
	c1_freq_len: f32,
	c1_freq_len_duty: f32,
	c1_volume_multiplier: f32,

	// Channel 2:
	c2_freq_len: f32,
	c2_freq_len_duty: f32,
	c2_volume_multiplier: f32,
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice) {
	measure_start(MEASURE_AUDIO_TICK_TIME);

	device.ring_buffer.try_write(|frames| {
		let soundcnt_l = cpu.memory.get_reg(ioreg::SOUNDCNT_L);
		let soundcnt_h = cpu.memory.get_reg(ioreg::SOUNDCNT_H);
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		let sound_1_4_vol = 2 - min!(2, soundcnt_h & 0x3); // this is for shifting the output right.
		let sound_1_4_left_vol = ((soundcnt_l & 0x7) as f32) / 7.0;
		let sound_1_4_right_vol = (((soundcnt_l >> 4) & 0x7) as f32) / 7.0;
		let sound_1_4_enable_left = soundcnt_l >> 8;
		let sound_1_4_enable_right = soundcnt_l >> 12;

		pyrite_debugging!({
			debug_print_vars_ln!(
				soundcnt_l,
				soundcnt_h,
				soundcnt_x,
				sound_1_4_vol,
				sound_1_4_left_vol,
				sound_1_4_right_vol,
				sound_1_4_enable_left,
				sound_1_4_enable_right
			);
		});

		let mut state: AudioState = Default::default();

		init_channel1(cpu, device, &mut state);
		// init_channel2(cpu, device, &mut state);

		for idx in 0..frames.len() {
			let mut sig_left = 0;
			let mut sig_right = 0;

			// Sound 1:
			if cpu.memory.internal_regs.audio_channel1.playing {
				let (mut left, mut right) = tick_channel1(cpu, device, &mut state);

				if (sound_1_4_enable_left & 1) != 0 { // Sound 1 Left Enable
					left >>= sound_1_4_vol as i16;
					sig_left += apply_volume(left, sound_1_4_left_vol) >> 2;
				}

				if (sound_1_4_enable_right & 1) != 0 { // Sound 1 Right Enable
					right >>= sound_1_4_vol as i16;
					sig_right += apply_volume(right, sound_1_4_right_vol) >> 2;
				}
			}

			// // Sound 2:
			// if cpu.memory.internal_regs.audio_channel2.playing {
			// 	let (mut left, mut right) = tick_channel2(cpu, device, &mut state);

			// 	if (sound_1_4_enable_left & 2) != 0 { // Sound 2 Left Enable
			// 		left >>= sound_1_4_vol as i16;
			// 		sig_left += apply_volume(left, sound_1_4_left_vol) >> 2;
			// 	}

			// 	if (sound_1_4_enable_right & 2) != 0 { // Sound 2 Right Enable
			// 		right >>= sound_1_4_vol as i16;
			// 		sig_right += apply_volume(right, sound_1_4_right_vol) >> 2;
			// 	}
			// }

			frames[idx] = (sig_left, sig_right);
		}

		return true
	});

	measure_end(MEASURE_AUDIO_TICK_TIME);
}

fn apply_volume(sample: i16, volume: f32) -> i16 {
	((sample as f32) * volume) as i16
}

fn apply_volume_stereo(sample: i16, volume: f32) -> (i16, i16) {
	let r = ((sample as f32) * volume) as i16;
	return (r, r)
}

fn get_freq_len_duty(flen: f32, duty: u16) -> f32 {
	match duty {
		0 => flen / 8.0,
		1 => flen / 4.0,
		2 => flen / 2.0,
		3 => flen / (4.0 / 3.0),
		_ => flen / 2.0
	}
}

fn init_channel1(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	let channel: &mut GbaChannel1 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel1 as *mut GbaChannel1) };

	if channel.initial {
		channel.sweep_time_acc = 0;
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

fn tick_channel1(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
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