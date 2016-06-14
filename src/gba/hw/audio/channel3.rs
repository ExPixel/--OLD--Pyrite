use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel3;
use super::apply_volume;
use std;

pub fn convert_sample(sample4: u16) -> i16 {
	// return (sample4 << 12) as i16;
	// return ((sample4 << 12) | 0xfff) as i16;
	const SAMPLES: [i16; 16] = [-32768, -28672, -24576, -20480, -16384, -12288, -8192, -4096, 0, 4096, 8192, 12288, 16384, 20480, 24576, 32767];
	return SAMPLES[(sample4 as usize) & 0xf];
}

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice) {
	let channel: &mut GbaChannel3 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel3 as *mut GbaChannel3) };

	let reset_sample_rate = channel.initial || (!channel.length_flag);

	if channel.initial {
		channel.sound_length_time_acc = device.millis_to_frames(3, 9) * (256 - channel.sound_length as u32);
		channel.current_wav_index = 0;
		channel.freq_acc = 0.0;
		channel.initial = false;
	}

	if reset_sample_rate {
		let sample_rate = 2097152.0 / (2048.0 - channel.sample_rate as f32);
		channel.freq_inc = sample_rate / device.sample_rate_f;
	}

	channel.playing = channel.channel_on && (!channel.length_flag || channel.sound_length_time_acc > 0);
	if !channel.playing {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !4); // turn the sound off.
	} else {
		let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
		cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x | 4);
	}

	if channel.force_volume {
		channel.c3_volume_multiplier = 0.75;
	} else {
		channel.c3_volume_multiplier =  match channel.sound_volume {
			0 => 0.0,
			1 => 1.0,
			2 => 0.5,
			3 => 0.25,
			_ => unreachable!()
		};
	}
}

pub fn tick(cpu: &mut ArmCpu, _: &AudioDevice) -> i16 {
	let channel: &mut GbaChannel3 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel3 as *mut GbaChannel3) };
	if channel.channel_on && (!channel.length_flag || channel.sound_length_time_acc > 0) {
		let wav_idx = channel.current_wav_index & 0x1f;
		let bank = if channel.wav_ram_banked {
			if (channel.current_wav_index & 0x3f) > 0x1f {
				(channel.wav_ram_bank ^ 1) as usize
			} else {
				channel.wav_ram_bank as usize
			}
		} else {
			channel.wav_ram_bank as usize
		};
		
		// playing order is 4-7, 0-3, 12-15, 8-11

		// byte select
		let b_sel = ((wav_idx >> 1) & 1) << 3;

		// nibble select
		let n_sel = (1 - ((wav_idx) & 1)) << 2;

		let b = (channel.wav_ram[bank][wav_idx >> 2]) >> b_sel;
		let sample = (b >> n_sel) & 0xf;

		channel.freq_acc += channel.freq_inc;
		while channel.freq_acc >= 1.0 {
			channel.current_wav_index += 1;
			channel.freq_acc -= 1.0;
		}

		if channel.length_flag {
			channel.sound_length_time_acc -= 1;
			if channel.sound_length_time_acc == 0 {
				let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
				cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !4); // turn the sound off.
			}
		}

		return apply_volume(convert_sample(sample), channel.c3_volume_multiplier);
	}
	return apply_volume(std::i16::MIN, channel.c3_volume_multiplier);
}