pub mod arm;
pub mod thumb;
pub mod alu;
pub mod clock;

use super::memory::GbaMemory;
use super::registers::*;
use self::arm::execute_arm;
use self::thumb::execute_thumb;
use self::clock::*;

struct Pipeline<T : Copy> {
	fetched: T,
	decoded: T,
	count: u8
}

impl<T : Copy> Pipeline<T> {
	pub fn new(initTo: T) -> Pipeline<T> {
		Pipeline {
			fetched: initTo,
			decoded: initTo,
			count: 0u8
		}
	}

	pub fn flush(&mut self) {
		self.count = 0;
	}

	pub fn next(&mut self, instr: T) {
		self.decoded = self.fetched;
		self.fetched = instr;
		self.count += 1;
	}

	/// Returns true if this pipeline is ready to execute.
	pub fn ready(&self) -> bool { self.count > 0 }
}


/// GameBoy ARM7TDMI Cpu.
pub struct ArmCpu {
	thumb_pipeline: Pipeline<u16>,
	arm_pipeline: Pipeline<u32>,
	pub registers: ArmRegisters,
	pub memory: GbaMemory
}

impl ArmCpu {
	pub fn new() -> ArmCpu {
		ArmCpu {
			thumb_pipeline: Pipeline::new(0u16),
			arm_pipeline: Pipeline::new(0u32),
			registers: ArmRegisters::new(),
			memory: GbaMemory::new()
		}
	}

	/// Advances the ARM pipeline.
	/// executes, decodes, and then fetches the next instruction.
	pub fn tick(&mut self) {
		let thumb_mode = self.registers.getf_t();
		if thumb_mode { self.thumb_tick(); }
		else { self.arm_tick(); } 
	}

	pub fn rget(&self, register: u32) -> u32 {
		return self.registers.get(register)
	}

	pub fn rset(&mut self, register: u32, value: u32) {
		return self.registers.set(register, value);
	}

	fn arm_tick(&mut self) {
		let branched;

		if self.arm_pipeline.ready() {
			let saved_pc = self.registers.get(REG_PC);
			let decoded = self.arm_pipeline.decoded;
			execute_arm(self, decoded);
			branched = saved_pc != self.registers.get(REG_PC);
		} else {
			branched = false;
		}

		if branched {
			self.arm_pipeline.flush();
		} else {
			let pc = self.registers.get(REG_PC);
			let next = self.memory.read32(pc);
			self.registers.set(REG_PC, pc + 4);
			self.arm_pipeline.next(next);
		}
	}

	fn thumb_tick(&mut self) {
		let branched;

		if self.thumb_pipeline.ready() {
			let saved_pc = self.registers.get(REG_PC);
			let decoded = self.thumb_pipeline.decoded;
			execute_thumb(self, decoded);
			branched = saved_pc != self.registers.get(REG_PC);
		} else {
			branched = false;
		}

		if branched {
			self.thumb_pipeline.flush();
		} else {
			let pc = self.registers.get(REG_PC);
			let next = self.memory.read16(pc);
			self.registers.set(REG_PC, pc + 2);
			self.thumb_pipeline.next(next);
		}
	}

	/// ARM Condition Field {cond}
	/// The opcode {cond} suffixes can be used for conditionally executed code based on the C,N,Z,V flags in CPSR register. For example: BEQ = Branch if Equal, MOVMI = Move if Signed.
	/// In ARM mode, {cond} can be used with all opcodes (except for a few newer ARMv5 instructions: BKPT, PLD, CDP2, LDC2, MCR2, MRC2, STC2, and BLX_imm are nonconditional; however BLX_reg can be conditional).
	/// In THUMB mode, {cond} can be used only for branch opcodes.
	///   Code Suffix Flags         Meaning
	///   0:   EQ     Z=1           equal (zero) (same)
	///   1:   NE     Z=0           not equal (nonzero) (not same)
	///   2:   CS/HS  C=1           unsigned higher or same (carry set)
	///   3:   CC/LO  C=0           unsigned lower (carry cleared)
	///   4:   MI     N=1           negative (minus)
	///   5:   PL     N=0           positive or zero (plus)
	///   6:   VS     V=1           overflow (V set)
	///   7:   VC     V=0           no overflow (V cleared)
	///   8:   HI     C=1 and Z=0   unsigned higher
	///   9:   LS     C=0 or Z=1    unsigned lower or same
	///   A:   GE     N=V           greater or equal
	///   B:   LT     N<>V          less than
	///   C:   GT     Z=0 and N=V   greater than
	///   D:   LE     Z=1 or N<>V   less or equal
	///   E:   AL     -             always (the "AL" suffix can be omitted)
	///   F:   NV     -             never (ARMv1,v2 only) (Reserved ARMv3 and up)
	/// Execution Time: If condition=false: 1S cycle. Otherwise: as specified for the respective opcode.
	fn check_condition(&self, condition: u32) -> bool {
		true // #TODO
	}

	/// Returns true if the program counter is at an executable
	/// location.
	pub fn executable(&self) -> bool {
		let pc = self.registers.get(REG_PC);
		let area = (pc >> 24) & 0xf; // Area of memory we are at.
		match area {
			// BIOS, External Work Ram, Internal Work Ram, ROM0-2
			0 | 2 | 3 | 8 | 0xA | 0xC => true,
			_ => false
		}
	}
}