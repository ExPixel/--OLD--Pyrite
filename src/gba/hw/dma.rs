use super::super::core::cpu::ArmCpu;
// use super::super::core::memory::ioreg::DMAInternalReg;
use super::super::core::memory::ioreg::IORegister32;
use super::super::core::memory::ioreg::IORegister16;
use super::super::core::memory::*;

pub const DMA_TIMING_IMMEDIATE: u16 = 0;
pub const DMA_TIMING_VBLANK: u16 = 1;
pub const DMA_TIMING_HBLANK: u16 = 2;
pub const DMA_TIMING_SPECIAL: u16 = 3;

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

macro_rules! dma_reg {
    ($container:expr, $channel_index:expr) => (
    	$container.memory.internal_regs.dma_registers[$channel_index];
    )
}

pub fn ongoing(cpu: &ArmCpu) -> bool {
	(dma_reg!(cpu, 0).units_remaining != 0 && dma_reg!(cpu, 0).enabled) |
	(dma_reg!(cpu, 1).units_remaining != 0 && dma_reg!(cpu, 1).enabled) |
	(dma_reg!(cpu, 2).units_remaining != 0 && dma_reg!(cpu, 2).enabled) |
	(dma_reg!(cpu, 3).units_remaining != 0 && dma_reg!(cpu, 3).enabled)
}

pub fn tick(cpu: &mut ArmCpu) {
	if dma_reg!(cpu, 0).units_remaining > 0 && dma_reg!(cpu, 0).enabled {
		tick_channel(cpu, 0)
	} else if dma_reg!(cpu, 1).units_remaining > 0 && dma_reg!(cpu, 1).enabled {
		tick_channel(cpu, 1)
	} else if dma_reg!(cpu, 2).units_remaining > 0 && dma_reg!(cpu, 2).enabled {
		tick_channel(cpu, 2)
	} else if dma_reg!(cpu, 3).units_remaining > 0 && dma_reg!(cpu, 3).enabled {
		tick_channel(cpu, 3)
	}
}

fn tick_channel(cpu: &mut ArmCpu, channel_index: usize) {
	let src = dma_reg!(cpu, channel_index).source_addr;
	let dest = dma_reg!(cpu, channel_index).destination_addr;
	if dma_reg!(cpu, channel_index).transfer_word {
		dma_reg!(cpu, channel_index).units_remaining -= 1;
		let data = cpu.memory.read32(src);
		cpu.memory.write32(dest, data);

		if channel_index == 1 && dest == 0x040000A0 {
			::debug::debugger::get_debugger().dma_transfer_counter += 1;
		}

		if dma_reg!(cpu, channel_index).first_transfer {
			dma_reg!(cpu, channel_index).first_transfer = false;
			cpu.clock.data_access32_nonseq(src);
			cpu.clock.data_access32_nonseq(dest);

			// Internal time for DMA processing is 2I (normally), or 4I (if both source and destination are in gamepak memory area).
			let src_area = (src >> 24) & 0xFF;
			let dest_area = (dest >> 24) & 0xFF;
			if src_area > 0x07 && dest_area > 0x07 {
				cpu.clock.internal(4);
			} else {
				cpu.clock.internal(2);
			}
		} else {
			cpu.clock.data_access32_seq(src);
			cpu.clock.data_access32_seq(dest);
		}

		dma_reg!(cpu, channel_index).destination_addr += dma_reg!(cpu, channel_index).dest_addr_inc;
		dma_reg!(cpu, channel_index).source_addr += dma_reg!(cpu, channel_index).source_addr_inc;
	} else {
		dma_reg!(cpu, channel_index).units_remaining -= 1;
		let data = cpu.memory.read16(src);
		cpu.memory.write16(dest, data);

		if dma_reg!(cpu, channel_index).first_transfer {
			dma_reg!(cpu, channel_index).first_transfer = false;
			cpu.clock.data_access16_nonseq(src);
			cpu.clock.data_access16_nonseq(dest);

			// Internal time for DMA processing is 2I (normally), or 4I (if both source and destination are in gamepak memory area).
			let src_area = (src >> 24) & 0xFF;
			let dest_area = (dest >> 24) & 0xFF;
			if src_area > 0x07 && dest_area > 0x07 {
				cpu.clock.internal(4);
			} else {
				cpu.clock.internal(2);
			}
		} else {
			cpu.clock.data_access16_seq(src);
			cpu.clock.data_access16_seq(dest);
		}

		dma_reg!(cpu, channel_index).destination_addr += dma_reg!(cpu, channel_index).dest_addr_inc;
		dma_reg!(cpu, channel_index).source_addr += dma_reg!(cpu, channel_index).source_addr_inc;
	}

	return if dma_reg!(cpu, channel_index).units_remaining == 0 {
		// The DMA is completed:
		completed(cpu, channel_index)
	}
}

