use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel3;
use super::{AudioState, apply_volume_stereo};
use std;

const SAMPLES: [i16; 16] = [-32768, -28672, -24576, -20480, -16384, -12288, -8192, -4096, 0, 4096, 8192, 12288, 16384, 20480, 24576, 32767];

pub fn convert_sample(sample4: u16) -> i16 {
	// return (sample4 << 12) as i16;
	return SAMPLES[(sample4 as usize) & 0xf];
	// return ((sample4 << 12) | 0xfff) as i16;
}

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	let channel: &mut GbaChannel3 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel3 as *mut GbaChannel3) };
	if channel.initial {
		channel.sound_length_time_acc = device.millis_to_frames(3, 9) * (256 - channel.sound_length as u32);
		channel.current_wav_index = 0;
		channel.ticks_acc = 0;
		channel.initial = false;
		// println!("reinitialize!");
	}

	let sample_rate = 2097152.0 / (2048.0 - channel.sample_rate as f32);
	// let sample_rate = 4194304.0/(32.0*(2048.0-channel.sample_rate as f32));
	if sample_rate <= device.sample_rate_f {
		channel.ticks_per_inc = (device.sample_rate_f / sample_rate).floor() as u32;
		channel.inc_wav_index_by = 1;
	} else {
		channel.ticks_per_inc = 1;
		channel.inc_wav_index_by = max!((sample_rate / device.sample_rate_f).floor() as u32, 1);
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
		state.c3_volume_multiplier = 0.75;
	} else {
		state.c3_volume_multiplier =  match channel.sound_volume {
			0 => 0.0,
			1 => 1.0,
			2 => 0.5,
			3 => 0.25,
			_ => unreachable!()
		};
	}


	// pyrite_debugging!({
	// 	println!("CONTENTS BANK 0:");
	// 	for i in 0..8 {
	// 		print!("{:04X}", channel.wav_ram[0][i]);
	// 	}

	// 	println!("\nCONTENTS BANK 1:");
	// 	for i in 0..8 {
	// 		print!("{:04X}", channel.wav_ram[1][i]);
	// 	}
	// 	println!("");
	// 	println!("");
	// });
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
	let channel: &mut GbaChannel3 = unsafe { std::mem::transmute(&mut cpu.memory.internal_regs.audio_channel3 as *mut GbaChannel3) };
	if channel.channel_on && (!channel.length_flag || channel.sound_length_time_acc > 0) {
		let wav_idx = channel.current_wav_index & 0x1f;
		let bank = if channel.wav_ram_banked {
			if (channel.current_wav_index & 0x3f) > 31 {
				if channel.wav_ram_bank == 0 { 1usize }
				else { 0usize }
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

		// if pyrite_counter_diff!(8, wav_idx) {
		// 	print!("{:01X}", sample);
		// }

		channel.ticks_acc += 1;
		if channel.ticks_acc >= channel.ticks_per_inc {
			channel.current_wav_index += channel.inc_wav_index_by as usize;
			channel.ticks_acc = 0;
		}

		if channel.length_flag {
			channel.sound_length_time_acc -= 1;
			if channel.sound_length_time_acc == 0 {
				let soundcnt_x = cpu.memory.get_reg(ioreg::SOUNDCNT_X);
				cpu.memory.set_reg(ioreg::SOUNDCNT_X, soundcnt_x & !4); // turn the sound off.
			}
		}

		return apply_volume_stereo(convert_sample(sample), 1.0);
	}
	return apply_volume_stereo(std::i16::MIN, 1.0);
}