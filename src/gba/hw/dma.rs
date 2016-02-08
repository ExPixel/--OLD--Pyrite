use super::super::core::cpu::ArmCpu;
use super::super::core::memory::ioreg::IORegister32;
use super::super::core::memory::ioreg::IORegister16;
use super::super::core::memory::*;

struct DmaChannel {
	reg_sad: IORegister32,
	reg_dad: IORegister32,
	reg_cnt_l: IORegister16,
	reg_cnt_h: IORegister16,

	src_mask: u32,
	dest_mask: u32,
	max_units: u32
}

const CHANNELS: [DmaChannel; 4] = [
	DmaChannel { 
		reg_sad: ioreg::DMA0SAD,
		reg_dad: ioreg::DMA0DAD,
		reg_cnt_l: ioreg::DMA0CNT_L,
		reg_cnt_h: ioreg::DMA0CNT_H,
		src_mask: 0x07ffffff,
		dest_mask: 0x07ffffff,
		max_units: 0x4000
	},
	DmaChannel {
		reg_sad: ioreg::DMA1SAD,
		reg_dad: ioreg::DMA1DAD,
		reg_cnt_l: ioreg::DMA1CNT_L,
		reg_cnt_h: ioreg::DMA1CNT_H,
		src_mask: 0x0fffffff,
		dest_mask: 0x07ffffff,
		max_units: 0x4000
	},
	DmaChannel {
		reg_sad: ioreg::DMA2SAD,
		reg_dad: ioreg::DMA2DAD,
		reg_cnt_l: ioreg::DMA2CNT_L,
		reg_cnt_h: ioreg::DMA2CNT_H,
		src_mask: 0x0fffffff,
		dest_mask: 0x07ffffff,
		max_units: 0x4000
	},
	DmaChannel {
		reg_sad: ioreg::DMA3SAD,
		reg_dad: ioreg::DMA3DAD,
		reg_cnt_l: ioreg::DMA3CNT_L,
		reg_cnt_h: ioreg::DMA3CNT_H,
		src_mask: 0x0fffffff,
		dest_mask: 0x0fffffff,
		max_units: 0x10000
	}
];

#[derive(Default)]
pub struct DmaHandler {
	dma_cycles: u32
}

/*
This is wrong. DMAs should be able to interrupt each other I think
so I'm going to switch to a different model where they can. I should
use an array of ongoing DMA transfers (by moving in_dma) to the DmaInternal
struct itself. And then each tick I can check which is the highest priority
DMA transfer currently being done and move along with that one instead.
To speed things up, I can probably pass along the target cycles to the DMA
transfer and keep the code running here in the DmaHandler rather than passing
back and forth between the Gba and the DmaHandler and running for loops everywhere.
*/

impl DmaHandler {
	pub fn new() -> DmaHandler {
		Default::default()
	}

	pub fn check_dmas(&mut self, cpu: &mut ArmCpu, timing: u16) {
		self.try_start_dma(cpu, timing, 0);
		self.try_start_dma(cpu, timing, 1);
		self.try_start_dma(cpu, timing, 2);
		self.try_start_dma(cpu, timing, 3);
	}

	#[inline(always)]
	fn try_start_dma(&mut self, cpu: &mut ArmCpu, timing: u16, channel_index: usize) {
		let channel = &CHANNELS[channel_index];
		let dma_cnt_h = cpu.memory.get_reg(channel.reg_cnt_h);
		if ((dma_cnt_h >> 15) & 1) != 0 && ((dma_cnt_h >> 12) & 0x3) == timing { // DMA is enabled && Timing is correct
			self.start_dma(cpu, dma_cnt_h, channel);
		}
	}