pub fn check_started(cpu: &mut ArmCpu, timing: u16, channel_index: usize) {
	// #FIXME can a DMA interrupt itcpu if it has taken too long (let's say an Hblank DMA?)

	// DMA is not ongoing, is enabled and the timing is correct
	if dma_reg!(cpu, channel_index).units_remaining == 0
		&& dma_reg!(cpu, channel_index).enabled
		&& dma_reg!(cpu, channel_index).start_timing == timing {
			start(cpu, channel_index);
	}
}

fn is_fifo_dest(dest: u32) -> bool {
	return dest == 0x040000A0 || dest == 0x040000A4;
}

#[inline]
pub fn get_destination(cpu: &mut ArmCpu, channel_index: usize) -> u32 {
	let _dest = cpu.memory.get_reg(CHANNELS[channel_index].reg_dad);
	return _dest & CHANNELS[channel_index].dest_mask;
}

fn start(cpu: &mut ArmCpu, channel_index: usize) {
	let channel_info = &CHANNELS[channel_index];

	let _dest = cpu.memory.get_reg(channel_info.reg_dad);
	let _source = cpu.memory.get_reg(channel_info.reg_sad);
	let _units = cpu.memory.get_reg(channel_info.reg_cnt_l) as u32 & (channel_info.max_units - 1);

	if !dma_reg!(cpu, channel_index).is_repeat {
		dma_reg!(cpu, channel_index).destination_addr = _dest & channel_info.dest_mask;
		dma_reg!(cpu, channel_index).original_destination_addr = dma_reg!(cpu, channel_index).destination_addr;
		dma_reg!(cpu, channel_index).source_addr = _source & channel_info.src_mask;
	}

	if (channel_index == 1 || channel_index == 2) &&
		dma_reg!(cpu, channel_index).start_timing == DMA_TIMING_SPECIAL && 
		is_fifo_dest(dma_reg!(cpu, channel_index).destination_addr) {
		// #TODO should I check the repeat bit as well? It's supposed to be set.
		dma_reg!(cpu, channel_index).dest_addr_inc = 0;
		dma_reg!(cpu, channel_index).units = 4;
		dma_reg!(cpu, channel_index).transfer_word = true;
	} else {
		dma_reg!(cpu, channel_index).units = if _units == 0 { channel_info.max_units } else { _units };
	}
	dma_reg!(cpu, channel_index).units_remaining = dma_reg!(cpu, channel_index).units;
	dma_reg!(cpu, channel_index).first_transfer = true;

	dma_reg!(cpu, channel_index).is_repeat = false;
}

/// Returns an interrupt mask for the DMA register that is completed if
/// IRQ interrupts are enabled for the given DMA channel.
fn completed(cpu: &mut ArmCpu, channel_index: usize) {
	if !dma_reg!(cpu, channel_index).repeat {
		// We clear the enable bit if the DMA is not repeating.
		dma_reg!(cpu, channel_index).enabled = false;
		let dma_cnt_h = cpu.memory.get_reg(CHANNELS[channel_index].reg_cnt_h);
		cpu.memory.set_reg(CHANNELS[channel_index].reg_cnt_h, dma_cnt_h & 0x7fff);
	} else {
		let _dest = cpu.memory.get_reg(CHANNELS[channel_index].reg_dad);
		dma_reg!(cpu, channel_index).is_repeat = true;
		if dma_reg!(cpu, channel_index).reload {
			let _end  =dma_reg!(cpu, channel_index).destination_addr;
			dma_reg!(cpu, channel_index).destination_addr = _dest & CHANNELS[channel_index].dest_mask;
		}
	}

	dma_reg!(cpu, channel_index).units_remaining = 0; // Make sure the units are 0
	dma_reg!(cpu, channel_index).first_transfer = true;

	if dma_reg!(cpu, channel_index).irq {
		cpu.hardware_interrupt(INT_DMA0 << (channel_index as u16));
	}
}
