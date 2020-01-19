
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

	/// Sets up timings based on waitstates.
	/// ```
	/// 4000204h - WAITCNT - Waitstate Control (R/W)
	/// This register is used to configure game pak access timings. The game pak ROM is mirrored to three address regions at 08000000h, 0A000000h, and 0C000000h, these areas are called Wait State 0-2. Different access timings may be assigned to each area (this might be useful in case that a game pak contains several ROM chips with different access times each).
	///   Bit   Expl.
	///   0-1   SRAM Wait Control          (0..3 = 4,3,2,8 cycles)
	///   2-3   Wait State 0 First Access  (0..3 = 4,3,2,8 cycles)
	///   4     Wait State 0 Second Access (0..1 = 2,1 cycles)
	///   5-6   Wait State 1 First Access  (0..3 = 4,3,2,8 cycles)
	///   7     Wait State 1 Second Access (0..1 = 4,1 cycles; unlike above WS0)
	///   8-9   Wait State 2 First Access  (0..3 = 4,3,2,8 cycles)
	///   10    Wait State 2 Second Access (0..1 = 8,1 cycles; unlike above WS0,WS1)
	///   11-12 PHI Terminal Output        (0..3 = Disable, 4.19MHz, 8.38MHz, 16.78MHz)
	///   13    Not used
	///   14    Game Pak Prefetch Buffer (Pipe) (0=Disable, 1=Enable)
	///   15    Game Pak Type Flag  (Read Only) (0=GBA, 1=CGB) (IN35 signal)
	///   16-31 Not used
	/// ```
	pub fn setup_timings(&mut self, waitcnt: u16) {
		fn fa_timing(x: u16) -> u8 {
			const FIRST_ACCESS_TIMINGS: [u8; 4] = [4, 3, 2, 8];
			FIRST_ACCESS_TIMINGS[x as usize]
		}

		let sram_timing = fa_timing(waitcnt & 0x3) + 1;
		let sram_timing_tup = (sram_timing, sram_timing, sram_timing);
		self.memory_timings[0xE] = (sram_timing_tup, sram_timing_tup);

		let waitstate0_nseq = fa_timing((waitcnt >> 2) & 0x3);
		let waitstate0_seq = if (waitcnt & 0x10) != 0 { 1 } else { 2 };
		self.memory_timings[0x8] = (
			(waitstate0_seq + 1, waitstate0_seq + 1, (waitstate0_seq * 2)),
			(waitstate0_nseq + 1, waitstate0_nseq + 1, (waitstate0_nseq + 1) + (waitstate0_seq + 1))
		);

		let waitstate1_nseq = fa_timing((waitcnt >> 5) & 0x3);
		let waitstate1_seq = if (waitcnt & 0x80) != 0 { 1 } else { 4 };
		self.memory_timings[0xA] = (
			(waitstate1_seq + 1, waitstate1_seq + 1, (waitstate1_seq * 2)),
			(waitstate1_nseq + 1, waitstate1_nseq + 1, (waitstate1_nseq + 1) + (waitstate1_seq + 1))
		);

		let waitstate2_nseq = fa_timing((waitcnt >> 8) & 0x3);
		let waitstate2_seq = if (waitcnt & 0x400) != 0 { 1 } else { 8 };
		self.memory_timings[0xC] = (
			(waitstate2_seq + 1, waitstate2_seq + 1, (waitstate2_seq * 2)),
			(waitstate2_nseq + 1, waitstate2_nseq + 1, (waitstate2_nseq + 1) + (waitstate2_seq + 1))
		);

		self.memory_timings[0x9] = self.memory_timings[0x8];
		self.memory_timings[0xB] = self.memory_timings[0xA];
		self.memory_timings[0xD] = self.memory_timings[0xC];
	}

	pub fn setup_default_timings(&mut self) {
		// #TODO this is the default setting but I should be reading the value somehow instead.
		self.setup_timings(0x4317);
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
