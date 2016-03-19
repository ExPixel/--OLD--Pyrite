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

// Internal IO registers.
#[derive(Default)]
pub struct InternalRegisters {
	pub bg2x: u32,
	pub bg2y: u32,
	pub bg3x: u32,
	pub bg3y: u32,

	pub halted: bool,
	pub stopped: bool,

	pub dma_dirty: bool
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
					// println!("CPU NOW IN STOPPED MODE."); // #TODO remove testing code.
				} else {
					self.halted = true;
					self.stopped = false;
					// println!("CPU NOW IN HALT MODE."); // #TODO remove testing code.
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

			0x00000BA | 0x00000C6 | 
			0x00000D2 | 0x00000DE => { self.dma_dirty = true },

			_ => {}
		}
	}
}

/*
pub const DMA0SAD: IORegister32 = IORegister32(0x00000b0);
pub const DMA0DAD: IORegister32 = IORegister32(0x00000b4);
pub const DMA1SAD: IORegister32 = IORegister32(0x00000bc);
pub const DMA1DAD: IORegister32 = IORegister32(0x00000c0);
pub const DMA2SAD: IORegister32 = IORegister32(0x00000c8);
pub const DMA2DAD: IORegister32 = IORegister32(0x00000cc);
pub const DMA3SAD: IORegister32 = IORegister32(0x00000d4);
pub const DMA3DAD: IORegister32 = IORegister32(0x00000d8);
*/

/*
pub const BG2X: IORegister32 = IORegister32(0x0000028);
pub const BG2Y: IORegister32 = IORegister32(0x000002c);
pub const BG3X: IORegister32 = IORegister32(0x0000038);
pub const BG3Y: IORegister32 = IORegister32(0x000003c);
*/