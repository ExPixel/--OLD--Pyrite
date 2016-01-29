use super::super::super::ArmCpu;
use super::super::super::alu::*;

fn dataproc_imm_operands(cpu: &ArmCpu, instr: u32) -> (u32, u32) {
	let _rm = cpu.rget(instr & 0xf);
	let shift_amt = (instr >> 7) & 0x1f;
	(_rm, shift_amt)
}

fn dataproc_reg_operands(cpu: &ArmCpu, instr: u32) -> (u32, u32) {
	let rm = instr & 0xf;
	// If a register is used to specify the shift amount the PC will be 12 bytes ahead.
	let _rm = if rm == 15 { cpu.rget(15) + 4 } else { cpu.rget(rm) };
	// Only the least significant byte of the contents of Rs is used to determine the shift amount. 
	// Rs can be any general register other than R15.
	let _rs = cpu.rget((instr >> 8) & 0xf) & 0xff;
	(_rm, _rs)
}

/// The immediate operand rotate field is a 4 bit unsigned integer which specifies a shift
/// operation on the 8 bit immediate value. This value is zero extended to 32 bits, and then
/// subject to a rotate right by twice the value in the rotate field. This enables many
/// common constants to be generated, for example all powers of 2.
pub fn arm_fn_op2_imm(_: &mut ArmCpu, instr: u32) -> u32 {
	let imm = ((instr & 0xff) << 24) >> 24; // 8 bit imm value, sign extended to 32 bits
	let rotate = ((instr >> 8) & 0xf) * 2; // twice the rotate field as specified in documentation.
	imm.rotate_right(rotate)
}

pub fn arm_fn_op2_lli(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_lli(lhs, rhs)
}

pub fn arm_fn_op2_llr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_llr(lhs, rhs)
}

pub fn arm_fn_op2_lri(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_lri(lhs, rhs)
}

pub fn arm_fn_op2_lrr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_lrr(lhs, rhs)
}

pub fn arm_fn_op2_ari(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_ari(lhs, rhs)
}

pub fn arm_fn_op2_arr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_arr(lhs, rhs)
}

pub fn arm_fn_op2_rri(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_rri(cpu, lhs, rhs)
}

pub fn arm_fn_op2_rrr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_rrr(lhs, rhs)
}

// -- FLAG SETTING VERSIONS --
/// The immediate operand rotate field is a 4 bit unsigned integer which specifies a shift
/// operation on the 8 bit immediate value. This value is zero extended to 32 bits, and then
/// subject to a rotate right by twice the value in the rotate field. This enables many
/// common constants to be generated, for example all powers of 2.
pub fn arm_fn_op2_imm_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	arm_fn_op2_imm(cpu, instr)
}

pub fn arm_fn_op2_lli_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_lli_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_llr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_llr_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_lri_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_lri_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_lrr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_lrr_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_ari_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_ari_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_arr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_arr_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_rri_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_imm_operands(cpu, instr);
	arm_alu_rri_s(cpu, lhs, rhs)
}

pub fn arm_fn_op2_rrr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = dataproc_reg_operands(cpu, instr);;
	arm_alu_rrr_s(cpu, lhs, rhs)
}