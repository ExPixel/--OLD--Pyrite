pub use super::*;

#[derive(Copy, Clone)]
pub struct IORegister8(pub usize);

#[derive(Copy, Clone)]
pub struct IORegister16(pub usize);

#[derive(Copy, Clone)]
pub struct IORegister32(pub usize);

// Offsets from 0x4000000

pub const POSTFLG: IORegister8 = IORegister8(0x0000300);
pub const HALTCNT: IORegister8 = IORegister8(0x0000301);
pub const DISPCNT: IORegister16 = IORegister16(0x0000000);
pub const DISPSTAT: IORegister16 = IORegister16(0x0000004);
pub const VCOUNT: IORegister16 = IORegister16(0x0000006);
pub const BG0CNT: IORegister16 = IORegister16(0x0000008);
pub const BG1CNT: IORegister16 = IORegister16(0x000000a);
pub const BG2CNT: IORegister16 = IORegister16(0x000000c);
pub const BG3CNT: IORegister16 = IORegister16(0x000000e);
pub const BG0HOFS: IORegister16 = IORegister16(0x0000010);
pub const BG0VOFS: IORegister16 = IORegister16(0x0000012);
pub const BG1HOFS: IORegister16 = IORegister16(0x0000014);
pub const BG1VOFS: IORegister16 = IORegister16(0x0000016);
pub const BG2HOFS: IORegister16 = IORegister16(0x0000018);
pub const BG2VOFS: IORegister16 = IORegister16(0x000001a);
pub const BG3HOFS: IORegister16 = IORegister16(0x000001c);
pub const BG3VOFS: IORegister16 = IORegister16(0x000001e);
pub const BG2PA: IORegister16 = IORegister16(0x0000020);
pub const BG2PB: IORegister16 = IORegister16(0x0000022);
pub const BG2PC: IORegister16 = IORegister16(0x0000024);
pub const BG2PD: IORegister16 = IORegister16(0x0000026);
pub const BG3PA: IORegister16 = IORegister16(0x0000030);
pub const BG3PB: IORegister16 = IORegister16(0x0000032);
pub const BG3PC: IORegister16 = IORegister16(0x0000034);
pub const BG3PD: IORegister16 = IORegister16(0x0000036);
pub const WIN0H: IORegister16 = IORegister16(0x0000040);
pub const WIN1H: IORegister16 = IORegister16(0x0000042);
pub const WIN0V: IORegister16 = IORegister16(0x0000044);
pub const WIN1V: IORegister16 = IORegister16(0x0000046);
pub const WININ: IORegister16 = IORegister16(0x0000048);
pub const WINOUT: IORegister16 = IORegister16(0x000004a);
pub const MOSAIC: IORegister16 = IORegister16(0x000004c);
pub const BLDCNT: IORegister16 = IORegister16(0x0000050);
pub const BLDALPHA: IORegister16 = IORegister16(0x0000052);
pub const BLDY: IORegister16 = IORegister16(0x0000054);
pub const SOUND1CNT_L: IORegister16 = IORegister16(0x0000060);
pub const SOUND1CNT_H: IORegister16 = IORegister16(0x0000062);
pub const SOUND1CNT_X: IORegister16 = IORegister16(0x0000064);
pub const SOUND2CNT_L: IORegister16 = IORegister16(0x0000068);
pub const SOUND2CNT_H: IORegister16 = IORegister16(0x000006c);
pub const SOUND3CNT_L: IORegister16 = IORegister16(0x0000070);
pub const SOUND3CNT_H: IORegister16 = IORegister16(0x0000072);
pub const SOUND3CNT_X: IORegister16 = IORegister16(0x0000074);
pub const SOUND4CNT_L: IORegister16 = IORegister16(0x0000078);
pub const SOUND4CNT_H: IORegister16 = IORegister16(0x000007c);
pub const SOUNDCNT_L: IORegister16 = IORegister16(0x0000080);
pub const SOUNDCNT_H: IORegister16 = IORegister16(0x0000082);
pub const SOUNDCNT_X: IORegister16 = IORegister16(0x0000084);
pub const SOUNDBIAS: IORegister16 = IORegister16(0x0000088);
pub const WAVE_RAM0_L: IORegister16 = IORegister16(0x0000090);
pub const WAVE_RAM0_H: IORegister16 = IORegister16(0x0000092);
pub const WAVE_RAM1_L: IORegister16 = IORegister16(0x0000094);
pub const WAVE_RAM1_H: IORegister16 = IORegister16(0x0000096);
pub const WAVE_RAM2_L: IORegister16 = IORegister16(0x0000098);
pub const WAVE_RAM2_H: IORegister16 = IORegister16(0x000009a);
pub const WAVE_RAM3_L: IORegister16 = IORegister16(0x000009c);
pub const WAVE_RAM3_H: IORegister16 = IORegister16(0x000009e);
pub const FIF0_A_L: IORegister16 = IORegister16(0x00000a0);
pub const FIFO_A_H: IORegister16 = IORegister16(0x00000a2);
pub const FIFO_B_L: IORegister16 = IORegister16(0x00000a4);
pub const FIFO_B_H: IORegister16 = IORegister16(0x00000a6);
pub const DMA0CNT_L: IORegister16 = IORegister16(0x00000b8);
pub const DMA0CNT_H: IORegister16 = IORegister16(0x00000ba);
pub const DMA1CNT_L: IORegister16 = IORegister16(0x00000c4);
pub const DMA1CNT_H: IORegister16 = IORegister16(0x00000c6);
pub const DMA2CNT_L: IORegister16 = IORegister16(0x00000d0);
pub const DMA2CNT_H: IORegister16 = IORegister16(0x00000d2);
pub const DMA3CNT_L: IORegister16 = IORegister16(0x00000dc);
pub const DMA3CNT_H: IORegister16 = IORegister16(0x00000de);
pub const TM0CNT_L: IORegister16 = IORegister16(0x0000100);
pub const TM0CNT_H: IORegister16 = IORegister16(0x0000102);
pub const TM1CNT_L: IORegister16 = IORegister16(0x0000104);
pub const TM1CNT_H: IORegister16 = IORegister16(0x0000106);
pub const TM2CNT_L: IORegister16 = IORegister16(0x0000108);
pub const TM2CNT_H: IORegister16 = IORegister16(0x000010a);
pub const TM3CNT_L: IORegister16 = IORegister16(0x000010c);
pub const TM3CNT_H: IORegister16 = IORegister16(0x000010e);
pub const SIOMULTI0: IORegister16 = IORegister16(0x0000120);
pub const SIOMULTI1: IORegister16 = IORegister16(0x0000122);
pub const SIOMULTI2: IORegister16 = IORegister16(0x0000124);
pub const SIOMULTI3: IORegister16 = IORegister16(0x0000126);
pub const SIOCNT: IORegister16 = IORegister16(0x0000128);
pub const SIOMLT_SEND: IORegister16 = IORegister16(0x000012a);
pub const KEYINPUT: IORegister16 = IORegister16(0x0000130);
pub const KEYCNT: IORegister16 = IORegister16(0x0000132);
pub const RCNT: IORegister16 = IORegister16(0x0000134);
pub const IR: IORegister16 = IORegister16(0x0000136);
pub const JOYCNT: IORegister16 = IORegister16(0x0000140);
pub const JOY_STAT: IORegister16 = IORegister16(0x0000158);
pub const IE: IORegister16 = IORegister16(0x0000200);
pub const IF: IORegister16 = IORegister16(0x0000202);
pub const WAITCNT: IORegister16 = IORegister16(0x0000204);
pub const IME: IORegister16 = IORegister16(0x0000208);
pub const BG2X: IORegister32 = IORegister32(0x0000028);
pub const BG2Y: IORegister32 = IORegister32(0x000002c);
pub const BG3X: IORegister32 = IORegister32(0x0000038);
pub const BG3Y: IORegister32 = IORegister32(0x000003c);
pub const FIFO_A: IORegister32 = IORegister32(0x00000a0);
pub const FIFO_B: IORegister32 = IORegister32(0x00000a4);
pub const DMA0SAD: IORegister32 = IORegister32(0x00000b0);
pub const DMA0DAD: IORegister32 = IORegister32(0x00000b4);
pub const DMA1SAD: IORegister32 = IORegister32(0x00000bc);
pub const DMA1DAD: IORegister32 = IORegister32(0x00000c0);
pub const DMA2SAD: IORegister32 = IORegister32(0x00000c8);
pub const DMA2DAD: IORegister32 = IORegister32(0x00000cc);
pub const DMA3SAD: IORegister32 = IORegister32(0x00000d4);
pub const DMA3DAD: IORegister32 = IORegister32(0x00000d8);
pub const SIODATA32: IORegister32 = IORegister32(0x0000120);
pub const JOY_RECV: IORegister32 = IORegister32(0x0000150);
pub const JOY_TRANS: IORegister32 = IORegister32(0x0000154);

