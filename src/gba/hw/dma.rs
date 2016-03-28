use super::super::core::cpu::ArmCpu;
use super::super::core::memory::ioreg::IORegister32;
use super::super::core::memory::ioreg::IORegister16;
use super::super::core::memory::*;

pub const DMA_TIMING_IMMEDIATE: u16 = 0;
pub const DMA_TIMING_VBLANK: u16 = 1;
pub const DMA_TIMING_HBLANK: u16 = 2;
pub const DMA_TIMING_SPECIAL: u16 = 3; // #TODO where the fuck?

struct DmaChannel {
	index: u8,
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
		index: 0,
		reg_sad: ioreg::DMA0SAD,
		reg_dad: ioreg::DMA0DAD,
		reg_cnt_l: ioreg::DMA0CNT_L,
		reg_cnt_h: ioreg::DMA0CNT_H,
		src_mask: 0x07ffffff,
		dest_mask: 0x07ffffff,
		max_units: 0x4000
	},
	DmaChannel {
		index: 1,
		reg_sad: ioreg::DMA1SAD,
		reg_dad: ioreg::DMA1DAD,
		reg_cnt_l: ioreg::DMA1CNT_L,
		reg_cnt_h: ioreg::DMA1CNT_H,
		src_mask: 0x0fffffff,
		dest_mask: 0x07ffffff,
		max_units: 0x4000
	},
	DmaChannel {
		index: 2,
		reg_sad: ioreg::DMA2SAD,
		reg_dad: ioreg::DMA2DAD,
		reg_cnt_l: ioreg::DMA2CNT_L,
		reg_cnt_h: ioreg::DMA2CNT_H,
		src_mask: 0x0fffffff,
		dest_mask: 0x07ffffff,
		max_units: 0x4000
	},
	DmaChannel {
		index: 3,
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
	pub dma_cycles: u64
}

impl DmaHandler {
	pub fn new() -> DmaHandler {
		Default::default()
	}

	#[inline(always)]
	pub fn try_start_dma(&mut self, cpu: &mut ArmCpu, timing: u16, channel_index: usize) -> bool {
		let channel = &CHANNELS[channel_index];
		let dma_cnt_h = cpu.memory.get_reg(channel.reg_cnt_h);
		if ((dma_cnt_h >> 15) & 1) != 0 && ((dma_cnt_h >> 12) & 0x3) == timing { // DMA is enabled && Timing is correct
			self.start_dma(cpu, dma_cnt_h, channel);
			true
		} else {
			false
		}
	}

	/// Checks a DMA channel and starts it and returns true if a DMA transfer was done.
	fn start_dma(&mut self, cpu: &mut ArmCpu, dma_cnt_h: u16, channel: &DmaChannel) -> bool {
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
			println!("DMA32[{}] {} units from 0x{:08x} to 0x{:08x} | {}, {}", channel.index, units, src_addr, dest_addr,
				src_inc as i32, dest_inc as i32);
			ending_dest = self.do_dma_transfer32(cpu, src_addr, dest_addr, units, src_inc, dest_inc);
		} else {
			let src_inc = Self::get_increment((dma_cnt_h >> 5) & 0x3, 2);
			let dest_inc = Self::get_increment((dma_cnt_h >> 7) & 0x3, 2);
			println!("DMA16[{}] {} units from 0x{:08x} to 0x{:08x} | {}, {}", channel.index, units, src_addr, dest_addr,
				src_inc as i32, dest_inc as i32);
			ending_dest = self.do_dma_transfer16(cpu, src_addr, dest_addr, units, src_inc, dest_inc);
		}

		if ((dma_cnt_h >> 5) & 0x3) == 0x3 {
			cpu.memory.set_reg(channel.reg_dad, ending_dest & channel.dest_mask);
		}

		if ((dma_cnt_h >> 9) & 1) == 0 { // If this is not repeating.
			cpu.memory.set_reg(channel.reg_cnt_h, dma_cnt_h & 0x7fff); // clears the enable bit.
			return false
		}
		return true
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
			let seq = (units as u64) - 1;
			(cpu.clock.get_nonseq_cycles16(src_addr) as u64) + (cpu.clock.get_nonseq_cycles16(dest_addr) as u64) +
			((cpu.clock.get_seq_cycles16(src_addr) as u64) * seq) + ((cpu.clock.get_seq_cycles16(dest_addr) as u64) * seq)
		} else {
			(cpu.clock.get_nonseq_cycles16(src_addr) as u64)+ (cpu.clock.get_nonseq_cycles16(dest_addr) as u64)
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
			let data = cpu.memory.read32(src);
			cpu.memory.write32(dest, data);
			src += src_inc;
			dest += dest_inc;
		}

		// -- Timing Stuff ---
		self.dma_cycles += if units > 1 {
			let seq = (units as u64) - 1;
			(cpu.clock.get_nonseq_cycles32(src_addr) as u64)+ (cpu.clock.get_nonseq_cycles32(dest_addr) as u64) +
			((cpu.clock.get_seq_cycles32(src_addr) as u64) * seq) + ((cpu.clock.get_seq_cycles32(dest_addr) as u64) * seq)
		} else {
			(cpu.clock.get_nonseq_cycles32(src_addr) as u64)+ (cpu.clock.get_nonseq_cycles32(dest_addr) as u64)
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
