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
	pub fn daccess8_seq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Sequential 16 bit data access
	pub fn daccess16_seq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Sequential 32 bit data access
	pub fn daccess32_seq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Nonsequential 8bit data access
	pub fn daccess8_nonseq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Nonsequential 16 bit data access
	pub fn daccess16_nonseq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Nonsequential 32 bit data access
	pub fn daccess32_nonseq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Sequential 8bit code access
	pub fn caccess8_seq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Sequential 16 bit code access
	pub fn caccess16_seq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Sequential 32 bit code access
	pub fn caccess32_seq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Nonsequential 8bit code access
	pub fn caccess8_nonseq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Nonsequential 16 bit code access
	pub fn caccess16_nonseq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Nonsequential 32 bit code access
	pub fn caccess32_nonseq(&mut self, addr: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM alu operation.
	/// `pc_loaded` and `reg_shift` are set to true if the PC was loaded
	/// during that operation and/or if the ALU operation used a register
	/// shift respectively.
	pub fn clock_arm_alu(&mut self, prefetch: u32, pc_loaded: bool, reg_shift: bool) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM MSR instruction.
	/// `pc_loaded` is set to true if the PC was loaded during the operation.
	pub fn clock_arm_msr(&mut self, prefetch: u32, pc_loaded: bool) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM MSR instruction.
	/// `pc_loaded` is set to true if the PC was loaded during the operation.
	pub fn clock_arm_mrs(&mut self, prefetch: u32, pc_loaded: bool) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM LDR instruction.
	/// `pc_loaded` is set to true if the PC was loaded during the operation.
	pub fn clock_arm_ldr(&mut self, prefetch: u32, pc_loaded: bool, address: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM STR instruction.
	pub fn clock_arm_str(&mut self, prefetch: u32, address: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for the start of an ARM LDM instruction.
	/// This should be followed by the appropriate number of calls to 
	/// `clock_arm_ldm_single` or `clock_arm_ldm_single_pc`.
	pub fn clock_arm_ldm_start(&mut self, prefetch: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for a single transfer in an ARM LDM instruction.
	///
	/// Note: If the register being transferred to is the PC, `clock_arm_ldm_single_pc`
	/// should be used instead.
	pub fn clock_arm_ldm_single(&mut self, prefetch: u32, address: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for a single transfer in an ARM LDM instruction
	/// that transfer the data from an address to the PC.
	pub fn clock_arm_ldm_single_pc(&mut self ,address: u32) {
		unimplemented!();
	}

	/// Increments the cycler counter for the start of an ARM STM instruction.
	/// This should be followed by the appropriate number of calls to
	/// `clock_arm_stm_single`.
	pub fn clock_arm_stm_start(&mut self, prefetch: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for a single transfer in an ARM STM instruction.
	pub fn clock_arm_stm_single(&mut self, address: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for a single transfer in an ARM  SWP instruction.
	pub fn clock_arm_swp(&mut self, prefetch: u32, ddress: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM BX instruction.
	pub fn clock_arm_bx(&mut self, prefetch: u32, address: u32) {
		
	}

	/// Increments the cycle counter for an ARM Branch instruction.
	/// This function should be used for B, BL.
	/// The address that the branch is jumping should be passed in
	/// as the `address` argument.
	pub fn clock_arm_b(&mut self, prefetch: u32, address: u32) {
		self.daccess32_seq(prefetch); // prefetch of the next instruction is done anyways.
		self.daccess32_nonseq(address); // non sequential access of the next address.

		// sequential access of the following instruction. 
		// We just use the same one because chances are they are in the same area.
		self.daccess32_seq(address); 
	}

	/// Increments the cycle counter for the first part of the THUMB Long Branch instruction.
	pub fn clock_thumb_bl_setup(&mut self, prefetch: u32) {
		self.daccess16_seq(prefetch); // The prefetch is done anyways.
	}

	/// Increments the cycle counter for the second part of the THUMB Long Branch instruction.
	/// The address that the branch is jumping to should be passed as 
	/// the `address` argument. 
	pub fn clock_thumb_bl_off(&mut self, prefetch: u32, address: u32) {
		self.daccess16_seq(prefetch); // The prefetch is done anyways.
		self.daccess16_nonseq(address); // peforms a fetch from the branch destination.
		self.daccess16_seq(address); // peforms a fetch on the next instruction to fill the pipeline.
	}

	/// Increments the cycle counter for an ARM SWI instruction.
	pub fn clock_arm_swi_trap(&mut self, prefetch: u32) {
		unimplemented!();
	}

	/// Increments the cycler counter for a 32-bit ARM MUL instruction.
	pub fn clock_arm_mul32(&mut self, prefetch: u32, rhs: u32) {
		unimplemented!();
	}

	/// Increments the cycler counter for a 32-bit ARM MLA instruction.
	pub fn clock_arm_mla32(&mut self, prefetch: u32, rhs: u32) {
		unimplemented!();
	}

	/// Increments the cycle counter for a 64-bit ARM MUL instruction.
	pub fn clock_arm_mul64(&mut self, prefetch: u32, rhs: u64) {
		unimplemented!();
	}

	/// Increments the cycle counter for a 64-bit ARM MLA instruction.
	pub fn clock_arm_mla64(&mut self, prefetch: u32, rhs: u64) {
		unimplemented!();
	}

	/// Increments the cycle counter for an ARM instruction 
	/// with a false condition that that was skipped over.
	pub fn clock_arm_skipped(&mut self, prefetch: u32) {
		unimplemented!();
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
