use super::super::ArmCpu;
// use super::super::super::memory::GbaMemory;
use super::thumb_impl::*;
use super::thumb_dp_impl::*;

// #TODO make this a match statement instead. (Measure first!)
pub static THUMB_OPCODE_TABLE:[[fn(&mut ArmCpu, u32); 16]; 16] = [
[thumb_lsl_imm, thumb_lsl_imm, thumb_lsl_imm, thumb_lsl_imm, thumb_lsl_imm, thumb_lsl_imm, thumb_lsl_imm, thumb_lsl_imm, thumb_lsr_imm, thumb_lsr_imm, thumb_lsr_imm, thumb_lsr_imm, thumb_lsr_imm, thumb_lsr_imm, thumb_lsr_imm, thumb_lsr_imm],
[thumb_asr_imm, thumb_asr_imm, thumb_asr_imm, thumb_asr_imm, thumb_asr_imm, thumb_asr_imm, thumb_asr_imm, thumb_asr_imm, thumb_add_reg, thumb_add_reg, thumb_sub_reg, thumb_sub_reg, thumb_add_imm3, thumb_add_imm3, thumb_sub_imm3, thumb_sub_imm3],
[thumb_mov_i8r0, thumb_mov_i8r1, thumb_mov_i8r2, thumb_mov_i8r3, thumb_mov_i8r4, thumb_mov_i8r5, thumb_mov_i8r6, thumb_mov_i8r7, thumb_cmp_i8r0, thumb_cmp_i8r1, thumb_cmp_i8r2, thumb_cmp_i8r3, thumb_cmp_i8r4, thumb_cmp_i8r5, thumb_cmp_i8r6, thumb_cmp_i8r7],
[thumb_add_i8r0, thumb_add_i8r1, thumb_add_i8r2, thumb_add_i8r3, thumb_add_i8r4, thumb_add_i8r5, thumb_add_i8r6, thumb_add_i8r7, thumb_sub_i8r0, thumb_sub_i8r1, thumb_sub_i8r2, thumb_sub_i8r3, thumb_sub_i8r4, thumb_sub_i8r5, thumb_sub_i8r6, thumb_sub_i8r7],
[thumb_dp_g1, thumb_dp_g2, thumb_dp_g3, thumb_dp_g4, thumb_addh, thumb_cmph, thumb_movh, thumb_bx_reg, thumb_ldrpc_r0, thumb_ldrpc_r1, thumb_ldrpc_r2, thumb_ldrpc_r3, thumb_ldrpc_r4, thumb_ldrpc_r5, thumb_ldrpc_r6, thumb_ldrpc_r7],
[thumb_str_reg, thumb_str_reg, thumb_strh_reg, thumb_strh_reg, thumb_strb_reg, thumb_strb_reg, thumb_ldrsb_reg, thumb_ldrsb_reg, thumb_ldr_reg, thumb_ldr_reg, thumb_ldrh_reg, thumb_ldrh_reg, thumb_ldrb_reg, thumb_ldrb_reg, thumb_ldrsh_reg, thumb_ldrsh_reg],
[thumb_str_imm5, thumb_str_imm5, thumb_str_imm5, thumb_str_imm5, thumb_str_imm5, thumb_str_imm5, thumb_str_imm5, thumb_str_imm5, thumb_ldr_imm5, thumb_ldr_imm5, thumb_ldr_imm5, thumb_ldr_imm5, thumb_ldr_imm5, thumb_ldr_imm5, thumb_ldr_imm5, thumb_ldr_imm5],
[thumb_strb_imm5, thumb_strb_imm5, thumb_strb_imm5, thumb_strb_imm5, thumb_strb_imm5, thumb_strb_imm5, thumb_strb_imm5, thumb_strb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5, thumb_ldrb_imm5],
[thumb_strh_imm5, thumb_strh_imm5, thumb_strh_imm5, thumb_strh_imm5, thumb_strh_imm5, thumb_strh_imm5, thumb_strh_imm5, thumb_strh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5, thumb_ldrh_imm5],
[thumb_strsp_r0, thumb_strsp_r1, thumb_strsp_r2, thumb_strsp_r3, thumb_strsp_r4, thumb_strsp_r5, thumb_strsp_r6, thumb_strsp_r7, thumb_ldrsp_r0, thumb_ldrsp_r1, thumb_ldrsp_r2, thumb_ldrsp_r3, thumb_ldrsp_r4, thumb_ldrsp_r5, thumb_ldrsp_r6, thumb_ldrsp_r7],
[thumb_addpc_r0, thumb_addpc_r1, thumb_addpc_r2, thumb_addpc_r3, thumb_addpc_r4, thumb_addpc_r5, thumb_addpc_r6, thumb_addpc_r7, thumb_addsp_r0, thumb_addsp_r1, thumb_addsp_r2, thumb_addsp_r3, thumb_addsp_r4, thumb_addsp_r5, thumb_addsp_r6, thumb_addsp_r7],
[thumb_addsp_imm7, thumb_undefined, thumb_undefined, thumb_undefined, thumb_push, thumb_push_lr, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_pop, thumb_pop_pc, thumb_undefined, thumb_undefined],
[thumb_stmia_r0, thumb_stmia_r1, thumb_stmia_r2, thumb_stmia_r3, thumb_stmia_r4, thumb_stmia_r5, thumb_stmia_r6, thumb_stmia_r7, thumb_ldmia_r0, thumb_ldmia_r1, thumb_ldmia_r2, thumb_ldmia_r3, thumb_ldmia_r4, thumb_ldmia_r5, thumb_ldmia_r6, thumb_ldmia_r7],
[thumb_beq, thumb_bne, thumb_bcs, thumb_bcc, thumb_bmi, thumb_bpl, thumb_bvs, thumb_bvc, thumb_bhi, thumb_bls, thumb_bge, thumb_blt, thumb_bgt, thumb_ble, thumb_undefined, thumb_swi],
[thumb_b, thumb_b, thumb_b, thumb_b, thumb_b, thumb_b, thumb_b, thumb_b, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined, thumb_undefined],
[thumb_bl_setup, thumb_bl_setup, thumb_bl_setup, thumb_bl_setup, thumb_bl_setup, thumb_bl_setup, thumb_bl_setup, thumb_bl_setup, thumb_bl_off, thumb_bl_off, thumb_bl_off, thumb_bl_off, thumb_bl_off, thumb_bl_off, thumb_bl_off, thumb_bl_off]
];

pub static THUMB_DP_OPCODE_TABLE:[[fn(&mut ArmCpu, u32); 4]; 4] = [
[thumb_dp_and, thumb_dp_eor, thumb_dp_lsl, thumb_dp_lsr],
[thumb_dp_asr, thumb_dp_add, thumb_dp_sub, thumb_dp_ror],
[thumb_dp_tst, thumb_dp_neg, thumb_dp_cmp, thumb_dp_cmn],
[thumb_dp_orr, thumb_dp_mul, thumb_dp_bic, thumb_dp_mvn]
];

