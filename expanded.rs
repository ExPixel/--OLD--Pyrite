#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use super::super::ArmCpu;
use super::super::super::registers;
use super::super::super::memory::GbaMemory;
use super::functions::*;



// Used to decide whether IO indexing should be pre/post function.
const PRE: bool = true;
const POST: bool = false;

// Used to decide whether IO indexing should increment or decrement.
const DEC: bool = true;
const INC: bool = false;

// Functions for loading/storing
const LDR: fn(&mut ArmCpu, u32, u32) = arm_fn_ldr;
const LDRB: fn(&mut ArmCpu, u32, u32) = arm_fn_ldrb;
const STR: fn(&mut ArmCpu, u32, u32) = arm_fn_str;
const STRB: fn(&mut ArmCpu, u32, u32) = arm_fn_strb;

// Functions for calculating the offset of a single data transfer.
const SDT_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_imm;
const SDT_LSL: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_lsl;
const SDT_LSR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_lsr;
const SDT_ASR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_asr;
const SDT_ROR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_ror;

const SDT_NEG_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_neg_imm;
const SDT_NEG_LSL: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_neg_lsl;
const SDT_NEG_LSR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_neg_lsr;
const SDT_NEG_ASR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_neg_asr;
const SDT_NEG_ROR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_neg_ror;

const SDT_POS_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_pos_imm;
const SDT_POS_LSL: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_pos_lsl;
const SDT_POS_LSR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_pos_lsr;
const SDT_POS_ASR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_pos_asr;
const SDT_POS_ROR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_pos_ror;
pub fn arm_and_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
         // the name of the instruction
         // the function being used by the instruction.
         // boolean - true if this is pre-indexed, false otherwise
         // boolean - true if this is incrementing, false if decrementing
         // the function used to generate an offset.
         // boolean - true if this should writeback (still writes back if post indexed or user mode)
         // boolean - true if this should force user mode registers.

         // base



        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_and_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// MUL
/// Multiply registers
pub fn arm_mul(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH ptrm
/// Store halfword
/// Register offset, post-decrement
pub fn arm_strh_ptrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}


