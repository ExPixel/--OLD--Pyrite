use super::super::super::ArmCpu;

/// The immediate operand rotate field is a 4 bit unsigned integer which specifies a shift
/// operation on the 8 bit immediate value. This value is zero extended to 32 bits, and then
/// subject to a rotate right by twice the value in the rotate field. This enables many
/// common constants to be generated, for example all powers of 2.
pub fn arm_fn_op2_imm(cpu: &mut ArmCpu, instr: u32) -> u32 {
	let imm = ((instr & 0xff) << 24) >> 24; // 8 bit imm value, sign extended to 32 bits
	let rotate = ((instr >> 8) & 0xf) * 2; // twice the rotate field as specified in documentation.
	imm.rotate_right(rotate)
}

pub fn arm_fn_op2_lli(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_llr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_lri(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_lrr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_ari(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_arr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_rri(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_rrr(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

// -- FLAG SETTING VERSIONS --
/// The immediate operand rotate field is a 4 bit unsigned integer which specifies a shift
/// operation on the 8 bit immediate value. This value is zero extended to 32 bits, and then
/// subject to a rotate right by twice the value in the rotate field. This enables many
/// common constants to be generated, for example all powers of 2.
pub fn arm_fn_op2_imm_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	// #FIXME I'm not sure if this is actually suppose to set
	// any flags, so for now this function will exist just in case
	// I have to make any changes in the future.
	arm_fn_op2_imm(cpu, instr)
}

pub fn arm_fn_op2_lli_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_llr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_lri_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_lrr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_ari_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_arr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_rri_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_op2_rrr_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
}