	/// Checks a DMA channel and starts it and returns true if a DMA transfer was done.
	fn start_dma(&mut self, cpu: &mut ArmCpu, dma_cnt_h: u16, channel: &DmaChannel) {
		let src_addr = cpu.memory.get_reg(channel.reg_sad) & channel.src_mask;
		let dest_addr = cpu.memory.get_reg(channel.reg_dad) & channel.dest_mask;
		let mut units = cpu.memory.get_reg(channel.reg_cnt_l) as u32;

		if units == 0 {
			units = channel.max_units
		} else {
			units &= channel.max_units - 1;
		}

		let ending_dest;

		if ((dma_cnt_h >> 10) & 1) != 0 {
			let src_inc = Self::get_increment((dma_cnt_h >> 5) & 0x3, 4);
			let dest_inc = Self::get_increment((dma_cnt_h >> 7) & 0x3, 4);
			ending_dest = self.do_dma_transfer32(cpu, src_addr, dest_addr, units, src_inc, dest_inc);
		} else {
			let src_inc = Self::get_increment((dma_cnt_h >> 5) & 0x3, 2);
			let dest_inc = Self::get_increment((dma_cnt_h >> 7) & 0x3, 2);
			ending_dest = self.do_dma_transfer16(cpu, src_addr, dest_addr, units, src_inc, dest_inc);
		}

		if ((dma_cnt_h >> 5) & 0x3) == 0x3 {
			cpu.memory.set_reg(channel.reg_dad, ending_dest & channel.dest_mask);
		}

		if ((dma_cnt_h >> 9) & 1) == 0 { // If this is not repeating.
			cpu.memory.set_reg(channel.reg_cnt_h, dma_cnt_h & 0x7fff); // clears the enable bit.
		}
	}

	fn get_increment(n: u16, size: i32) -> u32 {
		let inc = match n {
			0 | 3 => size,
			1 => size * -1,
			2 => 0,
			_ => unreachable!()
		};
		return inc as u32
	}

	fn do_dma_transfer16(&mut self, cpu: &mut ArmCpu, src_addr: u32, dest_addr: u32, units: u32, src_inc: u32, dest_inc: u32) -> u32 {
		let mut src = src_addr;
		let mut dest = dest_addr;
		for _ in 0..units {
			let data = cpu.memory.read16(src);
			cpu.memory.write16(dest, data);
			src += src_inc;
			dest += dest_inc;
		}

		// -- Timing Stuff ---
		self.dma_cycles += if units > 1 {
			(cpu.clock.get_nonseq_cycles16(src_addr) as u32) + (cpu.clock.get_nonseq_cycles16(dest_addr) as u32) +
			((cpu.clock.get_seq_cycles16(src_addr) as u32) * (units - 1)) + ((cpu.clock.get_seq_cycles16(dest_addr) as u32) * (units - 1))
		} else {
			(cpu.clock.get_nonseq_cycles16(src_addr) as u32)+ (cpu.clock.get_nonseq_cycles16(dest_addr) as u32)
		};

		// 2 internal cycles unless both dest_addr and src_addr are in gamepak memory area.
		let src_area = (src_addr >> 24) & 0xFF;
		let dest_area = (dest_addr >> 24) & 0xFF;
		if src_area > 0x07 && dest_area > 0x07 {
			self.dma_cycles += 4;
		} else {
			self.dma_cycles += 2;
		}

		dest
	}


	fn do_dma_transfer32(&mut self, cpu: &mut ArmCpu, src_addr: u32, dest_addr: u32, units: u32, src_inc: u32, dest_inc: u32) -> u32 {
		let mut src = src_addr;
		let mut dest = dest_addr;
		for _ in 0..units {
			let data = cpu.memory.read16(src);
			cpu.memory.write16(dest, data);
			src += src_inc;
			dest += dest_inc;
		}

		// -- Timing Stuff ---
		self.dma_cycles += if units > 1 {
			(cpu.clock.get_nonseq_cycles32(src_addr) as u32)+ (cpu.clock.get_nonseq_cycles32(dest_addr) as u32) +
			((cpu.clock.get_seq_cycles32(src_addr) as u32) * (units - 1)) + ((cpu.clock.get_seq_cycles32(dest_addr) as u32) * (units - 1))
		} else {
			(cpu.clock.get_nonseq_cycles32(src_addr) as u32)+ (cpu.clock.get_nonseq_cycles32(dest_addr) as u32)
		};

		// 2 internal cycles unless both dest_addr and src_addr are in gamepak memory area.
		let src_area = (src_addr >> 24) & 0xFF;
		let dest_area = (dest_addr >> 24) & 0xFF;
		if src_area > 0x07 && dest_area > 0x07 {
			self.dma_cycles += 4;
		} else {
			self.dma_cycles += 2;
		}

		dest
	}
}
