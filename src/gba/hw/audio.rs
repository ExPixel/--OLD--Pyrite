use super::super::core::cpu::ArmCpu;
use super::super::core::memory::*;
use super::super::core::memory::ioreg::GbaChannel1;
use super::super::device::audio::AudioDevice;
use ::util::measure::*;
use time;

impl GbaAudio for ArmCpu {
	fn audio_tick_channel1(&mut self, device: &mut AudioDevice) {
		let channel = self.get_audio_channel1();

		let mut freq_len = device.sample_rate_f / max!(channel.frequency_f, 1.0);

		device.ring_buffer.try_write(|frames| {
			for idx in 0..frames.len() {
				// Sweeps:
				if channel.sweep_time > 0 {
					channel.sweep_time_acc += 1;
					let sweep_time_frames = device.millis_to_frames(7, 8) * (channel.sweep_time as u32);
					if channel.sweep_time_acc >= sweep_time_frames {

						let mut f = channel.frequency;

						if channel.sweep_frequency_dec {
							if (channel.frequency >> channel.sweep_shift_number) < f {
								// ^ we stop this from becoming 0 or "lower"
								f -= channel.frequency >> channel.sweep_shift_number;
							}
						} else {
							f += channel.frequency >> channel.sweep_shift_number;
						}

						channel.frequency = min!(2047, f);
						channel.frequency_f = 131072.0 / (2048.0 - channel.frequency as f32);

						freq_len = device.sample_rate_f / channel.frequency_f;

						channel.sweep_time_acc = 0;
					}
				}

				// Envelope Function: #TODO

				use std;
				channel.frequency_step += 1.0;
				if channel.frequency_step > freq_len {
					channel.frequency_step = 0.0;
				}

				if channel.frequency_step < (freq_len / 2.0) {
					frames[idx] = (std::i16::MAX, std::i16::MAX);
				} else {
					frames[idx] = (std::i16::MIN, std::i16::MIN);
				}
			}

			return true
		});
	}
}

pub trait GbaAudio {
	fn audio_tick(&mut self, device: &mut AudioDevice) {
		measure_start(MEASURE_AUDIO_TICK_TIME);
		self.audio_tick_channel1(device);
		measure_end(MEASURE_AUDIO_TICK_TIME);
	}

	fn audio_tick_channel1(&mut self, device: &mut AudioDevice);
}

trait GbaChannelContainer {
	fn get_audio_channel1(&mut self) -> &mut GbaChannel1;
}

impl GbaChannelContainer for ArmCpu {
	fn get_audio_channel1(&mut self) -> &mut GbaChannel1 {
		use std::mem;
		let ptr: *mut GbaChannel1 = &mut self.memory.internal_regs.audio_channel1 as *mut GbaChannel1;
		return unsafe { mem::transmute(ptr) };
	}
}