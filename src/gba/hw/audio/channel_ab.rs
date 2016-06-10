use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel4;
use super::super::super::hw::dma;
use super::{AudioState, apply_volume_stereo};
use std;

fn convert_sample(sample8: i8) -> i16 {
	return (sample8 as i16) << 8;
}

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
	cpu.memory.internal_regs.audio_fifo_a.freq_inc = 
		cpu.memory.internal_regs.audio_fifo_a.frequency / device.sample_rate_f;
	cpu.memory.internal_regs.audio_fifo_b.freq_inc = 
		cpu.memory.internal_regs.audio_fifo_b.frequency / device.sample_rate_f;

	pyrite_debugging!({
		println!("SAMPLE RATE: {}, {}", cpu.memory.internal_regs.audio_fifo_a.frequency,
			 cpu.memory.internal_regs.audio_fifo_a.freq_inc);
	});
}

pub fn tick_a(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
	cpu.memory.internal_regs.audio_fifo_a.freq_acc += cpu.memory.internal_regs.audio_fifo_a.freq_inc;
	let mut sample8 = cpu.memory.internal_regs.audio_fifo_a.sample;

	let mut print = true;

	// while cpu.memory.internal_regs.audio_fifo_a.freq_acc >= 1.0 {
	// 	print = true;
	// 	cpu.memory.internal_regs.audio_fifo_a.next_sample();
	// 	sample8 = cpu.memory.internal_regs.audio_fifo_a.sample;
	// 	cpu.memory.internal_regs.audio_fifo_a.freq_acc -= 1.0
	// }
	let sample16 = convert_sample(sample8);

	print &= sample16 != 0 && sample16 != -32768;

	// if print {
 // 		println!("Playing Sample: [sample: {} ({} [0x{:02X}])] [len: {}] [freq: {}]", 
 // 			sample16, 
 // 			sample8,
 // 			sample8 as u8,
 // 			cpu.memory.internal_regs.audio_fifo_a.out_remaining(),
 // 			cpu.memory.internal_regs.audio_fifo_a.frequency);
	// }

	return (sample16, sample16);
}

pub fn tick_b(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
	return (std::i16::MIN, std::i16::MIN);
}

pub fn timer_overflow(cpu: &mut ArmCpu, timer: u16) {
	if cpu.memory.internal_regs.audio_fifo_a.timer == timer {
		// println!("Sending sample from fifo A: {}", timer);
		let sample = cpu.memory.internal_regs.audio_fifo_a.pop();
		// pyrite_debugging!({
			//println!("PUSHING A: {}", sample);
		// });
		cpu.memory.internal_regs.audio_fifo_a.sample = sample;// out_push(sample);
		if cpu.memory.internal_regs.audio_fifo_a.remaining() < 16 {
			// println!("refilling fifo A: {}", timer);
			dma::check_started(cpu, dma::DMA_TIMING_SPECIAL, 1);
			dma::check_started(cpu, dma::DMA_TIMING_SPECIAL, 2);
		}
	}

	// if cpu.memory.internal_regs.audio_fifo_b.timer == timer {
	// 	// println!("Sending sample from fifo B: {}", timer);
	// 	let sample = cpu.memory.internal_regs.audio_fifo_b.pop();
	// 	cpu.memory.internal_regs.audio_fifo_b.out_push(sample);
	// 	if cpu.memory.internal_regs.audio_fifo_b.remaining() < 16 {
	// 		// println!("refilling fifo B: {}", timer);
	// 		dma::check_started(cpu, dma::DMA_TIMING_SPECIAL, 1);
	// 		dma::check_started(cpu, dma::DMA_TIMING_SPECIAL, 2);
	// 	}
	// }
}