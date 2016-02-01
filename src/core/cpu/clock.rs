pub struct ArmCpuClock {
	/// The number of cycles that have gone by.
	pub cycles: u64
}

#[allow(unused_variables)]
impl ArmCpuClock {
	pub fn new() -> ArmCpuClock {
		ArmCpuClock {
			cycles: 0
		}
	}
	/// Internal cycle
	pub fn internal(&mut self, cycles: u64) {
		self.cycles += cycles;
	}

	/// Sequential 8bit data access
	pub fn data_access8_seq(&mut self, addr: u32) {
		self.internal(1); // #TODO make this right.
	}

	/// Sequential 16 bit data access
	pub fn data_access16_seq(&mut self, addr: u32) {
		self.internal(1); // #TODO make this right.
	}

	/// Sequential 32 bit data access
	pub fn data_access32_seq(&mut self, addr: u32) {
		self.internal(1); // #TODO make this right.
	}

	/// Nonsequential 8bit data access
	pub fn data_access8_nonseq(&mut self, addr: u32) {
		self.internal(1); // #TODO make this right.
	}

	/// Nonsequential 16 bit data access
	pub fn data_access16_nonseq(&mut self, addr: u32) {
		self.internal(1); // #TODO make this right.
	}

	/// Nonsequential 32 bit data access
	pub fn data_access32_nonseq(&mut self, addr: u32) {
		self.internal(1); // #TODO make this right.
	}

	pub fn code_access32_seq(&mut self, addr: u32) {
		self.internal(1);
	}


	pub fn code_access32_nonseq(&mut self, addr: u32) {
		self.internal(1);
	}

	pub fn code_access16_seq(&mut self, addr: u32) {
		self.internal(1);
	}


	pub fn code_access16_nonseq(&mut self, addr: u32) {
		self.internal(1);
	}
}

// Instruction Cycle Summary
// 
//   Instruction      Cycles      Additional
//   ---------------------------------------------------------------------
//   ALU              1S          +1S+1N if R15 loaded, +1I if SHIFT(Rs)
//   MSR,MRS          1S
//   LDR              1S+1N+1I    +1S+1N if R15 loaded
//   STR              2N
//   LDM              nS+1N+1I    +1S+1N if R15 loaded
//   STM              (n-1)S+2N
//   SWP              1S+2N+1I
//   BL (THUMB)       3S+1N
//   B,BL             2S+1N
//   SWI,trap         2S+1N
//   MUL              1S+ml
//   MLA              1S+(m+1)I
//   MULL             1S+(m+1)I
//   MLAL             1S+(m+2)I
//   CDP              1S+bI
//   LDC,STC          (n-1)S+2N+bI
//   MCR              1N+bI+1C
//   MRC              1S+(b+1)I+1C
//   {cond} false     1S
// 
// 
// ARM9:
//   Q{D}ADD/SUB      1S+Interlock.
//   CLZ              1S.
//   LDR              1S+1N+1L
//   LDRB,LDRH,LDRmis 1S+1N+2L
//   LDR PC ...
//   STR              1S+1N        (not 2N, and both in parallel)
// Execution Time: 1S+Interlock (SMULxy,SMLAxy,SMULWx,SMLAWx)
// Execution Time: 1S+1I+Interlock (SMLALxy)
// 
// 
// Whereas,
//   n = number of words transferred
//   b = number of cycles spent in coprocessor busy-wait loop
//   m = depends on most significant byte(s) of multiplier operand
// Above 'trap' is meant to be the execution time for exceptions. 
// And '{cond} false' is meant to be the execution time for conditional instructions 
// which haven't been actually executed because the condition has been false.
// 
// The separate meaning of the N,S,I,C cycles is:
// 
// N - Non-sequential cycle
// Requests a transfer to/from an address which is NOT related to the address used in the previous cycle. 
// (Called 1st Access in GBA language).
// The execution time for 1N is 1 clock cycle (plus non-sequential access waitstates).
// 
// S - Sequential cycle
// Requests a transfer to/from an address which is located directly after the address used in the previous cycle. 
// Ie. for 16bit or 32bit accesses at incrementing addresses, the first access is Non-sequential, 
// the following accesses are sequential. (Called 2nd Access in GBA language).
// The execution time for 1S is 1 clock cycle (plus sequential access waitstates).
// 
// I - Internal Cycle
// CPU is just too busy, not even requesting a memory transfer for now.
// The execution time for 1I is 1 clock cycle (without any waitstates).
// 
// C - Coprocessor Cycle
// The CPU uses the data bus to communicate with the coprocessor (if any), but no memory transfers are requested.
// 
// Memory Waitstates
// Ideally, memory may be accessed free of waitstates (1N and 1S are then equal to 1 clock cycle each). 
// However, a memory system may generate waitstates for several reasons: The memory may be just too slow. 
// Memory is currently accessed by DMA, eg. sound, video, 
// memory transfers, etc. Or when data is squeezed through a 16bit data bus (in that special case, 
// 32bit access may have more waitstates than 8bit and 16bit accesses). 
// Also, the memory system may separate between S and N cycles 
// (if so, S cycles would be typically faster than N cycles).
// 
// Memory Waitstates for Different Memory Areas
// Different memory areas (eg. ROM and RAM) may have different waitstates. 
// When executing code in one area which accesses data in another area, 
// then the S+N cycles must be split into code and data accesses: 1N is used for data access, 
// plus (n-1)S for LDM/STM, the remaining S+N are code access. If an instruction jumps to a different memory area, 
// then all code cycles for that opcode are having waitstate characteristics of the 
// NEW memory area (except Thumb BL which still executes 1S in OLD area).

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
//   GamePak ROM   16    8/16/32   -         5/5/8 **/***
//   GamePak Flash 16    8/16/32   16/32     5/5/8 **/***
//   GamePak SRAM  8     8         8         5     **
// Timing Notes:
//   *   Plus 1 cycle if GBA accesses video memory at the same time.
//   **  Default waitstate settings, see System Control chapter.
//   *** Separate timings for sequential, and non-sequential accesses.
//   One cycle equals approx. 59.59ns (ie. 16.78MHz clock).
// All memory (except GamePak SRAM) can be accessed by 16bit and 32bit DMA.
