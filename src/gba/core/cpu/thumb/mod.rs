pub mod functions;
pub mod table;
pub mod thumb_dp_impl;
pub mod thumb_impl;

use super::ArmCpu;
// use super::super::memory::GbaMemory;

pub fn execute_thumb(cpu: &mut ArmCpu, instr: u32) {
	let instr_sz = instr as usize;
	let decoded = table::THUMB_OPCODE_TABLE[(instr_sz >> 12) & 0xf][(instr_sz >> 8) & 0xf];
	decoded(cpu, instr);
}