pub mod functions;
pub mod thumb_dp_impl;
pub mod thumb_impl;

#[cfg(not(feature = "instr-match"))]
pub mod table;

#[cfg(feature = "instr-match")]
pub mod table_match;

use super::ArmCpu;
// use super::super::memory::GbaMemory;

// pub fn execute_thumb(cpu: &mut ArmCpu, instr: u32) {
// 	let instr_sz = instr as usize;
// 	let decoded = table::THUMB_OPCODE_TABLE[(instr_sz >> 12) & 0xf][(instr_sz >> 8) & 0xf];
// 	decoded(cpu, instr);
// }

pub fn execute_thumb(cpu: &mut ArmCpu, instr: u32) {
	let row = (instr >> 12) & 0xf;
	let column = (instr >> 8) & 0xf;
	execute_thumb_rc(cpu, instr, row, column);
}

#[cfg(not(feature = "instr-match"))]
#[inline(always)]
pub fn execute_thumb_rc(cpu: &mut ArmCpu, instr: u32, row: u32, column: u32) {
	let decoded = table::THUMB_OPCODE_TABLE[row as usize][column as usize];
	decoded(cpu, instr);
}

#[cfg(feature = "instr-match")]
#[inline(always)]
pub fn execute_thumb_rc(cpu: &mut ArmCpu, instr: u32, row: u32, column: u32) {
	let idx = table_match::to_index(row, column);
	table_match::run_instr(cpu, idx, instr);
}

#[cfg(feature = "instr-match")]
#[inline(always)]
pub fn execute_thumb_dp_rc(cpu: &mut ArmCpu, instr: u32, row: u32, column: u32) {
	let idx = table_match::to_index_dp(row, column);
	table_match::run_instr_dp(cpu, idx, instr);
}

#[cfg(not(feature = "instr-match"))]
#[inline(always)]
pub fn execute_thumb_dp_rc(cpu: &mut ArmCpu, instr: u32, row: u32, column: u32) {
	let decoded = table::THUMB_DP_OPCODE_TABLE[row as usize][column as usize];
	decoded(cpu, instr);
}