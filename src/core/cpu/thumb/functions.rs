use super::super::ArmCpu;

pub fn thumb_fn_neg(cpu: &mut ArmCpu, _: u32, rhs: u32) -> u32 {
	super::super::arm::functions::arm_fn_sub_s(cpu, 0, rhs)
}

pub fn thumb_fn_mul(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let result = lhs * rhs;
	super::super::alu::set_nz_flags(cpu, result);
	return result;
}

pub fn thumb_fn_ldr(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_ldrb(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_str(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_strb(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_ldrh(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_strh(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_ldrsb(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_fn_ldrsh(cpu: &mut ArmCpu, address: u32, rd: u32) {

}

pub fn thumb_sdt_addr_reg(cpu: &ArmCpu, instr: u32) -> u32 {0}
pub fn thumb_sdt_addr_imm5(cpu: &ArmCpu, instr: u32) -> u32 {0}