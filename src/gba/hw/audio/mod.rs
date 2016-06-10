// #TODO this entire module can use some serious optimization.
//       moving expensive divisions everywhere to LUT and the like.
//       First thing I can do is move channel specific volumes from the
//       expensive apply_volume_stereo to a lookup table.

mod channel1;
mod channel2;
mod channel3;
mod channel4;
pub mod channel_ab;

use super::super::core::cpu::ArmCpu;
use super::super::core::memory::*;
use super::super::device::audio::AudioDevice;
use ::util::measure::*;
use ::debug::debugger;

// There is a lot of stuff I don't want to be calculating
// over and over because a lot of division is involved,
// so I cache the results in this struct and just pass this
// around instead.
// #TODO these can be moved to their respective channels.
#[derive(Default)]
pub struct AudioState {
	// Channel 1:
	c1_freq_len: f32,
	c1_freq_len_duty: f32,
	c1_volume_multiplier: f32,

	// Channel 2:
	c2_freq_len: f32,
	c2_freq_len_duty: f32,
	c2_volume_multiplier: f32,

	// Channel 3:
	c3_volume_multiplier: f32,

	//Channel 4:
	c4_volume_multiplier: f32
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice) {
	measure_start(MEASURE_AUDIO_TICK_TIME);
	measure_iteration(MEASURE_AUDIO_TICK_TIME);
	let debugger = debugger::get_debugger();

	device.ring_buffer.try_write(|frames| {
		let soundcnt_l = cpu.memory.get_reg(ioreg::SOUNDCNT_L);
		let soundcnt_h = cpu.memory.get_reg(ioreg::SOUNDCNT_H);
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		let sound_1_4_vol = 2 - min!(2, soundcnt_h & 0x3); // this is for shifting the output right.
		let sound_1_4_left_vol = ((soundcnt_l & 0x7) as f32) / 7.0;
		let sound_1_4_right_vol = (((soundcnt_l >> 4) & 0x7) as f32) / 7.0;
		let sound_1_4_enable_left = soundcnt_l >> 8;
		let sound_1_4_enable_right = soundcnt_l >> 12;

		let mut state: AudioState = Default::default();

		channel1::init(cpu, device, &mut state);
		channel2::init(cpu, device, &mut state);
		channel3::init(cpu, device, &mut state);
		channel4::init(cpu, device, &mut state);
		channel_ab::init(cpu, device, &mut state);

		for idx in 0..frames.len() {
			let mut sig_left = 0;
			let mut sig_right = 0;

			// Sound 1:
			if cpu.memory.internal_regs.audio_channel1.playing {
				let (mut left, mut right) = channel1::tick(cpu, device, &mut state);
				debugger.sound_channel_1_plot.plot(left as f32);

				if (sound_1_4_enable_left & 1) != 0 { // Sound 1 Left Enable
					left >>= sound_1_4_vol as i16;
					sig_left += apply_volume(left, sound_1_4_left_vol) >> 2;
				}

				if (sound_1_4_enable_right & 1) != 0 { // Sound 1 Right Enable
					right >>= sound_1_4_vol as i16;
					sig_right += apply_volume(right, sound_1_4_right_vol) >> 2;
				}
			} else {
				debugger.sound_channel_1_plot.plot(-32768.0);
			}

			// Sound 2:
			if cpu.memory.internal_regs.audio_channel2.playing {
				let (mut left, mut right) = channel2::tick(cpu, device, &mut state);
				debugger.sound_channel_2_plot.plot(left as f32);

				if (sound_1_4_enable_left & 2) != 0 { // Sound 2 Left Enable
					left >>= sound_1_4_vol as i16;
					sig_left += apply_volume(left, sound_1_4_left_vol) >> 2;
				}

				if (sound_1_4_enable_right & 2) != 0 { // Sound 2 Right Enable
					right >>= sound_1_4_vol as i16;
					sig_right += apply_volume(right, sound_1_4_right_vol) >> 2;
				}
			} else {
				debugger.sound_channel_2_plot.plot(-32768.0);
			}

			// Sound 3:
			if cpu.memory.internal_regs.audio_channel3.playing {
				let (mut left, mut right) = channel3::tick(cpu, device, &mut state);
				debugger.sound_channel_3_plot.plot(left as f32);

				if(sound_1_4_enable_left & 4) != 0 {
					left >>= sound_1_4_vol as i16;
					sig_left += apply_volume(left, sound_1_4_left_vol) >> 2;
				}

				if (sound_1_4_enable_right & 4) != 0 { // Sound 2 Right Enable
					right >>= sound_1_4_vol as i16;
					sig_right += apply_volume(right, sound_1_4_right_vol) >> 2;
				}
			} else {
				debugger.sound_channel_3_plot.plot(-32768.0);
			}

			// Sound 4:
			if cpu.memory.internal_regs.audio_channel4.playing {
				let (mut left, mut right) = channel4::tick(cpu, device, &mut state);
				debugger.sound_channel_4_plot.plot(left as f32);

				if(sound_1_4_enable_left & 8) != 0 {
					left >>= sound_1_4_vol as i16;
					sig_left += apply_volume(left, sound_1_4_left_vol) >> 2;
				}

				if (sound_1_4_enable_right & 8) != 0 { // Sound 2 Right Enable
					right >>= sound_1_4_vol as i16;
					sig_right += apply_volume(right, sound_1_4_right_vol) >> 2;
				}
			} else {
				debugger.sound_channel_4_plot.plot(-32768.0);
			}

			// DMA Sound:
			if (soundcnt_x & 0x80) != 0 {
				// DMA Sound A:
				{
					let (left, right) = channel_ab::tick_a(cpu, device, &mut state);
					debugger.sound_channel_a_plot.plot(left as f32);
					
					if cpu.memory.internal_regs.audio_fifo_a.enable_left {
						sig_left += left;
					}

					if cpu.memory.internal_regs.audio_fifo_a.enable_right {
						sig_right += right;
					}
				}

				// // DMA Sound B:
				// {
				// 	let (left, right) = channel_ab::tick_b(cpu, device, &mut state);
				//  debugger.sound_channel_b_plot.plot(left as f32);
					
				// 	if cpu.memory.internal_regs.audio_fifo_b.enable_left {
				// 		sig_left += left >> 2;
				// 	}

				// 	if cpu.memory.internal_regs.audio_fifo_b.enable_right {
				// 		sig_right += right >> 2;
				// 	}
				// }
			}

			frames[idx] = (sig_left, sig_right);
			debugger.sound_plot.plot(sig_left as f32);
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