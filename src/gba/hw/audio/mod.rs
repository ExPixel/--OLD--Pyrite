mod channel1;
mod channel2;
mod channel3;
mod channel4;
pub mod channel_ab;

use super::super::core::cpu::ArmCpu;
use super::super::core::memory::*;
use super::super::device::audio::AudioDevice;
use ::util::measure::*;


const AMPLITUDE_OUTPUTS: [i16; 32] = [
// HIGH:
0, 2184, 4369, 6553, 8738, 10922, 13107, 15291,
17476, 19660, 21845, 24029, 26214, 28398, 30583, 32767,

// LOW:
0, -2185, -4369, -6554, -8738, -10923, -13107, -15292,
-17476, -19661, -21845, -24030, -26214, -28399, -30583, -32768
];

const PSG_VOLUME_MULS: [f32; 8] = [
	0.0, 0.14285714285714285,  0.2857142857142857, 0.42857142857142855,
	0.5714285714285714, 0.7142857142857143, 0.8571428571428571, 1.0
];

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice) {
	measure_start(MEASURE_AUDIO_TICK_TIME);
	measure_iteration(MEASURE_AUDIO_TICK_TIME);

	// console_warn!("{} OUT REMINAING", cpu.memory.internal_regs.audio_fifo_a.out_remaining());

	device.ring_buffer.try_write(|frames| {
		let soundcnt_l = cpu.memory.get_reg(ioreg::SOUNDCNT_L);
		let soundcnt_h = cpu.memory.get_reg(ioreg::SOUNDCNT_H);
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);

		let mut mixer = GbaAudioMixer::new(soundcnt_l, soundcnt_h, soundcnt_x);
		mixer.init();

		channel1::init(cpu, device);
		channel2::init(cpu, device);
		channel3::init(cpu, device);
		channel4::init(cpu, device);
		channel_ab::init(cpu, device);

		for idx in 0..frames.len() {
			mixer.psg_count = 0;

			// Sound 1:
			if cpu.memory.internal_regs.audio_channel1.playing {
				mixer.c1 = AMPLITUDE_OUTPUTS[channel1::tick(cpu, device)];
				mixer.psg_count += 1;
			} else {
				mixer.c1 = 0;
			}

			// Sound 2:
			if cpu.memory.internal_regs.audio_channel2.playing {
				mixer.c2 = AMPLITUDE_OUTPUTS[channel2::tick(cpu, device)];
				mixer.psg_count += 1;
			} else {
				mixer.c2 = 0;
			}

			// Sound 3:
			if cpu.memory.internal_regs.audio_channel3.playing {
				mixer.c3 = channel3::tick(cpu, device);
				mixer.psg_count += 1;
			} else {
				mixer.c3 = 0;
			}

			// Sound 4:
			if cpu.memory.internal_regs.audio_channel4.playing {
				mixer.c4 = AMPLITUDE_OUTPUTS[channel4::tick(cpu, device)];
				mixer.psg_count += 1;
			} else {
				mixer.c4 = 0;
			}

			// DMA Sound:
			if (soundcnt_x & 0x80) != 0 {
				mixer.ca = channel_ab::tick_a(cpu);
				mixer.cb = channel_ab::tick_b(cpu);
			} else {
				mixer.ca = 0;
				mixer.cb = 0;
			}

			if psetting!(sound_enabled) {
				frames[idx] = mixer.mix();
			} else {
				frames[idx] = (0, 0);
			}
		}

		return true
	});

	measure_end(MEASURE_AUDIO_TICK_TIME);
}

#[derive(Default)]
pub struct GbaAudioMixer {
	c1: i16, c2: i16, c3: i16, c4: i16,
	ca: i16, cb: i16,

	psg_count: u32,

	psg_right_vol: f32,
	psg_left_vol: f32,

	soundcnt_l: u16,
	soundcnt_h: u16,
	soundcnt_x: u16,
}

impl GbaAudioMixer {
	pub fn new(soundcnt_l: u16, soundcnt_h: u16, soundcnt_x: u16) -> GbaAudioMixer {
		GbaAudioMixer {
			soundcnt_l: soundcnt_l,
			soundcnt_h: soundcnt_h,
			soundcnt_x: soundcnt_x,
			..Default::default()
		}
	}

