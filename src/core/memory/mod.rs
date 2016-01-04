pub mod ioreg;

use self::ioreg::IORegisters;

pub struct MemoryRegion {
	pub start: u32,
	pub end: u32,
	pub size: usize,
	local_addr: usize
}

const INTERNAL_MEM_SIZE: usize	= 0x64bff;

// Internal Memory
pub const MEM_BIOS:		MemoryRegion = MemoryRegion { start: 0x0, end: 0x3fff, size: 0x4000, local_addr: 0x0 };
pub const MEM_WRAM_B:	MemoryRegion = MemoryRegion { start: 0x02000000, end: 0x0203ffff, size: 0x40000, local_addr: 0x4000 };
pub const MEM_WRAM_C:	MemoryRegion = MemoryRegion { start: 0x03000000, end: 0x03007fff, size: 0x8000, local_addr: 0x44000 };
pub const MEM_IOREG:	MemoryRegion = MemoryRegion { start: 0x04000000, end: 0x040003fe, size: 0x3ff, local_addr: 0x4c000 };

// Internal Video Memory
pub const MEM_PAL:		MemoryRegion = MemoryRegion { start: 0x05000000, end: 0x050003ff, size: 0x400, local_addr: 0x4c3ff };
pub const MEM_VRAM:		MemoryRegion = MemoryRegion { start: 0x06000000, end: 0x06017fff, size: 0x18000, local_addr: 0x4c7ff };
pub const MEM_OAM:		MemoryRegion = MemoryRegion { start: 0x07000000, end: 0x070003ff, size: 0x400, local_addr: 0x647ff };

// External Memory
pub const MEM_ROM0:		MemoryRegion = MemoryRegion { start: 0x08000000, end: 0x09ffffff, size: 0x2000000, local_addr: 0 };
pub const MEM_ROM1:		MemoryRegion = MemoryRegion { start: 0x0a000000, end: 0x0Bffffff, size: 0x2000000, local_addr: 0 };
pub const MEM_ROM2:		MemoryRegion = MemoryRegion { start: 0x0c000000, end: 0x0Dffffff, size: 0x2000000, local_addr: 0 };
pub const MEM_SRAM:		MemoryRegion = MemoryRegion { start: 0x0e000000, end: 0x0E00ffff, size: 0x10000, local_addr: 0 };

pub struct GbaMemory {
	internal_data: [u8; INTERNAL_MEM_SIZE],
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
			rom: vec![]
		}
	}

	/// Transforms a global GBA memory address
	/// into a local address in our internal data array.
	fn transform(&self, address32: u32) -> usize {
		let address = address32 as usize;
		match address {
			// System ROM (BIOS):
			// Start: 0x00000000
			// End:  0x0003FFF
			// Size: 16kb 
			// Port Size: 32 bit
			// Wait State: 0
			0x0 ... 0x3fff => address + MEM_BIOS.local_addr,

			// External Work RAM (On-Board):
			// Start: 0x02000000
			// End:   0x0203FFFF
			// Size:  256kb
			// Port Size: 16 bit
			// Mirrors:  Every 0x40000 bytes from 0x02000000 to 0x02FFFFFF
			0x02000000 ... 0x02FFFFFF => (address % 0x40000) + MEM_WRAM_B.local_addr,

			// Internal Work RAM (On-Chip):
			// Start: 0x03000000
			// End:   0x03007FFF
			// Size:  32kb
			// Port Size: 32 bit
			// Mirrors:  Every 0x8000 bytes from 0x03000000 to 0x03FFFFFF
			0x03000000 ... 0x03FFFFFF => (address % 0x8000) + MEM_WRAM_C.local_addr,

			// Palette RAM:
			// Start: 0x05000000
			// End:   0x050003FF
			// Size:  1kb
			// Port Size:  16 bit
			// Mirrors: Every 0x400 bytes from 0x05000000 to 0x5FFFFFF
			0x05000000 ... 0x5FFFFFF => (address % 0x400) + MEM_PAL.local_addr,

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
				let vram_offset = if vram_128k >= 0x6000000 {
					// If this is over 96K (it goes into the mirrored 32K area)
					vram_128k - 0x8000 // Subtract 32K
				} else {
					vram_128k
				};
				vram_offset + MEM_VRAM.local_addr
			}

			// OAM:
			// Start: 0x07000000
			// End:   0x070003FF
			// Size:  1kb
			// Port Size: 32 bit
			// Mirrors: Every 0x400 bytes from 0x07000000 to 0x07FFFFFF
			0x07000000 ... 0x07FFFFFF => (address % 0x400) + MEM_OAM.local_addr,
			_ => 0
		}
	}

	pub fn read8(&self, address: u32) -> u8 {
		self.__read8__(address)
	}

	pub fn write8(&mut self, address: u32, value: u8) {
		self.__write8__(address, value)
	}

	pub fn read16(&self, address: u32) -> u16 {
		self.__read8__(address) as u16 | 
		((self.__read8__(address + 1) as u16) << 8)
	}

	pub fn write16(&mut self, address: u32, value: u16) {
		self.__write8__(address, (value & 0xff) as u8);
		self.__write8__(address + 1, ((value >> 8) & 0xff) as u8);
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
	}

	#[inline]
	fn __write8__(&mut self, address: u32, value: u8) {
		match address {
			0x08000000 ... 0x0Dffffff => self.rom_write8(address, value),
			_ => {
				let local_addr = self.transform(address);
				self.internal_data[local_addr] = value;
			}	
		}
	}

	#[inline]
	fn __read8__(&self, address: u32) -> u8 {
		match address {
			0x08000000 ... 0x0Dffffff => self.rom_read8(address),
			_ => {
				let local_addr = self.transform(address);
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
