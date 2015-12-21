/*
 * General purpose functions for the ARM CPU.
 */
use super::super::ArmCpu;

pub fn arm_fn_mov(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_mov_s(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_and(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_and_s(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_orr(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_orr_s(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_eor(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_eor_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_bic(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_bic_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_sub(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_sub_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_rsb(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_rsb_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_add(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_add_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_adc(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_adc_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_sbc(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_sbc_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_rsc(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_rsc_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_tst_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_teq_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_cmp_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_cmn_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_mvn(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }

pub fn arm_fn_mvn_s(cpu: &ArmCpu, a: u32, b: u32) -> u32 { 0 }



// --- BARREL SHIFTER FUNCTIONS ---
pub fn arm_fn_op2_imm(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
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
pub fn arm_fn_op2_imm_s(cpu: &mut ArmCpu, instr: u32) -> u32 {
	0
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