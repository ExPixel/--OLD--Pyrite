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
	max_units: u16
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

struct DmaInternal {
	src_addr: u32,
	dest_addr: u32,
	units: u16,
	channel_index: usize
}

pub struct DmaHandler {
	/// The current DMA if there is one.
	current_dma: Option<DmaInternal>,

	/// Cycle count at which to begin the DMA transfer.
	/// DMAs need 2 cycles before they start.
	do_dma_at: u64
}


impl DmaHandler {
	pub fn check_dmas(&mut self, cpu: &mut ArmCpu, timing: u16) {
		if let Some(internal) = self.current_dma.take() {
			Self::complete_dma(cpu, internal);
		} else {
			for channel in 0..4 {
				if self.try_start_dma(cpu, timing, channel) {
					break;
				}
			}
		}
	}

	/// Checks a DMA channel and starts it and returns true if it is active.
	fn try_start_dma(&mut self, cpu: &mut ArmCpu, timing: u16, channel_index: usize) -> bool {
		let channel = &CHANNELS[channel_index];
		let dma_cnt_h = cpu.memory.get_reg(channel.reg_cnt_h);
		if ((dma_cnt_h >> 15) & 1) != 0 { // DMA enable.
			let src_addr = cpu.memory.get_reg(channel.reg_sad) & channel.src_mask;
			let dest_addr = cpu.memory.get_reg(channel.reg_dad) & channel.dest_mask;
			let mut units = cpu.memory.get_reg(channel.reg_cnt_l);

			if units == 0 {
				units = channel.max_units
			} else {
				units &= channel.max_units - 1;
			}

			self.current_dma = Some(DmaInternal {
				src_addr: src_addr,
				dest_addr: dest_addr,
				units: units,
				channel_index: channel_index
			});
			return true
		}
		return false
	}

	fn complete_dma(cpu: &mut ArmCpu, info: DmaInternal) {
	}
}