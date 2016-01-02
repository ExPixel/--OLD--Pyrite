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
	pub fn ready(&self) -> bool { self.count > 1 }
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

	pub fn mread8(&self, address: u32) -> u8 {
		self.memory.read8(address)
	}

	pub fn mread16(&self, address: u32) -> u16 {
		self.memory.read16(address)
	}

	pub fn mread32(&self, address: u32) -> u32 {
		self.memory.read32(address)
	}

	pub fn mwrite8(&mut self, address: u32, value: u8) {
		self.memory.write8(address, value);
	}

	pub fn mwrite16(&mut self, address: u32, value: u16) {
		self.memory.write16(address, value);
	}

	pub fn mwrite32(&mut self, address: u32, value: u32) {
		self.memory.write32(address, value);
	}

	fn arm_tick(&mut self) {
		let branched;

		if self.arm_pipeline.ready() {
			let saved_pc = self.registers.get(REG_PC);
			let decoded = self.arm_pipeline.decoded;
			let condition = (decoded >> 28) & 0xf;

			{
				let __pc = self.registers.get(REG_PC) - 8;
				println!("executing {}", super::super::debug::armdis::disasm_arm(__pc, &self.memory, 0b11111111));
			}

			if self.check_condition(condition) {
				execute_arm(self, decoded);
			} /* #TODO else increase the clock by 1S cycle. */
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
		match condition {
			0x0 => self.registers.getf_z(),		// EQ
			0x1 => !self.registers.getf_z(),	// NE
			0x2 => self.registers.getf_c(),		// CS / HS
			0x3 => !self.registers.getf_c(),	// CC / LO
			0x4 => self.registers.getf_n(),		// MI
			0x5 => !self.registers.getf_n(),	// PL
			0x6 => self.registers.getf_v(),		// VS
			0x7 => !self.registers.getf_v(),	// VC
			0x8 => self.registers.getf_c() & !self.registers.getf_z(),	// HI
			0x9 => !self.registers.getf_c() || self.registers.getf_z(),	// LS
			0xA => self.registers.getf_n() == self.registers.getf_v(),	// GE
			0xB => self.registers.getf_n() != self.registers.getf_v(),	// LT
			0xC => !self.registers.getf_z() && (self.registers.getf_n() == self.registers.getf_v()), // GT
			0xD => self.registers.getf_z() && (self.registers.getf_n() != self.registers.getf_v()), // LE
			0xE => true, // AL
			0xF | _ => false // NV
		}
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

	/// Software interrupt in thumb mode.
	pub fn thumb_swi(&mut self, instr: u16) {

	}

	/// Software interrupt in ARM mode.
	pub fn arm_swi(&mut self, instr: u32) {
	}

	/// The CPU has hit an undefined instruction.
	pub fn on_undefined(&mut self) {
	}
}