	pub fn init(&mut self) {
		// 0-2   Sound 1-4 Master Volume RIGHT (0-7)
		self.psg_right_vol = PSG_VOLUME_MULS[(self.soundcnt_l & 0x7) as usize];

		// 4-6   Sound 1-4 Master Volume LEFT (0-7)
		self.psg_left_vol = PSG_VOLUME_MULS[((self.soundcnt_l >> 4) & 0x7) as usize];
	}

	pub fn mix(&self) -> (i16, i16) {
		let mut psg_right = 0;
		let mut psg_left = 0;
		let mut dma_right = 0;
		let mut dma_left = 0;

		let psg_balance_div = self.psg_count as f32;
		let psg_right_vol_b = self.psg_right_vol / psg_balance_div;
		let psg_left_vol_b = self.psg_left_vol / psg_balance_div;

		if psetting!(channel1_enabled) {
			if (self.soundcnt_l & 0x100) != 0 { psg_right += apply_volume(self.c1, psg_right_vol_b) }
			if (self.soundcnt_l & 0x1000) != 0 { psg_left += apply_volume(self.c1, psg_left_vol_b) }
		}

		if psetting!(channel2_enabled) {
			if (self.soundcnt_l & 0x200) != 0 { psg_right += apply_volume(self.c2, psg_right_vol_b) }
			if (self.soundcnt_l & 0x2000) != 0 { psg_left += apply_volume(self.c2, psg_left_vol_b) }
		}

		if psetting!(channel3_enabled) {
			if (self.soundcnt_l & 0x400) != 0 { psg_right += apply_volume(self.c3, psg_right_vol_b) }
			if (self.soundcnt_l & 0x4000) != 0 { psg_left += apply_volume(self.c3, psg_left_vol_b) }
		}

		if psetting!(channel4_enabled) {
			if (self.soundcnt_l & 0x800) != 0 { psg_right += apply_volume(self.c4, psg_right_vol_b) }
			if (self.soundcnt_l & 0x8000) != 0 { psg_left += apply_volume(self.c4, psg_left_vol_b) }
		}


		// 0-1   Sound # 1-4 Volume   (0=25%, 1=50%, 2=100%, 3=Prohibited)
		let psg_volume_shift = 2 - min!(2, self.soundcnt_h & 0x3);

		psg_left >>= psg_volume_shift;
		psg_right >>= psg_volume_shift;

		if (self.soundcnt_x & 0x80) != 0 {
			if psetting!(channela_enabled) {
				let dma_a_volume_shift = 1 - ((self.soundcnt_h >> 2) & 1);
				let dma_a = self.ca >> dma_a_volume_shift;
				if (self.soundcnt_h & 0x100) != 0 { dma_right += dma_a; }
				if (self.soundcnt_h & 0x200) != 0 { dma_left += dma_a; }
			}

			if psetting!(channelb_enabled) {
				let dma_b_v_volume_shift = 1 - ((self.soundcnt_h >> 3) & 1);
				let dma_b = self.cb >> dma_b_v_volume_shift;
				if (self.soundcnt_h & 0x1000) != 0 { dma_right += dma_b; }
				if (self.soundcnt_h & 0x2000) != 0 { dma_left += dma_b; }
			}
		}

		let left = psg_left + dma_left;
		let right = psg_right + dma_right;

		// DEBUGGER:
		{
			let debugger = ::debug::debugger::get_debugger();
			debugger.sound_channel_1_plot.plot(self.c1 as f32);
			debugger.sound_channel_2_plot.plot(self.c2 as f32);
			debugger.sound_channel_3_plot.plot(self.c3 as f32);
			debugger.sound_channel_4_plot.plot(self.c4 as f32);
			debugger.sound_channel_a_plot.plot(self.ca as f32);
			debugger.sound_channel_b_plot.plot(self.cb as f32);
			debugger.sound_plot.plot(
				if left == 0 { right } else { left } as f32
			);
		}

		(left, right)
	}
}

fn apply_volume(sample: i16, volume: f32) -> i16 {
	((sample as f32) * volume) as i16
}

// fn apply_volume_stereo(sample: i16, volume: f32) -> (i16, i16) {
// 	let r = ((sample as f32) * volume) as i16;
// 	return (r, r)
// }

fn get_freq_len_duty(flen: f32, duty: u16) -> f32 {
	match duty {
		0 => flen / 8.0,
		1 => flen / 4.0,
		2 => flen / 2.0,
		3 => flen / (4.0 / 3.0),
		_ => flen / 2.0
	}
}