pub mod ioreg;

use self::ioreg::IORegister8;
use self::ioreg::IORegister16;
use self::ioreg::IORegister32;
use self::ioreg::InternalRegisters;

pub struct MemoryRegion {
	pub start: u32,
	pub end: u32,
	pub size: usize,
	pub local_addr: usize
}

// Internal Memory
pub const MEM_BIOS:		MemoryRegion = MemoryRegion { start: 0x0, end: 0x3fff, size: 0x4000, local_addr: 0x0 };
pub const MEM_WRAM_B:	MemoryRegion = MemoryRegion { start: 0x02000000, end: 0x0203ffff, size: 0x40000, local_addr: MEM_BIOS.local_addr + MEM_BIOS.size };
pub const MEM_WRAM_C:	MemoryRegion = MemoryRegion { start: 0x03000000, end: 0x03007fff, size: 0x8000, local_addr: MEM_WRAM_B.local_addr + MEM_WRAM_B.size };
pub const MEM_IOREG:	MemoryRegion = MemoryRegion { start: 0x04000000, end: 0x040003fe, size: 0x805, local_addr: MEM_WRAM_C.local_addr + MEM_WRAM_C.size };

// Internal Video Memory
pub const MEM_PAL:		MemoryRegion = MemoryRegion { start: 0x05000000, end: 0x050003ff, size: 0x400, local_addr: MEM_IOREG.local_addr + MEM_IOREG.size };
pub const MEM_VRAM:		MemoryRegion = MemoryRegion { start: 0x06000000, end: 0x06017fff, size: 0x18000, local_addr: MEM_PAL.local_addr + MEM_PAL.size };
pub const MEM_OAM:		MemoryRegion = MemoryRegion { start: 0x07000000, end: 0x070003ff, size: 0x400, local_addr: MEM_VRAM.local_addr + MEM_VRAM.size };

const INTERNAL_MEM_SIZE: usize	= MEM_OAM.local_addr + MEM_OAM.size;

// External Memory
pub const MEM_ROM0:		MemoryRegion = MemoryRegion { start: 0x08000000, end: 0x09ffffff, size: 0x2000000, local_addr: 0 };
pub const MEM_ROM1:		MemoryRegion = MemoryRegion { start: 0x0a000000, end: 0x0Bffffff, size: 0x2000000, local_addr: 0 };
pub const MEM_ROM2:		MemoryRegion = MemoryRegion { start: 0x0c000000, end: 0x0Dffffff, size: 0x2000000, local_addr: 0 };
pub const MEM_SRAM:		MemoryRegion = MemoryRegion { start: 0x0e000000, end: 0x0E00ffff, size: 0x10000, local_addr: 0 };

pub struct GbaMemory {
	pub internal_data: [u8; INTERNAL_MEM_SIZE],
	pub internal_regs: InternalRegisters,
	pub rom: Vec<u8>
	// #TODO add SRAM.
}

// trait MemoryBoundCheckable {
// 	fn within(self, start: u32, end: u32) -> bool;
// }

// impl MemoryBoundCheckable for u32 {
// 	#[inline(always)]
// 	fn within(self, start: u32, end: u32) -> bool {
// 		self >= start && self <= end
// 	}
// }

impl GbaMemory {
	pub fn new() -> GbaMemory {
		GbaMemory {
			internal_data: [0u8; INTERNAL_MEM_SIZE],
			internal_regs: InternalRegisters::new(),
			rom: vec![]
		}
	}

	// #TODO handle rom.
	pub fn get_region_mut(&mut self, region: MemoryRegion) -> &mut [u8] {
		&mut self.internal_data[region.local_addr..(region.local_addr+region.size)]
	}


	/// Returns a slice of the memory in the range [start_address, end_address).
	/// This should never try to slice memory across regions or mirrors. Bad things might happen.
	/// NOTE: Ranges in this are inclusive!
	pub fn get_slice(&self, start_address: u32, end_address: u32) -> &[u8] {
		let (start_index, _) = self.transform(start_address);
		let (end_index, _) = self.transform(end_address);
		&self.internal_data[start_index..(end_index + 1)]
	}