// #TODO make the mask table.
// pub const IOREG_MASK_TABLE = [];

/// #FIXME: Not sure about the write status of 4000158h
/// Read write permissions table for IO registers.
/// 2 bits in this table represents a R/W pair for a byte in the IO registers.
/// This is a shortened table that doesn't include 0x4000300, 0x4000301 and, 0x4000800
/// and instead just returns R/W for any address not in the table.
pub const IOREG_PERMISSIONS_TABLE: [u32; 33] = [
	0xffff5fff,0xaaaaaaaa,0xaaaaaaaa,0xaaaaaaaa,0xfaffaaaa,0xfffffaaf,0xffffffff,0xffffffff,
	0xffffffff,0xffffffff,0xffffaaaa,0xaafaaaaa,0xaaaafaaa,0xfaaaaafa,0xffffffff,0xffffffff,
	0xffffffff,0xffffffff,0xffffffff,0xfffffff5,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
	0x000fffff
];

pub fn is_ioreg_addr_readable(addr: u32) -> bool {
	let ioreg_index = addr - 0x04000000;
	let permissions_index = (ioreg_index >> 4) as usize; // there are 16 registers stuffed into each index.
	if permissions_index > 32 { return true } // If it's not in our table it's just readable.
	let permission_shift = (addr & 15) << 1;
	((IOREG_PERMISSIONS_TABLE[permissions_index] >> permission_shift) & 1) == 1
}

