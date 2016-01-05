pub mod functions;
pub mod arm_impl;
pub mod table;

use super::ArmCpu;
// use super::super::memory::GbaMemory;

pub fn execute_arm(cpu: &mut ArmCpu, instr: u32) {
	let instr_sz = instr as usize;
	let decoded = table::ARM_OPCODE_TABLE[(instr_sz >> 20) & 0xff][(instr_sz >> 4) &0xf];
	decoded(cpu, instr);
}