	// #TODO handle rom.
	pub fn get_region(&self, region: MemoryRegion) -> &[u8] {
		&self.internal_data[region.local_addr..(region.local_addr+region.size)]
	}

	/// Transforms a global GBA memory address
	/// into a local index in our internal data array.
	/// returns address and true if write is allowed.
	pub fn transform(&self, address32: u32) -> (usize, bool) {
		let address = address32 as usize;
		match address {
			// System ROM (BIOS):
			// Start: 0x00000000
			// End:  0x0003FFF
			// Size: 16kb 
			// Port Size: 32 bit
			// Wait State: 0
			0x0 ... 0x3fff => (address + MEM_BIOS.local_addr, false),

			// External Work RAM (On-Board):
			// Start: 0x02000000
			// End:   0x0203FFFF
			// Size:  256kb
			// Port Size: 16 bit
			// Mirrors:  Every 0x40000 bytes from 0x02000000 to 0x02FFFFFF
			0x02000000 ... 0x02FFFFFF => ((address % 0x40000) + MEM_WRAM_B.local_addr, true),

			// Internal Work RAM (On-Chip):
			// Start: 0x03000000
			// End:   0x03007FFF
			// Size:  32kb
			// Port Size: 32 bit
			// Mirrors:  Every 0x8000 bytes from 0x03000000 to 0x03FFFFFF
			0x03000000 ... 0x03FFFFFF => ((address % 0x8000) + MEM_WRAM_C.local_addr, true),

			0x04000000 ... 0x04FFFFFF => {
				let _addrmasked = address & 0xFFFF;
				if _addrmasked >= 0x0800  && _addrmasked <= 0x0803 { (MEM_IOREG.local_addr + _addrmasked, true) }
				else if address <= 0x04000804 { (address - 0x04000000 + MEM_IOREG.local_addr, true) }
				else { panic!("Invalid IO register address. Not sure how to handle this! {:08x}", address) }
			}

			// Palette RAM:
			// Start: 0x05000000
			// End:   0x050003FF
			// Size:  1kb
			// Port Size:  16 bit
			// Mirrors: Every 0x400 bytes from 0x05000000 to 0x5FFFFFF
			0x05000000 ... 0x5FFFFFF => ((address % 0x400) + MEM_PAL.local_addr, true),

			// VRAM:
			// Start: 0x06000000
			// End:   0x06017FFF
			// Size:  96kb
			// Port Size: 16 bit
			// Mirrors: Bytes 0x06010000 - 0x06017FFF is mirrored from 0x06018000 - 0x0601FFFF.
			//          The entire region from 0x06000000 - 0x06020000 is in turn mirrored every
			//          0x20000 bytes from 0x06000000 - 0x06FFFFFF
			// Ignore the above mirrors documentation, it's confusing
			// MIRRORS: Even though VRAM is sized 96K (64K+32K), 
			// it is repeated in steps of 128K (64K+32K+32K, the two 32K blocks itself being mirrors of each other)
			0x06000000 ... 0x06FFFFFF => {
				let vram_128k = address % 0x20000; // 0x20000 = 128K (using 1024 not that metric 1000 crap.)
				let vram_offset = if vram_128k >= 0x18000 {
					// If this is over 96K (it goes into the mirrored 32K area)
					vram_128k - 0x8000 // Subtract 32K
				} else {
					vram_128k
				};
				(vram_offset + MEM_VRAM.local_addr, true)
			}

			// OAM:
			// Start: 0x07000000
			// End:   0x070003FF
			// Size:  1kb
			// Port Size: 32 bit
			// Mirrors: Every 0x400 bytes from 0x07000000 to 0x07FFFFFF
			0x07000000 ... 0x07FFFFFF => ((address % 0x400) + MEM_OAM.local_addr, true),

			// #TODO Reading from Unused Memory (00004000-01FFFFFF,10000000-FFFFFFFF)
			_ => (0, false)
		}
	}