/// UNDEFINED
/// just increments the clock
pub fn arm_undefined(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_ands_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// MULS
/// Multiply registers, setting flags
pub fn arm_muls(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRH ptrm
/// Load halfword
/// Register offset, post-decrement
pub fn arm_ldrh_ptrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ptrm
/// Load signed byte
/// Register offset, post-decrement
pub fn arm_ldrsb_ptrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ptrm
/// Load signed halfword
/// Register offset, post-decrement
pub fn arm_ldrsh_ptrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_eor_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// MLA
/// Multiply and accumulate registers
pub fn arm_mla(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_eors_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// MLAS
/// Multiply and accumulate registers, setting flags
pub fn arm_mlas(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_sub_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// STRH ptim
/// Store halfword
/// Immediate offset, post-decrement
pub fn arm_strh_ptim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_subs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// LDRH ptim
/// Load halfword
/// Immediate offset, post-decrement
pub fn arm_ldrh_ptim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ptim
/// Load signed byte
/// Immediate offset, post-decrement
pub fn arm_ldrsb_ptim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ptim
/// Load signed halfword
/// Immediate offset, post-decrement
pub fn arm_ldrsh_ptim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_rsb_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// UMULL
/// Unsigned long multiply (32x32 to 64)
pub fn arm_umull(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH ptrp
/// Store halfword
/// Register offset, post-increment
pub fn arm_strh_ptrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_adds_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// UMULLS
/// Unsigned long multiply, setting flags
pub fn arm_umulls(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRH ptrp
/// Load halfword
/// Register offset, post-increment
pub fn arm_ldrh_ptrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ptrp
/// Load signed byte
/// Register offset, post-increment
pub fn arm_ldrsb_ptrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ptrp
/// Load signed halfword
/// Register offset, post-increment
pub fn arm_ldrsh_ptrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_adc_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// UMLAL
/// Unsigned long multiply and accumulate
pub fn arm_umlal(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_adcs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// UMLALS
/// Unsigned long multiply and accumulate, setting flags
pub fn arm_umlals(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_sbc_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// SMULL
/// Signed long multiply (32x32 to 64)
pub fn arm_smull(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH ptip
/// Store halfword
/// Immediate offset, post-increment
pub fn arm_strh_ptip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_sbcs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// SMULLS
/// Signed long multiply, setting flags
pub fn arm_smulls(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRH ptip
/// Load halfword
/// Immediate offset, post-increment
pub fn arm_ldrh_ptip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ptip
/// Load signed byte
/// Immediate offset, post-increment
pub fn arm_ldrsb_ptip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ptip
/// Load signed halfword
/// Immediate offset, post-increment
pub fn arm_ldrsh_ptip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_rsc_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// SMLAL
/// Signed long multiply and accumulate
pub fn arm_smlal(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_rscs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// SMLALS
/// Signed long multiply and accumulate, setting flags
pub fn arm_smlals(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// MRS rc
/// Move status word to register
/// Register, CPSR
pub fn arm_mrs_rc(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// SWP
/// Swap registers with memory word
pub fn arm_swp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH ofrm
/// Store halfword
/// Negative register offset
pub fn arm_strh_ofrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_tsts_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}
pub fn arm_tsts_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}

/// LDRH ofrm
/// Load halfword
/// Negative register offset
pub fn arm_ldrh_ofrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ofrm
/// Load signed byte
/// Negative register offset
pub fn arm_ldrsb_ofrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ofrm
/// Load signed halfword
/// Negative register offset
pub fn arm_ldrsh_ofrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// MSR rc
/// Move value to status word
/// Register, CPSR
pub fn arm_msr_rc(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// BX
/// Branch and switch execution modes
pub fn arm_bx(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH prrm
/// Store halfword
/// Register offset, pre-decrement
pub fn arm_strh_prrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_teqs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_teqs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}

/// LDRH prrm
/// Load halfword
/// Register offset, pre-decrement
pub fn arm_ldrh_prrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB prrm
/// Load signed byte
/// Register offset, pre-decrement
pub fn arm_ldrsb_prrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH prrm
/// Load signed halfword
/// Register offset, pre-decrement
pub fn arm_ldrsh_prrm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// MRS rs
/// Move status word to register
/// Register, SPSR
pub fn arm_mrs_rs(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// SWPB
/// Swap registers with memory byte
pub fn arm_swpb(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH ofim
/// Store halfword
/// Negative immediate offset
pub fn arm_strh_ofim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_cmps_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}

/// LDRH ofim
/// Load halfword
/// Negative immediate offset
pub fn arm_ldrh_ofim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ofim
/// Load signed byte
/// Negative immediate offset
pub fn arm_ldrsb_ofim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ofim
/// Load signed halfword
/// Negative immediate offset
pub fn arm_ldrsh_ofim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// MSR rs
/// Move value to status word
/// Register, SPSR
pub fn arm_msr_rs(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STRH prim
/// Store halfword
/// Immediate offset, pre-decrement
pub fn arm_strh_prim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_cmns_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_cmns_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}

/// LDRH prim
/// Load halfword
/// Immediate offset, pre-decrement
pub fn arm_ldrh_prim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB prim
/// Load signed byte
/// Immediate offset, pre-decrement
pub fn arm_ldrsb_prim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH prim
/// Load signed halfword
/// Immediate offset, pre-decrement
pub fn arm_ldrsh_prim(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_orr_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orr_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// STRH ofrp
/// Store halfword
/// Positive register offset
pub fn arm_strh_ofrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_orrs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// LDRH ofrp
/// Load halfword
/// Positive register offset
pub fn arm_ldrh_ofrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ofrp
/// Load signed byte
/// Positive register offset
pub fn arm_ldrsb_ofrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ofrp
/// Load signed halfword
/// Positive register offset
pub fn arm_ldrsh_ofrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_mov_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// STRH prrp
/// Store halfword
/// Register offset, pre-increment
pub fn arm_strh_prrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_movs_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// LDRH prrp
/// Load halfword
/// Register offset, pre-increment
pub fn arm_ldrh_prrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB prrp
/// Load signed byte
/// Register offset, pre-increment
pub fn arm_ldrsb_prrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH prrp
/// Load signed halfword
/// Register offset, pre-increment
pub fn arm_ldrsh_prrp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_bic_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// STRH ofip
/// Store halfword
/// Positive immediate offset
pub fn arm_strh_ofip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_bics_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// LDRH ofip
/// Load halfword
/// Positive immediate offset
pub fn arm_ldrh_ofip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB ofip
/// Load signed byte
/// Positive immediate offset
pub fn arm_ldrsb_ofip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH ofip
/// Load signed halfword
/// Positive immediate offset
pub fn arm_ldrsh_ofip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_mvn_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// STRH prip
/// Store halfword
/// Immediate offset, pre-increment
pub fn arm_strh_prip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_mvns_lli(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lli_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_llr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_llr_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_lri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lri_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_lrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_lrr_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_ari(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_ari_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_arr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_arr_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_rri(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rri_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_rrr(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_rrr_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}

/// LDRH prip
/// Load halfword
/// Immediate offset, pre-increment
pub fn arm_ldrh_prip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSB prip
/// Load signed byte
/// Immediate offset, pre-increment
pub fn arm_ldrsb_prip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDRSH prip
/// Load signed halfword
/// Immediate offset, pre-increment
pub fn arm_ldrsh_prip(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_and_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_and(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_ands_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_and_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eor_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_eor(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_eors_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_eor_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sub_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_sub(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_subs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_sub_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsb_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_rsb(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsbs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_rsb_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_add_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_add(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adds_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_add_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adc_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_adc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_adcs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_adc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbc_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_sbc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_sbcs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_sbc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rsc_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_rsc(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_rscs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_rsc_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_tsts_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    arm_fn_tst_s(cpu, rn_value, operand2);
}

/// MSR ic
/// Move value to status word
/// Immediate, CPSR
pub fn arm_msr_ic(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_teqs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    arm_fn_teq_s(cpu, rn_value, operand2);
}
pub fn arm_cmps_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    arm_fn_cmp_s(cpu, rn_value, operand2);
}

/// MSR is
/// Move value to status word
/// Immediate, SPSR
pub fn arm_msr_is(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
pub fn arm_cmns_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    arm_fn_cmn_s(cpu, rn_value, operand2);
}
pub fn arm_orr_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_orr(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_orrs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_orr_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mov_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_mov(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_movs_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_mov_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bic_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_bic(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_bics_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_bic_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvn_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm(cpu, instr);
    let result = arm_fn_mvn(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_mvns_imm(cpu: &mut ArmCpu, instr: u32) {
    let rn = (instr >> 16) & 15;
    let rn_value = cpu.rget(rn);
    let operand2 =
        arm_fn_op2_imm_s(cpu, instr);
    let result = arm_fn_mvn_s(cpu, rn_value, operand2);
    let rd = (instr >> 12) & 15;
    cpu.rset(rd, result);
}
pub fn arm_str_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prim(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prip(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_IMM(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if DEC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDR(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_strbt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    STRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrbt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       true {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if POST {
            if INC { _rn + offset } else { _rn - offset }
        } else { _rn };
    LDRB(cpu, address, rd);
    if false || true || !(POST) {
        cpu.rset(rn,
                 if !(POST) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if true { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_NEG_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrmll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrmlr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrmar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrmrr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if DEC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if DEC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_ofrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_ofrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_str_prrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldr_prrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDR(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_ofrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_ofrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_POS_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if false || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_strb_prrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    STRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrpll(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSL(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrplr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_LSR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrpar(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ASR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}
pub fn arm_ldrb_prrprr(cpu: &mut ArmCpu, instr: u32) {
    let last_mode = cpu.registers.mode;
    if 
       false {
        cpu.registers.mode = registers::MODE_USR;
    }
    let rn = (instr >> 16) & 15;
    let rd = (instr >> 12) & 15;
    let _rn = cpu.rget(rn);
    let offset = SDT_ROR(cpu, instr);
    let address =
        if PRE { if INC { _rn + offset } else { _rn - offset } } else { _rn };
    LDRB(cpu, address, rd);
    if true || false || !(PRE) {
        cpu.rset(rn,
                 if !(PRE) {
                     if INC { _rn + offset } else { _rn - offset }
                 } else { _rn });
    }
    if false { cpu.registers.mode = last_mode; }
}

/// STMDA 
/// Store multiple words, decrement after
pub fn arm_stmda(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDA 
/// Load multiple words, decrement after
pub fn arm_ldmda(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDA w
/// Store multiple words, decrement after
/// Write back
pub fn arm_stmda_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDA w
/// Load multiple words, decrement after
/// Write back
pub fn arm_ldmda_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDA u
/// Store multiple words, decrement after
/// Use user-mode registers
pub fn arm_stmda_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDA u
/// Load multiple words, decrement after
/// Use user-mode registers
pub fn arm_ldmda_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDA uw
/// Store multiple words, decrement after
/// Use user-mode registers, with write back
pub fn arm_stmda_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDA uw
/// Load multiple words, decrement after
/// Use user-mode registers, with write back
pub fn arm_ldmda_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIA 
/// Store multiple words, increment after
pub fn arm_stmia(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIA 
/// Load multiple words, increment after
pub fn arm_ldmia(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIA w
/// Store multiple words, increment after
/// Write back
pub fn arm_stmia_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIA w
/// Load multiple words, increment after
/// Write back
pub fn arm_ldmia_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIA u
/// Store multiple words, increment after
/// Use user-mode registers
pub fn arm_stmia_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIA u
/// Load multiple words, increment after
/// Use user-mode registers
pub fn arm_ldmia_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIA uw
/// Store multiple words, increment after
/// Use user-mode registers, with write back
pub fn arm_stmia_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIA uw
/// Load multiple words, increment after
/// Use user-mode registers, with write back
pub fn arm_ldmia_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDB 
/// Store multiple words, decrement before
pub fn arm_stmdb(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDB 
/// Load multiple words, decrement before
pub fn arm_ldmdb(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDB w
/// Store multiple words, decrement before
/// Write back
pub fn arm_stmdb_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDB w
/// Load multiple words, decrement before
/// Write back
pub fn arm_ldmdb_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDB u
/// Store multiple words, decrement before
/// Use user-mode registers
pub fn arm_stmdb_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDB u
/// Load multiple words, decrement before
/// Use user-mode registers
pub fn arm_ldmdb_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMDB uw
/// Store multiple words, decrement before
/// Use user-mode registers, with write back
pub fn arm_stmdb_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMDB uw
/// Load multiple words, decrement before
/// Use user-mode registers, with write back
pub fn arm_ldmdb_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIB 
/// Store multiple words, increment before
pub fn arm_stmib(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIB 
/// Load multiple words, increment before
pub fn arm_ldmib(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIB w
/// Store multiple words, increment before
/// Write back
pub fn arm_stmib_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIB w
/// Load multiple words, increment before
/// Write back
pub fn arm_ldmib_w(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIB u
/// Store multiple words, increment before
/// Use user-mode registers
pub fn arm_stmib_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIB u
/// Load multiple words, increment before
/// Use user-mode registers
pub fn arm_ldmib_u(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STMIB uw
/// Store multiple words, increment before
/// Use user-mode registers, with write back
pub fn arm_stmib_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDMIB uw
/// Load multiple words, increment before
/// Use user-mode registers, with write back
pub fn arm_ldmib_uw(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// B 
/// Branch
pub fn arm_b(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// BL 
/// Branch and link
pub fn arm_bl(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC ofm
/// Store coprocessor data to memory
/// Negative offset
pub fn arm_stc_ofm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC ofm
/// Load coprocessor data from memory
/// Negative offset
pub fn arm_ldc_ofm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC prm
/// Store coprocessor data to memory
/// Pre-decrement
pub fn arm_stc_prm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC prm
/// Load coprocessor data from memory
/// Pre-decrement
pub fn arm_ldc_prm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC ofp
/// Store coprocessor data to memory
/// Positive offset
pub fn arm_stc_ofp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC ofp
/// Load coprocessor data from memory
/// Positive offset
pub fn arm_ldc_ofp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC prp
/// Store coprocessor data to memory
/// Pre-increment
pub fn arm_stc_prp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC prp
/// Load coprocessor data from memory
/// Pre-increment
pub fn arm_ldc_prp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC unm
/// Store coprocessor data to memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_stc_unm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC unm
/// Load coprocessor data from memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_ldc_unm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC ptm
/// Store coprocessor data to memory
/// Post-decrement
pub fn arm_stc_ptm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC ptm
/// Load coprocessor data from memory
/// Post-decrement
pub fn arm_ldc_ptm(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC unp
/// Store coprocessor data to memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_stc_unp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC unp
/// Load coprocessor data from memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_ldc_unp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// STC ptp
/// Store coprocessor data to memory
/// Post-increment
pub fn arm_stc_ptp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// LDC ptp
/// Load coprocessor data from memory
/// Post-increment
pub fn arm_ldc_ptp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// CDP 
/// Perform coprocessor data operation
pub fn arm_cdp(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// MCR 
/// Write coprocessor register from ARM register
pub fn arm_mcr(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// MRC 
/// Read coprocessor register to ARM register
pub fn arm_mrc(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}

/// SWI 
/// Software interrupt (enter supervisor mode)
pub fn arm_swi(cpu: &mut ArmCpu, instr: u32) {
    // #TODO
}
