use super::super::core::cpu::ArmCpu;
// use super::super::core::memory::ioreg::DMAInternalReg;
use super::super::core::memory::ioreg::IORegister32;
use super::super::core::memory::ioreg::IORegister16;
use super::super::core::memory::*;

pub const DMA_TIMING_IMMEDIATE: u16 = 0;
pub const DMA_TIMING_VBLANK: u16 = 1;
pub const DMA_TIMING_HBLANK: u16 = 2;
pub const DMA_TIMING_SPECIAL: u16 = 3; // #TODO where the fuck?

/// DMA 0 Interrupt
pub const INT_DMA0: u16 = 0x100;

// /// DMA 1 Interrupt
// pub const INT_DMA1: u16 = 0x200;

// /// DMA 2 Interrupt
// pub const INT_DMA2: u16 = 0x400;

// /// DMA 3 Interrupt
// pub const INT_DMA3: u16 = 0x800;

struct DmaChannel {
	#[allow(dead_code)]
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

pub trait DmaHandler {
	fn dma_ongoing(&mut self) -> bool;
	fn dma_tick(&mut self) -> u16;
	fn dma_tick_channel(&mut self, channel_index: usize) -> u16;
	fn dma_check_started(&mut self, timing: u16, channel_index: usize);
	fn dma_start(&mut self, channel_index: usize);

	/// Returns an interrupt mask for the DMA register that is completed if
	/// IRQ interrupts are enabled for the given DMA channel.
	fn dma_completed(&mut self, channel_index: usize) -> u16;
}

macro_rules! dma_reg {
    ($container:expr, $channel_index:expr) => (
    	$container.memory.internal_regs.dma_registers[$channel_index];
    )
}

impl DmaHandler for ArmCpu {
	fn dma_ongoing(&mut self) -> bool {
		(dma_reg!(self, 0).units_remaining != 0 && dma_reg!(self, 0).enabled) |
		(dma_reg!(self, 1).units_remaining != 0 && dma_reg!(self, 1).enabled) |
		(dma_reg!(self, 2).units_remaining != 0 && dma_reg!(self, 2).enabled) |
		(dma_reg!(self, 3).units_remaining != 0 && dma_reg!(self, 3).enabled)
	}

	fn dma_tick(&mut self) -> u16 {
		if dma_reg!(self, 0).units_remaining > 0 && 
			dma_reg!(self, 0).enabled {
				self.dma_tick_channel(0)
		} else if dma_reg!(self, 1).units_remaining > 0 && 
			dma_reg!(self, 1).enabled {
				self.dma_tick_channel(1)
		} else if dma_reg!(self, 2).units_remaining > 0 && 
			dma_reg!(self, 2).enabled {
				self.dma_tick_channel(2)
		} else if dma_reg!(self, 3).units_remaining > 0 && 
			dma_reg!(self, 3).enabled {
				self.dma_tick_channel(3)
		} else {
			0
		}
	}

