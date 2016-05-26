pub mod functions;
pub mod arm_impl;

#[cfg(not(feature = "instr-match"))]
pub mod table;

#[cfg(feature = "instr-match")]
pub mod table_match;

use super::ArmCpu;
// use super::super::memory::GbaMemory;

#[cfg(not(feature = "instr-match"))]
pub fn execute_arm(cpu: &mut ArmCpu, instr: u32) {
	let instr_sz = instr as usize;
	let decoded = table::ARM_OPCODE_TABLE[(instr_sz >> 20) & 0xff][(instr_sz >> 4) &0xf];
	decoded(cpu, instr);
}

#[cfg(feature = "instr-match")]
pub fn execute_arm(cpu: &mut ArmCpu, instr: u32) {
	let row = (instr >> 20) & 0xff;
	let column = (instr >> 4) &0xf;
	let idx = table_match::to_index(row, column);
	table_match::run_instr(cpu, idx, instr);
}