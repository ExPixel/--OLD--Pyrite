pub mod arm;
pub mod thumb;

use super::memory::GbaMemory;
use super::registers::*;
use self::arm::execute_arm;
use self::thumb::execute_thumb;

struct Pipeline<T : Copy> {
	fetched: T,
	decoded: T,
	count: u8
}

impl<T : Copy> Pipeline<T> {
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
	registers: ArmRegisters,
	memory: GbaMemory
}

impl ArmCpu {
	/// Advances the ARM pipeline.
	/// executes, decodes, and then fetches the next instruction.
	pub fn tick(&mut self) {
		let thumb_mode = self.registers.get_flag(REG_FLAG_T);
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
}