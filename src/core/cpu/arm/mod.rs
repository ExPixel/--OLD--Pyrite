pub mod functions;
pub mod arm_impl;
pub mod table;

use super::ArmCpu;
use super::super::memory::GbaMemory;

pub fn execute_arm(cpu: &mut ArmCpu, instr: u32) {
	println!("Execute arm instruction.");
}