	fn dma_tick_channel(&mut self, channel_index: usize) -> u16 {
		let src = dma_reg!(self, channel_index).source_addr;
		let dest = dma_reg!(self, channel_index).destination_addr;
		if dma_reg!(self, channel_index).transfer_word {
			dma_reg!(self, channel_index).units_remaining -= 1;
			let data = self.memory.read32(src);
			self.memory.write32(dest, data);

			if dma_reg!(self, channel_index).first_transfer {
				dma_reg!(self, channel_index).first_transfer = false;
				self.clock.data_access32_nonseq(src);
				self.clock.data_access32_nonseq(dest);

				// Internal time for DMA processing is 2I (normally), or 4I (if both source and destination are in gamepak memory area).
				let src_area = (src >> 24) & 0xFF;
				let dest_area = (dest >> 24) & 0xFF;
				if src_area > 0x07 && dest_area > 0x07 {
					self.clock.internal(4);
				} else {
					self.clock.internal(2);
				}
			} else {
				self.clock.data_access32_seq(src);
				self.clock.data_access32_seq(dest);
			}

			dma_reg!(self, channel_index).destination_addr += dma_reg!(self, channel_index).dest_addr_inc;
			dma_reg!(self, channel_index).source_addr += dma_reg!(self, channel_index).source_addr_inc;
		} else {
			dma_reg!(self, channel_index).units_remaining -= 1;
			let data = self.memory.read16(src);
			self.memory.write16(dest, data);

			if dma_reg!(self, channel_index).first_transfer {
				dma_reg!(self, channel_index).first_transfer = false;
				self.clock.data_access16_nonseq(src);
				self.clock.data_access16_nonseq(dest);

				// Internal time for DMA processing is 2I (normally), or 4I (if both source and destination are in gamepak memory area).
				let src_area = (src >> 24) & 0xFF;
				let dest_area = (dest >> 24) & 0xFF;
				if src_area > 0x07 && dest_area > 0x07 {
					self.clock.internal(4);
				} else {
					self.clock.internal(2);
				}
			} else {
				self.clock.data_access16_seq(src);
				self.clock.data_access16_seq(dest);
			}

			dma_reg!(self, channel_index).destination_addr += dma_reg!(self, channel_index).dest_addr_inc;
			dma_reg!(self, channel_index).source_addr += dma_reg!(self, channel_index).source_addr_inc;
		}

		return if dma_reg!(self, channel_index).units_remaining == 0 {
			// The DMA is completed:
			self.dma_completed(channel_index)
		} else {
			// The DMA is still going
			0
		}
	}


	fn dma_check_started(&mut self, timing: u16, channel_index: usize) {
		// #FIXME can a DMA interrupt itself if it has taken too long (let's say an Hblank DMA?)

		// DMA is not ongoing, is enabled and the timing is correct
		if dma_reg!(self, channel_index).units_remaining == 0
			&& dma_reg!(self, channel_index).enabled
			&& dma_reg!(self, channel_index).start_timing == timing {
				self.dma_start(channel_index);
		}
	}

	fn dma_start(&mut self, channel_index: usize) {
		let channel_info = &CHANNELS[channel_index];

		let _dest = self.memory.get_reg(channel_info.reg_dad);
		let _source = self.memory.get_reg(channel_info.reg_sad);
		let _units = self.memory.get_reg(channel_info.reg_cnt_l) as u32;

		if !dma_reg!(self, channel_index).is_repeat {
			dma_reg!(self, channel_index).destination_addr = _dest & channel_info.dest_mask;
			dma_reg!(self, channel_index).original_destination_addr = dma_reg!(self, channel_index).destination_addr;
			dma_reg!(self, channel_index).source_addr = _source & channel_info.src_mask;
		}
		dma_reg!(self, channel_index).units = if _units == 0 { channel_info.max_units } else { _units };
		dma_reg!(self, channel_index).units_remaining = dma_reg!(self, channel_index).units;
		dma_reg!(self, channel_index).first_transfer = true;
	}

	/// Returns an interrupt mask for the DMA register that is completed if
	/// IRQ interrupts are enabled for the given DMA channel.
	fn dma_completed(&mut self, channel_index: usize) -> u16 {
		if !dma_reg!(self, channel_index).repeat {
			// We clear the enable bit if the DMA is not repeating.
			let dma_cnt_h = self.memory.get_reg(CHANNELS[channel_index].reg_cnt_h);
			self.memory.set_reg(CHANNELS[channel_index].reg_cnt_h, dma_cnt_h & 0x7fff);
		} else {
			dma_reg!(self, channel_index).is_repeat = true;
			if dma_reg!(self, channel_index).reload {
				let _end  =dma_reg!(self, channel_index).destination_addr;
				dma_reg!(self, channel_index).destination_addr = dma_reg!(self, channel_index).original_destination_addr;
			}
		}

		dma_reg!(self, channel_index).units_remaining = 0; // Make sure the units are 0
		dma_reg!(self, channel_index).first_transfer = true;

		if dma_reg!(self, channel_index).irq {
			return INT_DMA0 << (channel_index as u16);
		} else {
			return 0
		}
	}
}