	pub fn read8(&self, address: u32) -> u8 {
		self.__read8__(address)
	}

	pub fn write8(&mut self, address: u32, value: u8) {
		self.__write8__(address, value);
		if address >= 0x04000000 && address <= 0x4000803 {
			self.internal_regs.on_write8(address, value, &self.internal_data[MEM_IOREG.local_addr..(MEM_IOREG.local_addr+MEM_IOREG.size)]);
		}
	}

	pub fn read16(&self, address: u32) -> u16 {
		self.__read8__(address) as u16 | 
		((self.__read8__(address + 1) as u16) << 8)
	}

	pub fn write16(&mut self, address: u32, value: u16) {
		self.__write8__(address, (value & 0xff) as u8);
		self.__write8__(address + 1, ((value >> 8) & 0xff) as u8);
		if address >= 0x04000000 && address <= 0x4000803 {
			self.internal_regs.on_write16(address, value);
		}
	}

	pub fn read32(&self, address: u32) -> u32 {
		self.__read8__(address) as u32 | 
		((self.__read8__(address + 1) as u32) << 8) |
		((self.__read8__(address + 2) as u32) << 16) |
		((self.__read8__(address + 3) as u32) << 24)
	}

	pub fn write32(&mut self, address: u32, value: u32) {
		self.__write8__(address, (value & 0xff) as u8);
		self.__write8__(address + 1, ((value >> 8) & 0xff) as u8);
		self.__write8__(address + 2, ((value >> 16) & 0xff) as u8);
		self.__write8__(address + 3, ((value >> 24) & 0xff) as u8);
		if address >= 0x04000000 && address <= 0x4000803 {
			self.internal_regs.on_write32(address, value);
		}
	}

	pub fn direct_read8(&self, index: usize) -> u8 {
		self.internal_data[index]
	}

	pub fn direct_read16(&self, index: usize) -> u16 {
		self.direct_read8(index) as u16 | 
		((self.direct_read8(index + 1) as u16) << 8)
	}

	pub fn direct_read32(&self, index: usize) -> u32 {
		self.direct_read8(index) as u32 | 
		((self.direct_read8(index + 1) as u32) << 8) |
		((self.direct_read8(index + 2) as u32) << 16) |
		((self.direct_read8(index + 3) as u32) << 24)
	}

	pub fn direct_write8(&mut self, index: usize, value: u8) {
		self.internal_data[index] = value;
	}

	pub fn direct_write16(&mut self, index: usize, value: u16) {
		self.direct_write8(index, (value & 0xff) as u8);
		self.direct_write8(index + 1, ((value >> 8) & 0xff) as u8);
	}

	pub fn direct_write32(&mut self, index: usize, value: u32) {
		self.direct_write8(index, (value & 0xff) as u8);
		self.direct_write8(index + 1, ((value >> 8) & 0xff) as u8);
		self.direct_write8(index + 2, ((value >> 16) & 0xff) as u8);
		self.direct_write8(index + 3, ((value >> 24) & 0xff) as u8);	
	}

	#[inline]
	fn __write8__(&mut self, address: u32, value: u8) {
		match address {
			0x08000000 ... 0x0Dffffff => self.rom_write8(address, value),
			0x0E000000 ... 0x0E00FFFF => {}, // #TODO SRAM and shit.
			_ => {
				let (local_addr, writeable) = self.transform(address);
				if writeable {
					match address {
						// Handles the weirdness of writing to the IF register.
						// When one of the bits is set, it's actually cleared.
						// #FIXME take a look at this again at some point.
						//        because I'm not sure if this is exactly what
						//        is supposed to be happening.
						0x4000202 | 0x4000203 => self.internal_data[local_addr] &= !value,
						_ => self.internal_data[local_addr] = value
					}
				}
			}	
		}
	}