pub fn is_ioreg_addr_writeable(addr: u32) -> bool {
	let ioreg_index = addr - 0x04000000;
	let permissions_index = (ioreg_index >> 4) as usize; // there are 16 registers stuffed into each index.
	if permissions_index > 32 { return true } // If it's not in our table it's just writeable.
	let permission_shift = ((addr & 15) << 1) + 1;
	((IOREG_PERMISSIONS_TABLE[permissions_index] >> permission_shift) & 1) == 1
}

// Old version of the table which contains all of the regiters.
// pub const IOREG_PERMISSIONS_TABLE: [u32; 129] = [
// 	0xffff5fff,0xaaaaaaaa,0xaaaaaaaa,0xaaaaaaaa,0xfaffaaaa,0xfffffaaf,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffaaaa,0xaafaaaaa,0xaaaafaaa,0xfaaaaafa,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xfffffff5,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xfffffffb,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0x000000ff
// ];

macro_rules! put_lo16 {
	($key:expr, $value:expr) => (
		($key & (0xFFFF0000)) | ($value as u32)
	)
}

macro_rules! put_hi16 {
	($key:expr, $value:expr) => (
		($key & (0x0000FFFF)) | (($value as u32) << 16)
	)
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct DMAInternalReg {
	pub reload: bool,
	pub repeat: bool,
	pub transfer_word: bool, // transfers halfwords if false
	pub gamepak_drq: bool,  // #TODO I'm not even sure what this is.
	pub start_timing: u16, // (0=Immediately, 1=VBlank, 2=HBlank, 3=Special)
	pub irq: bool,
	pub enabled: bool,

	pub dest_addr_inc: u32,
	pub source_addr_inc: u32,

	// Everything below here is set and controlled by dma.rs:
	pub is_repeat: bool,
	pub units: u32,
	pub original_destination_addr: u32,
	pub destination_addr: u32,
	pub source_addr: u32,
	pub units_remaining: u32,
	pub first_transfer: bool
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct TimerInternalReg {
	pub prescaler: u32,

	/// When Count-up Timing is enabled, the prescaler value is ignored, 
	/// instead the time is incremented each time when the previous counter overflows. 
	/// This function cannot be used for Timer 0 (as it is the first timer).
	pub count_up: bool,

	pub irq_enabled: bool,
	pub operate: bool,

	// The couter before scaling.
	pub unscaled_counter: u32,
	pub counter: u32,
	pub reload: u32
}

// Internal IO registers.
#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct InternalRegisters {
	pub bg2x: u32,
	pub bg2y: u32,
	pub bg3x: u32,
	pub bg3y: u32,

	pub halted: bool,
	pub stopped: bool,

	pub dma_dirty: bool,
	pub dma_registers: [DMAInternalReg; 4],
	pub timers: [TimerInternalReg; 4],

	pub sound_channel1_dirty: bool,
	pub sound_channel2_dirty: bool,
	pub sound_channel3_dirty: bool,
	pub sound_channel4_dirty: bool,
}

impl InternalRegisters {
	pub fn new() -> InternalRegisters {
		Default::default()
	}

	// This should almost never happen.
	pub fn on_write8(&mut self, address: u32, value: u8, iodata: &[u8]) {
		let register = address & 0x3fe;
		self.on_reg_write8(address, value);
		let value16 = if (address & 1) == 1 {
			(iodata.direct_read16(register as usize) & 0xFF) | ((value as u16) << 8)
		} else {
			(iodata.direct_read16(register as usize) & 0xFF00) | ((value as u16))
		};
		self.on_reg_write(register, value16);
	}

	pub fn on_write16(&mut self, address: u32, value: u16) {
		self.on_reg_write(address & 0x3ff, value);
	}

	pub fn on_write32(&mut self, address: u32, value: u32) {
		self.on_reg_write(address & 0x3ff, (value & 0xFFFF) as u16);
		self.on_reg_write((address & 0x3ff) + 2, ((value >> 16) & 0xFFFF) as u16);
	}

	pub fn on_frame_end(&mut self, iodata: &[u8]) {
		// refresh BG2X, BG2Y, BG3X, BG3Y
		self.on_reg_write(0x0000028, iodata.direct_read16(0x0000028));
		self.on_reg_write(0x000002A, iodata.direct_read16(0x000002A));
		self.on_reg_write(0x000002C, iodata.direct_read16(0x000002C));
		self.on_reg_write(0x000002E, iodata.direct_read16(0x000002E));
		self.on_reg_write(0x0000038, iodata.direct_read16(0x0000038));
		self.on_reg_write(0x000003A, iodata.direct_read16(0x000003A));
		self.on_reg_write(0x000003C, iodata.direct_read16(0x000003C));
		self.on_reg_write(0x000003E, iodata.direct_read16(0x000003E));
	}

	pub fn on_reg_write8(&mut self, register: u32, value: u8) {
		match register {
			0x4000301 => {
				let low_power_bit = (value >> 7) & 1;
				if low_power_bit == 1 {
					self.stopped = true;
					self.halted = false;
					// println!(CPU NOW IN STOPPED MODE.); // #TODO remove testing code.
				} else {
					self.halted = true;
					self.stopped = false;
					// println!(CPU NOW IN HALT MODE.); // #TODO remove testing code.
				}
			},
			_ => {}
		}
	}

	pub fn on_reg_write(&mut self, register: u32, value: u16) {
		match register {
			0x0000028 => { self.bg2x = (((put_lo16!(self.bg2x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x000002A => { self.bg2x = (((put_hi16!(self.bg2x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x000002C => { self.bg2y = (((put_lo16!(self.bg2y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x000002E => { self.bg2y = (((put_hi16!(self.bg2y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x0000038 => { self.bg3x = (((put_lo16!(self.bg3x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x000003A => { self.bg3x = (((put_hi16!(self.bg3x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x000003C => { self.bg3y = (((put_lo16!(self.bg3y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x000003E => { self.bg3y = (((put_hi16!(self.bg3y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x0000100 => { self.update_timer_lo(0, value) },
			0x0000102 => { self.update_timer_hi(0, value) },
			0x0000104 => { self.update_timer_lo(1, value) },
			0x0000105 => { self.update_timer_hi(1, value) },
			0x0000108 => { self.update_timer_lo(2, value) },
			0x000010A => { self.update_timer_hi(2, value) },
			0x000010C => { self.update_timer_lo(3, value) },
			0x000010E => { self.update_timer_hi(3, value) },

			0x00000BA => { self.update_dma_hi(0, value); },
			0x00000C6 => { self.update_dma_hi(1, value); },
			0x00000D2 => { self.update_dma_hi(2, value); },
			0x00000DE => { self.update_dma_hi(3, value); },

			0x0000060 |
			0x0000062 |
			0x0000064 => { self.sound_channel1_dirty = true },

			_ => {}
		}
	}

	fn update_dma_hi(&mut self, dma_index: usize, dma_hi_data: u16) {
		let internal_reg = &mut self.dma_registers[dma_index];
		internal_reg.repeat = ((dma_hi_data >> 9) & 1) == 1;
		internal_reg.transfer_word = ((dma_hi_data >> 10) & 1) == 1;
		internal_reg.gamepak_drq = ((dma_hi_data >> 11) & 1) == 1;
		internal_reg.start_timing = (dma_hi_data >> 12) & 0x3;
		internal_reg.irq = ((dma_hi_data >> 14) & 1) == 1;
		internal_reg.enabled = ((dma_hi_data >> 15) & 1) == 1;
		internal_reg.reload = ((dma_hi_data >> 5) & 0x3) == 3;
		
		internal_reg.is_repeat = false;

		if !internal_reg.enabled {
			// if a DMA is suddenly stopped, we don't want there to be any units waiting to be transferred.
			internal_reg.units_remaining = 0;
		}

		let size: i32 = if internal_reg.transfer_word {4} else {2};

		internal_reg.dest_addr_inc = match (dma_hi_data >> 5) & 0x3 {
			0 | 3  => size as u32,
			1 => (size * -1) as u32,
			2 => 0u32,
			_ => unreachable!()
		};

		internal_reg.source_addr_inc = match (dma_hi_data >> 7) & 0x3 {
			0 | 3 => size as u32, // #FIXME 3 is actually not supported for the source_addr_inc. Maybe it should disable the DMA?
			1 => (size * -1) as u32,
			2 => 0u32,
			_ => unreachable!()
		};

		self.dma_dirty = true;
	}

	fn update_timer_lo(&mut self, t_idx: usize, lo_data: u16) {
		let timer = &mut self.timers[t_idx];
		timer.reload = lo_data as u32;
	}

	fn update_timer_hi(&mut self, t_idx: usize, hi_data: u16) {
		let timer = &mut self.timers[t_idx];
		timer.prescaler = match hi_data & 0x3 {
			0 => 0,  // 1
			1 => 6,  // 64
			2 => 8,  // 256
			3 => 10, // 1024
			_ => unreachable!()
		};

		timer.count_up = ((hi_data >> 2) & 1) == 1;
		timer.irq_enabled = ((hi_data >> 6) & 1) == 1;
		timer.operate = ((hi_data >> 7) & 1) == 1;

		// #FIXME not sure if this is suppose to happen if we enable
		//        an already enabled timer.
		timer.counter = timer.reload;
		timer.unscaled_counter = 0;
	}

	pub fn increment_timers(&mut self, amt: u32) -> u16 {
		let mut overflow_int_mask = 0;
		let mut last_timer_overflowed = false;
		for t_idx in 0..4 {
			let timer = &mut self.timers[t_idx];
			last_timer_overflowed = Self::increment_single_timer(timer, amt, last_timer_overflowed);
			if last_timer_overflowed && timer.irq_enabled {
				overflow_int_mask |= 0x08 << t_idx;
			}
		}
		return overflow_int_mask;
	}

	fn increment_single_timer(timer: &mut TimerInternalReg, amt: u32, previous_overflowed: bool) -> bool {
		if timer.operate {
			if timer.count_up {
				if previous_overflowed {
					timer.counter += 1;
				}
			} else {
				timer.unscaled_counter += amt;
				let scaled = timer.unscaled_counter >> timer.prescaler;
				timer.counter += scaled;
				timer.unscaled_counter -= scaled << timer.prescaler;
			}

			if timer.counter > 0xffff { // Ther timer has overflowed
				timer.counter = timer.reload;
				timer.unscaled_counter = 0;
				return true
			}
		}
		return false
	}
}
