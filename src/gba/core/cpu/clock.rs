
#[derive(RustcEncodable, RustcDecodable)]
pub struct ArmCpuClock {
	/// The number of cycles that have gone by.
	pub cycles: u64,
	pub audio_clock: u32,
	pub timer_cycles: u32,
	pub memory_timings: [((u8, u8, u8), (u8, u8, u8)); 15]
}

#[allow(unused_variables)]
impl ArmCpuClock {
	pub fn new() -> ArmCpuClock {
		let mut clock = ArmCpuClock {
			cycles: 0,

			timer_cycles: 0,

			audio_clock: 0,

			// Format (S, N)
			// S/N = (8bits, 16bits, 32bits)
			memory_timings: [
				// Internal Memory Timings (Static)
				((1, 1, 1), (1, 1, 1)),     // 0x0. BIOS
				((0, 0, 0), (0, 0, 0)),     // 0x1. Unused
				((3, 3, 6), (3, 3, 6)),     // 0x2. Work RAM 258K
				((1, 1, 1), (1, 1, 1)),     // 0x3. Work RAM 32K
				((1, 1, 1), (1, 1, 1)),     // 0x4. I/O
				((1, 1, 2), (1, 1, 2)),     // 0x5. Palette RAM
				((1, 1, 1), (1, 1, 1)),     // 0x6. VRAM
				((1, 1, 1), (1, 1, 1)),     // 0x7. OAM

				// Dynamic Memory Timings (Dynamic)
				((0, 0, 0), (0, 0, 0)),     // 0x8. Game Pak ROM - Wait State 0
				((0, 0, 0), (0, 0, 0)),     // 0x9

				((0, 0, 0), (0, 0, 0)),     // 0xA Game Pak ROM - Wait State 1
				((0, 0, 0), (0, 0, 0)),     // 0xB

				((0, 0, 0), (0, 0, 0)),     // 0xC Game Pak ROM - Wait State 2
				((0, 0, 0), (0, 0, 0)),     // 0xD

				((0, 0, 0), (0, 0, 0))      // 0xE Game Pak SRAM
			]
		};
		clock.setup_default_timings();
		clock
	}

	pub fn setup_default_timings(&mut self) {
		self.memory_timings[0x8] = ((3, 3, 4), (5, 5, 8));
		self.memory_timings[0x9] = self.memory_timings[0x8];

		self.memory_timings[0xA] = ((5, 5, 8), (5, 5, 8));
		self.memory_timings[0xB] = self.memory_timings[0xA];

		self.memory_timings[0xC] = ((9, 9, 16), (5, 5, 8));
		self.memory_timings[0xD] = self.memory_timings[0xC];

		self.memory_timings[0xE] = ((5, 5, 5), (5, 5, 5));
	}

	/// Internal cycle
	pub fn internal(&mut self, cycles: u64) {
		self.cycles += cycles;
		self.timer_cycles += cycles as u32;
		self.audio_clock += cycles as u32;
	}

	/// Sequential 8bit data access
	pub fn data_access8_seq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_seq_cycles8(addr) as u64;
		self.internal(cycles);
	}

	/// Sequential 16 bit data access
	pub fn data_access16_seq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_seq_cycles16(addr) as u64;
		self.internal(cycles);
	}

	/// Sequential 32 bit data access
	pub fn data_access32_seq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_seq_cycles32(addr) as u64;
		self.internal(cycles);
	}

	/// Nonsequential 8bit data access
	pub fn data_access8_nonseq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_nonseq_cycles8(addr) as u64;
		self.internal(cycles);
	}

	/// Nonsequential 16 bit data access
	pub fn data_access16_nonseq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_nonseq_cycles16(addr) as u64;
		self.internal(cycles);
	}

	/// Nonsequential 32 bit data access
	pub fn data_access32_nonseq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_nonseq_cycles32(addr) as u64;
		self.internal(cycles);
	}

	pub fn code_access32_seq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_seq_cycles32(addr) as u64;
		self.internal(cycles);
	}

	pub fn code_access32_nonseq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_nonseq_cycles32(addr) as u64;
		self.internal(cycles);
	}

	pub fn code_access16_seq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_seq_cycles16(addr) as u64;
		self.internal(cycles);
	}


	pub fn code_access16_nonseq(&mut self, addr: u32) {
		// #TODO implement the prefetch stuff so that ROM isn't slow as fuck.
		let cycles = self.get_nonseq_cycles16(addr) as u64;
		self.internal(cycles);
	}

	pub fn get_seq_cycles8(&self, address: u32) -> u8 {
		let area = ((address >> 24) & 0xff) as usize;
		if area > 0x0E { 1 }
		else { (self.memory_timings[area].0).0 }
	}

	pub fn get_nonseq_cycles8(&self, address: u32) -> u8 {
		let area = ((address >> 24) & 0xff) as usize;
		if area > 0x0E { 1 }
		else { (self.memory_timings[area].1).0 }
	}

	pub fn get_seq_cycles16(&self, address: u32) -> u8 {
		let area = ((address >> 24) & 0xff) as usize;
		if area > 0x0E { 1 }
		else { (self.memory_timings[area].0).1 }
	}

	pub fn get_nonseq_cycles16(&self, address: u32) -> u8 {
		let area = ((address >> 24) & 0xff) as usize;
		if area > 0x0E { 1 }
		else { (self.memory_timings[area].1).1 }
	}

	pub fn get_seq_cycles32(&self, address: u32) -> u8 {
		let area = ((address >> 24) & 0xff) as usize;
		if area > 0x0E { 1 }
		else { (self.memory_timings[area].0).2 }
	}

	pub fn get_nonseq_cycles32(&self, address: u32) -> u8 {
		let area = ((address >> 24) & 0xff) as usize;
		if area > 0x0E { 1 }
		else { (self.memory_timings[area].1).2 }
	}
}

// Address Bus Width and CPU Read/Write Access Widths
// Shows the Bus-Width, supported read and write widths, and the clock cycles for 8/16/32bit accesses.
//   Region        Bus   Read      Write     Cycles
//   BIOS ROM      32    8/16/32   -         1/1/1
//   Work RAM 32K  32    8/16/32   8/16/32   1/1/1
//   I/O           32    8/16/32   8/16/32   1/1/1
//   OAM           32    8/16/32   16/32     1/1/1 *
//   Work RAM 256K 16    8/16/32   8/16/32   3/3/6 **
//   Palette RAM   16    8/16/32   16/32     1/1/2 *
//   VRAM          16    8/16/32   16/32     1/1/2 *

// Timing Notes:
//   *   Plus 1 cycle if GBA accesses video memory at the same time.
//   **  Default waitstate settings, see System Control chapter.
//   *** Separate timings for sequential, and non-sequential accesses.
//   One cycle equals approx. 59.59ns (ie. 16.78MHz clock).
// All memory (except GamePak SRAM) can be accessed by 16bit and 32bit DMA.
