use super::super::super::ArmCpu;

pub fn arm_fn_mov(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_mov_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_and(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs & rhs}

pub fn arm_fn_and_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_orr(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs | rhs }

pub fn arm_fn_orr_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_eor(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs ^ rhs }

pub fn arm_fn_eor_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_bic(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs & !rhs }

pub fn arm_fn_bic_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_sub(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs - rhs }

pub fn arm_fn_sub_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_rsb(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { rhs - lhs }

pub fn arm_fn_rsb_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_add(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { lhs + rhs }

pub fn arm_fn_add_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_adc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_adc_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_sbc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_sbc_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_rsc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_rsc_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_tst_s(cpu: &ArmCpu, lhs: u32, rhs: u32) {}

pub fn arm_fn_teq_s(cpu: &ArmCpu, lhs: u32, rhs: u32) {}

pub fn arm_fn_cmp_s(cpu: &ArmCpu, lhs: u32, rhs: u32) {}

pub fn arm_fn_cmn_s(cpu: &ArmCpu, lhs: u32, rhs: u32) {}

pub fn arm_fn_mvn(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }

pub fn arm_fn_mvn_s(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 { 0 }
