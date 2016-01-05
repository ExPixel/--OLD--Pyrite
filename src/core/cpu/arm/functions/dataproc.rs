use super::super::super::ArmCpu;
use super::super::super::alu::*;

pub fn arm_fn_mov(_: &ArmCpu, _: u32, rhs: u32) -> u32 { rhs }

pub fn arm_fn_mov_s(cpu: &mut ArmCpu, _: u32, rhs: u32) -> u32 { set_nz_flags(cpu, rhs); rhs }

pub fn arm_fn_and(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs & rhs}

pub fn arm_fn_and_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_ands(cpu, lhs, rhs) }

pub fn arm_fn_orr(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs | rhs }

pub fn arm_fn_orr_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_orrs(cpu, lhs, rhs) }

pub fn arm_fn_eor(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs ^ rhs }

pub fn arm_fn_eor_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_eors(cpu, lhs, rhs) }

pub fn arm_fn_bic(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs & !rhs }

pub fn arm_fn_bic_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_bics(cpu, lhs, rhs) }

pub fn arm_fn_sub(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs - rhs }

pub fn arm_fn_sub_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_subs(cpu, lhs, rhs) }

pub fn arm_fn_rsb(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { rhs - lhs }

pub fn arm_fn_rsb_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_rsbs(cpu, lhs, rhs) }

pub fn arm_fn_add(_: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs + rhs }

pub fn arm_fn_add_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_adds(cpu, lhs, rhs) }

pub fn arm_fn_adc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_adc(cpu, lhs, rhs) }

pub fn arm_fn_adc_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_adcs(cpu, lhs, rhs) }

pub fn arm_fn_sbc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_sbc(cpu, lhs, rhs) }

pub fn arm_fn_sbc_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_sbcs(cpu, lhs, rhs) }

pub fn arm_fn_rsc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_rsc(cpu, lhs, rhs) }

pub fn arm_fn_rsc_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { arm_alu_rscs(cpu, lhs, rhs) }

pub fn arm_fn_tst_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) { arm_alu_ands(cpu, lhs, rhs); }

pub fn arm_fn_teq_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) { arm_alu_eors(cpu, lhs, rhs); }

pub fn arm_fn_cmp_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) { arm_alu_subs(cpu, lhs, rhs); }

pub fn arm_fn_cmn_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) { arm_alu_adds(cpu, lhs, rhs); }

pub fn arm_fn_mvn(_: &ArmCpu, _: u32, rhs: u32) -> u32 { !rhs }

pub fn arm_fn_mvn_s(cpu: &mut ArmCpu, _: u32, rhs: u32) -> u32 { let n = !rhs; set_nz_flags(cpu, n); n }
