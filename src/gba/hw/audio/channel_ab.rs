use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::hw::dma;

fn convert_sample(sample8: i8) -> i16 {
	return (sample8 as i16) << 8;
}

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice) {
	cpu.memory.internal_regs.audio_fifo_a.freq_inc = 
		cpu.memory.internal_regs.audio_fifo_a.frequency / device.sample_rate_f;
	cpu.memory.internal_regs.audio_fifo_b.freq_inc = 
		cpu.memory.internal_regs.audio_fifo_b.frequency / device.sample_rate_f;
}

pub fn tick_a(cpu: &mut ArmCpu) -> i16 {
	let mut sample8 = cpu.memory.internal_regs.audio_fifo_a.sample;
	cpu.memory.internal_regs.audio_fifo_a.freq_acc += cpu.memory.internal_regs.audio_fifo_a.freq_inc;
	while cpu.memory.internal_regs.audio_fifo_a.freq_acc >= 1.0 {
		sample8 = cpu.memory.internal_regs.audio_fifo_a.next_sample();
		cpu.memory.internal_regs.audio_fifo_a.freq_acc -= 1.0;
	}
	let sample16 = convert_sample(sample8);
	return sample16;
}

pub fn tick_b(cpu: &mut ArmCpu) -> i16 {
	let mut sample8 = cpu.memory.internal_regs.audio_fifo_b.sample;
	cpu.memory.internal_regs.audio_fifo_b.freq_acc += cpu.memory.internal_regs.audio_fifo_b.freq_inc;
	while cpu.memory.internal_regs.audio_fifo_b.freq_acc >= 1.0 {
		sample8 = cpu.memory.internal_regs.audio_fifo_b.next_sample();
		cpu.memory.internal_regs.audio_fifo_b.freq_acc -= 1.0
	}
	let sample16 = convert_sample(sample8);
	return sample16;
}

fn start_dma_fifo_addr_check(cpu: &mut ArmCpu, fifo_addr: u32, dma_index: usize) {
	let dest = dma::get_destination(cpu, dma_index);
	if dest == fifo_addr {
		dma::check_started(cpu, dma::DMA_TIMING_SPECIAL, dma_index);
	}
}

pub fn timer_overflow(cpu: &mut ArmCpu, timer: u16) {
	const FIFO_A_ADDR: u32 = 0x040000A0;
	const FIFO_B_ADDR: u32 = 0x040000A4;

	if cpu.memory.internal_regs.audio_fifo_a.timer == timer {
		if cpu.memory.internal_regs.audio_fifo_a.remaining() > 0 {
			::debug::debugger::get_debugger().sample_counter += 1;
			let sample = cpu.memory.internal_regs.audio_fifo_a.pop();
			cpu.memory.internal_regs.audio_fifo_a.out_push(sample);
		}
		if cpu.memory.internal_regs.audio_fifo_a.remaining() <= 16 {
			::debug::debugger::get_debugger().dma_counter += 1;
			start_dma_fifo_addr_check(cpu, FIFO_A_ADDR, 1);
			start_dma_fifo_addr_check(cpu, FIFO_A_ADDR, 2);
		}
	}

	if cpu.memory.internal_regs.audio_fifo_b.timer == timer {
		if cpu.memory.internal_regs.audio_fifo_b.remaining() > 0 {
			let sample = cpu.memory.internal_regs.audio_fifo_b.pop();
			cpu.memory.internal_regs.audio_fifo_b.out_push(sample);
		}
		if cpu.memory.internal_regs.audio_fifo_b.remaining() <= 16 {
			start_dma_fifo_addr_check(cpu, FIFO_B_ADDR, 1);
			start_dma_fifo_addr_check(cpu, FIFO_B_ADDR, 2);
		}
	}
}