	#[inline]
	fn __read8__(&self, address: u32) -> u8 {
		match address {
			0x08000000 ... 0x0Dffffff => self.rom_read8(address),
			0x0E000000 ... 0x0E00FFFF => 0, // #TODO SRAM and shit.
			_ => {
				let (local_addr, _) = self.transform(address);
				self.internal_data[local_addr]
			}
		}
	}

	#[inline]
	fn rom_write8(&mut self, address: u32, value: u8) {
		let local_addr = (address - (address & 0x0f000000)) as usize;
		if local_addr < self.rom.len() { self.rom[local_addr] = value }
	}

	#[inline]
	fn rom_read8(&self, address: u32) -> u8 {
		let local_addr = (address - (address & 0x0f000000)) as usize;
		if local_addr >= self.rom.len() { 0 }
		else { self.rom[local_addr] }
	}
}

pub trait ReadIOReg<R> {
	type RegSizeType: Sized;
	fn get_reg(&self, reg: R) -> Self::RegSizeType;
	fn set_reg(&mut self, reg: R, value: Self::RegSizeType);
}

impl ReadIOReg<IORegister8> for GbaMemory {
	type RegSizeType = u8;

	fn get_reg(&self, reg: IORegister8) -> u8 { return self.direct_read8(reg.0 + MEM_IOREG.local_addr) }
	fn set_reg(&mut self, reg: IORegister8, value: u8) { self.direct_write8(reg.0 + MEM_IOREG.local_addr, value); }
}


impl ReadIOReg<IORegister16> for GbaMemory {
	type RegSizeType = u16;

	fn get_reg(&self, reg: IORegister16) -> u16 { return self.direct_read16(reg.0 + MEM_IOREG.local_addr) }
	fn set_reg(&mut self, reg: IORegister16, value: u16) { self.direct_write16(reg.0 + MEM_IOREG.local_addr, value); }
}

impl ReadIOReg<IORegister32> for GbaMemory {
	type RegSizeType = u32;

	fn get_reg(&self, reg: IORegister32) -> u32 { return self.direct_read32(reg.0 + MEM_IOREG.local_addr) }
	fn set_reg(&mut self, reg: IORegister32, value: u32) { self.direct_write32(reg.0 + MEM_IOREG.local_addr, value); }
}

pub trait GbaMemoryReadAccess {
	fn direct_read8(&self, index: usize) -> u8;

	fn direct_read16(&self, index: usize) -> u16 {
		self.direct_read8(index) as u16 | 
		((self.direct_read8(index + 1) as u16) << 8)
	}

	fn direct_read32(&self, index: usize) -> u32 {
		self.direct_read8(index) as u32 | 
		((self.direct_read8(index + 1) as u32) << 8) |
		((self.direct_read8(index + 2) as u32) << 16) |
		((self.direct_read8(index + 3) as u32) << 24)
	}
}



pub trait GbaMemoryWriteAccess {
	fn direct_write8(&mut self, index: usize, value: u8);

	fn direct_write16(&mut self, index: usize, value: u16) {
		self.direct_write8(index, (value & 0xff) as u8);
		self.direct_write8(index + 1, ((value >> 8) & 0xff) as u8);
	}

	fn direct_write32(&mut self, index: usize, value: u32) {
		self.direct_write8(index, (value & 0xff) as u8);
		self.direct_write8(index + 1, ((value >> 8) & 0xff) as u8);
		self.direct_write8(index + 2, ((value >> 16) & 0xff) as u8);
		self.direct_write8(index + 3, ((value >> 24) & 0xff) as u8);	
	}
}

impl<'a> GbaMemoryReadAccess for &'a [u8] {
	fn direct_read8(&self, index: usize) -> u8 { self[index] }
}

impl<'a> GbaMemoryReadAccess for &'a mut [u8] {
	fn direct_read8(&self, index: usize) -> u8 { self[index] }
}

impl<'a> GbaMemoryWriteAccess for &'a mut [u8] {
	fn direct_write8(&mut self, index: usize, value: u8) { self[index] = value; }
}

