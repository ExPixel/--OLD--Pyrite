pub mod functions;
pub mod table;
pub mod thumb_dp_impl;
pub mod thumb_impl;

use super::ArmCpu;
use super::super::memory::GbaMemory;

pub fn execute_thumb(cpu: &mut ArmCpu, instr: u16) {
}