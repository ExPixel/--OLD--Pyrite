use super::super::ArmCpu;
use super::super::super::memory::GbaMemory;
use super::functions::*;
use super::super::alu::*;
use super::super::arm::functions::*;
use std::convert::Into;

// #TODO there is a lot of code duplication here.
// I should clean it up in the future.

const ADDR_REG: fn(&ArmCpu, u32) -> u32 = thumb_sdt_addr_reg;
const ADDR_IMM5: fn(&ArmCpu, u32) -> u32 = thumb_sdt_addr_imm5;

const LDR: fn(&mut ArmCpu, u32, u32) = thumb_fn_ldr;
const LDRB: fn(&mut ArmCpu, u32, u32) = thumb_fn_ldrb;
const STR: fn(&mut ArmCpu, u32, u32) = thumb_fn_str;
const STRB: fn(&mut ArmCpu, u32, u32) = thumb_fn_strb;
const LDRH: fn(&mut ArmCpu, u32, u32) = thumb_fn_ldrh;
const STRH: fn(&mut ArmCpu, u32, u32) = thumb_fn_strh;
const LDRSB: fn(&mut ArmCpu, u32, u32) = thumb_fn_ldrsb;
const LDRSH: fn(&mut ArmCpu, u32, u32) = thumb_fn_ldrsh;

/// Generates a single data transfer instruction
macro_rules! gen_sdt {
	(
		$instr_name: ident,
		$transfer_fn: ident,
		$addressing_fn: ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			let address = $addressing_fn(cpu, instr);
			let rd = instr & 0x7;
			$transfer_fn(cpu, address, rd);
		}
	)
}

/// LSL imm
/// Logical shift-left register
/// Immediate value
pub fn thumb_lsl_imm(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	let result = arm_alu_lli_s(cpu, rs, offset5);
	cpu.rset(rd.into(), result);
}

/// LSR imm
/// Logical shift-right register
/// Immediate value
pub fn thumb_lsr_imm(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	let result = arm_alu_lri_s(cpu, rs, offset5);
	cpu.rset(rd, result);
}

/// ASR imm
/// Arithmetic shift-right register
/// Immediate value
pub fn thumb_asr_imm(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	let result = arm_alu_ari_s(cpu, rs, offset5);
	cpu.rset(rd, result);
}

/// ADD reg
/// Add to register
/// Register offset
pub fn thumb_add_reg(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let rn = (instr >> 6) & 0x7;
	let _rn = cpu.rget(rn);
	let result = arm_fn_add_s(cpu, rs, _rn);
	cpu.rset(rd, result);
}

/// SUB reg
/// Subtract from register
/// Register offset
pub fn thumb_sub_reg(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let rn = (instr >> 6) & 0x7;
	let _rn = cpu.rget(rn);
	let result = arm_fn_sub_s(cpu, rs, _rn);
	cpu.rset(rd, result);
}

/// ADD imm3
/// Add to register
/// 3-bit immediate offset
pub fn thumb_add_imm3(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset3 = (instr >> 6) & 0x7;
	let result = arm_fn_add_s(cpu, rs, offset3);
	cpu.rset(rd, result);
}

/// SUB imm3
/// Subtract from register
/// 3-bit immediate offset
pub fn thumb_sub_imm3(cpu: &mut ArmCpu, instr: u32) {
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset3 = (instr >> 6) & 0x7;
	let result = arm_fn_sub_s(cpu, rs, offset3);
	cpu.rset(rd, result);
}

/// MOV i8r0
/// Move value to a register
/// 8-bit immediate offset, using r0
pub fn thumb_mov_i8r0(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(0);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(0, result);
}

/// MOV i8r1
/// Move value to a register
/// 8-bit immediate offset, using r1
pub fn thumb_mov_i8r1(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(1);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(1, result);
}

/// MOV i8r2
/// Move value to a register
/// 8-bit immediate offset, using r2
pub fn thumb_mov_i8r2(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(2);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(2, result);
}

/// MOV i8r3
/// Move value to a register
/// 8-bit immediate offset, using r3
pub fn thumb_mov_i8r3(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(3);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(3, result);
}

/// MOV i8r4
/// Move value to a register
/// 8-bit immediate offset, using r4
pub fn thumb_mov_i8r4(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(4);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(4, result);
}

/// MOV i8r5
/// Move value to a register
/// 8-bit immediate offset, using r5
pub fn thumb_mov_i8r5(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(5);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(5, result);
}

/// MOV i8r6
/// Move value to a register
/// 8-bit immediate offset, using r6
pub fn thumb_mov_i8r6(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(6);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(6, result);
}

/// MOV i8r7
/// Move value to a register
/// 8-bit immediate offset, using r7
pub fn thumb_mov_i8r7(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(7);
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset(7, result);
}

/// CMP i8r0
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r0
pub fn thumb_cmp_i8r0(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(0);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r1
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r1
pub fn thumb_cmp_i8r1(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(1);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r2
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r2
pub fn thumb_cmp_i8r2(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(2);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r3
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r3
pub fn thumb_cmp_i8r3(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(3);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r4
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r4
pub fn thumb_cmp_i8r4(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(4);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r5
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r5
pub fn thumb_cmp_i8r5(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(5);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r6
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r6
pub fn thumb_cmp_i8r6(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(6);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r7
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r7
pub fn thumb_cmp_i8r7(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(7);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// ADD i8r0
/// Add to register
/// 8-bit immediate offset, using r0
pub fn thumb_add_i8r0(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(0);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(0, result);
}

/// ADD i8r1
/// Add to register
/// 8-bit immediate offset, using r1
pub fn thumb_add_i8r1(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(1);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(1, result);
}

/// ADD i8r2
/// Add to register
/// 8-bit immediate offset, using r2
pub fn thumb_add_i8r2(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(2);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(2, result);
}

/// ADD i8r3
/// Add to register
/// 8-bit immediate offset, using r3
pub fn thumb_add_i8r3(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(3);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(3, result);
}

/// ADD i8r4
/// Add to register
/// 8-bit immediate offset, using r4
pub fn thumb_add_i8r4(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(4);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(4, result);
}

/// ADD i8r5
/// Add to register
/// 8-bit immediate offset, using r5
pub fn thumb_add_i8r5(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(5);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(5, result);
}

/// ADD i8r6
/// Add to register
/// 8-bit immediate offset, using r6
pub fn thumb_add_i8r6(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(6);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(6, result);
}

/// ADD i8r7
/// Add to register
/// 8-bit immediate offset, using r7
pub fn thumb_add_i8r7(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(7);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(7, result);
}

/// SUB i8r0
/// Subtract from register
/// 8-bit immediate offset, using r0
pub fn thumb_sub_i8r0(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(0);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(0, result);
}

/// SUB i8r1
/// Subtract from register
/// 8-bit immediate offset, using r1
pub fn thumb_sub_i8r1(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(1);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(1, result);
}

/// SUB i8r2
/// Subtract from register
/// 8-bit immediate offset, using r2
pub fn thumb_sub_i8r2(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(2);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(2, result);
}

/// SUB i8r3
/// Subtract from register
/// 8-bit immediate offset, using r3
pub fn thumb_sub_i8r3(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(3);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(3, result);
}

/// SUB i8r4
/// Subtract from register
/// 8-bit immediate offset, using r4
pub fn thumb_sub_i8r4(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(4);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(4, result);
}

/// SUB i8r5
/// Subtract from register
/// 8-bit immediate offset, using r5
pub fn thumb_sub_i8r5(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(5);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(5, result);
}

/// SUB i8r6
/// Subtract from register
/// 8-bit immediate offset, using r6
pub fn thumb_sub_i8r6(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(6);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(6, result);
}

/// SUB i8r7
/// Subtract from register
/// 8-bit immediate offset, using r7
pub fn thumb_sub_i8r7(cpu: &mut ArmCpu, instr: u32) {
	let lhs = cpu.rget(7);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(7, result);
}

/// DP g1
/// Thumb data processing
/// Instruction group 1
pub fn thumb_dp_g1(cpu: &mut ArmCpu, instr: u32) {
	let dp_instr = super::table::THUMB_DP_OPCODE_TABLE[0][((instr >> 6) & 0x3) as usize];
	dp_instr(cpu, instr);
}

/// DP g2
/// Thumb data processing
/// Instruction group 2
pub fn thumb_dp_g2(cpu: &mut ArmCpu, instr: u32) {
	let dp_instr = super::table::THUMB_DP_OPCODE_TABLE[1][((instr >> 6) & 0x3) as usize];
	dp_instr(cpu, instr);
}

/// DP g3
/// Thumb data processing
/// Instruction group 3
pub fn thumb_dp_g3(cpu: &mut ArmCpu, instr: u32) {
	let dp_instr = super::table::THUMB_DP_OPCODE_TABLE[2][((instr >> 6) & 0x3) as usize];
	dp_instr(cpu, instr);
}

/// DP g4
/// Thumb data processing
/// Instruction group 4
pub fn thumb_dp_g4(cpu: &mut ArmCpu, instr: u32) {
	let dp_instr = super::table::THUMB_DP_OPCODE_TABLE[3][((instr >> 6) & 0x3) as usize];
	dp_instr(cpu, instr);
}

/// ADDH 
/// Add registers, select from all 16
pub fn thumb_addh(cpu: &mut ArmCpu, instr: u32) {
	let mut rd = instr & 0x7;
	if (instr & 0x80) != 0 { rd += 8; }
	let mut rs = instr & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let lhs = cpu.rget(rd);
	let rhs = cpu.rget(rs);
	let result = arm_fn_add(cpu, lhs, rhs);
	cpu.rset(rd, result);
}

/// CMPH 
/// Compare registers, select from all 16
pub fn thumb_cmph(cpu: &mut ArmCpu, instr: u32) {
	let mut rd = instr & 0x7;
	if (instr & 0x80) != 0 { rd += 8; }
	let mut rs = instr & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let lhs = cpu.rget(rd);
	let rhs = cpu.rget(rs);
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// MOVH 
/// Move to a register, select from all 16
pub fn thumb_movh(cpu: &mut ArmCpu, instr: u32) {
	let mut rd = instr & 0x7;
	if (instr & 0x80) != 0 { rd += 8; }
	let mut rs = instr & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let lhs = cpu.rget(rd);
	let rhs = cpu.rget(rs);
	let result = arm_fn_mov(cpu, lhs, rhs);
	cpu.rset(rd, result);
}

/// BX reg
/// Branch and switch execution modes
/// Register offset
pub fn thumb_bx_reg(cpu: &mut ArmCpu, instr: u32) {
	let mut rs = instr & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let address = cpu.rget(rs);
	if (address & 1) == 1 {
		// Branch into thumb mode.
		cpu.registers.setf_t();
		cpu.rset(15, address & 0xFFFFFFFE);
	} else {
		// Branch into arm mode.
		cpu.rset(15, address & 0xFFFFFFFC);
	}
}

/// Common ldrpc function.
fn thumb_ldrpc(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	let pc = cpu.rget(15) & 0xFFFFFFFE;
	let address = (instr & 0xff) << 2;
	let data = cpu.mread32(pc + address);
	cpu.rset(rd, data);
}

/// LDRPC r0
/// r15-relative load word
/// Using r0
pub fn thumb_ldrpc_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 0);
}

/// LDRPC r1
/// r15-relative load word
/// Using r1
pub fn thumb_ldrpc_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 1);
}

/// LDRPC r2
/// r15-relative load word
/// Using r2
pub fn thumb_ldrpc_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 2);
}

/// LDRPC r3
/// r15-relative load word
/// Using r3
pub fn thumb_ldrpc_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 3);
}

/// LDRPC r4
/// r15-relative load word
/// Using r4
pub fn thumb_ldrpc_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 4);
}

/// LDRPC r5
/// r15-relative load word
/// Using r5
pub fn thumb_ldrpc_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 5);
}

/// LDRPC r6
/// r15-relative load word
/// Using r6
pub fn thumb_ldrpc_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 6);
}

/// LDRPC r7
/// r15-relative load word
/// Using r7
pub fn thumb_ldrpc_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrpc(cpu, instr, 7);
}

/// STR reg
/// Store word
/// Register offset
gen_sdt!(thumb_str_reg, STR, ADDR_REG);

/// STRH reg
/// Store halfword
/// Register offset
gen_sdt!(thumb_strh_reg, STRH, ADDR_REG);

/// STRB reg
/// Store byte
/// Register offset
gen_sdt!(thumb_strb_reg, STRB, ADDR_REG);

/// LDRSB reg
/// Load signed byte
/// Register offset
gen_sdt!(thumb_ldrsb_reg, LDRSB, ADDR_REG);

/// LDR reg
/// Load word
/// Register offset
gen_sdt!(thumb_ldr_reg, LDR, ADDR_REG);

/// LDRH reg
/// Load halfword
/// Register offset
gen_sdt!(thumb_ldrh_reg, LDRH, ADDR_REG);

/// LDRB reg
/// Load byte
/// Register offset
gen_sdt!(thumb_ldrb_reg, LDRB, ADDR_REG);

/// LDRSH reg
/// Load signed halfword
/// Register offset
gen_sdt!(thumb_ldrsh_reg, LDRSH, ADDR_REG);

/// STR imm5
/// Store word
/// 5-bit immediate offset
gen_sdt!(thumb_str_imm5, STR, ADDR_IMM5);

/// LDR imm5
/// Load word
/// 5-bit immediate offset
gen_sdt!(thumb_ldr_imm5, LDR, ADDR_IMM5);

/// STRB imm5
/// Store byte
/// 5-bit immediate offset
gen_sdt!(thumb_strb_imm5, STRB, ADDR_IMM5);

/// LDRB imm5
/// Load byte
/// 5-bit immediate offset
gen_sdt!(thumb_ldrb_imm5, LDRB, ADDR_IMM5);

/// STRH imm5
/// Store halfword
/// 5-bit immediate offset
gen_sdt!(thumb_strh_imm5, STRH, ADDR_IMM5);

/// LDRH imm5
/// Load halfword
/// 5-bit immediate offset
gen_sdt!(thumb_ldrh_imm5, LDRH, ADDR_IMM5);

/// Common strsp function.
fn thumb_strsp(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	// #TODO
}

/// STRSP r0
/// r13-relative store word
/// Using r0
pub fn thumb_strsp_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 0);
}

/// STRSP r1
/// r13-relative store word
/// Using r1
pub fn thumb_strsp_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 1);
}

/// STRSP r2
/// r13-relative store word
/// Using r2
pub fn thumb_strsp_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 2);
}

/// STRSP r3
/// r13-relative store word
/// Using r3
pub fn thumb_strsp_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 3);
}

/// STRSP r4
/// r13-relative store word
/// Using r4
pub fn thumb_strsp_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 4);
}

/// STRSP r5
/// r13-relative store word
/// Using r5
pub fn thumb_strsp_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 5);
}

/// STRSP r6
/// r13-relative store word
/// Using r6
pub fn thumb_strsp_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 6);
}

/// STRSP r7
/// r13-relative store word
/// Using r7
pub fn thumb_strsp_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_strsp(cpu, instr, 7);
}

/// Common ldrsp function.
fn thumb_ldrsp(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	// #TODO
}

/// LDRSP r0
/// r13-relative load word
/// Using r0
pub fn thumb_ldrsp_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 0);
}

/// LDRSP r1
/// r13-relative load word
/// Using r1
pub fn thumb_ldrsp_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 1);
}

/// LDRSP r2
/// r13-relative load word
/// Using r2
pub fn thumb_ldrsp_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 2);
}

/// LDRSP r3
/// r13-relative load word
/// Using r3
pub fn thumb_ldrsp_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 3);
}

/// LDRSP r4
/// r13-relative load word
/// Using r4
pub fn thumb_ldrsp_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 4);
}

/// LDRSP r5
/// r13-relative load word
/// Using r5
pub fn thumb_ldrsp_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 5);
}

/// LDRSP r6
/// r13-relative load word
/// Using r6
pub fn thumb_ldrsp_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 6);
}

/// LDRSP r7
/// r13-relative load word
/// Using r7
pub fn thumb_ldrsp_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldrsp(cpu, instr, 7);
}

// Common addpc function.
fn thumb_addpc(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	// #TODO
}

/// ADDPC r0
/// Using r0
pub fn thumb_addpc_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 0);
}

/// ADDPC r1
/// Using r1
pub fn thumb_addpc_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 1);
}

/// ADDPC r2
/// Using r2
pub fn thumb_addpc_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 2);
}

/// ADDPC r3
/// Using r3
pub fn thumb_addpc_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 3);
}

/// ADDPC r4
/// Using r4
pub fn thumb_addpc_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 4);
}

/// ADDPC r5
/// Using r5
pub fn thumb_addpc_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 5);
}

/// ADDPC r6
/// Using r6
pub fn thumb_addpc_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 6);
}

/// ADDPC r7
/// Using r7
pub fn thumb_addpc_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_addpc(cpu, instr, 7);
}

/// Common addsp function.
fn thumb_addsp(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	// #TODO
}

/// ADDSP r0
/// Using r0
pub fn thumb_addsp_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 0);
}

/// ADDSP r1
/// Using r1
pub fn thumb_addsp_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 1);
}

/// ADDSP r2
/// Using r2
pub fn thumb_addsp_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 2);
}

/// ADDSP r3
/// Using r3
pub fn thumb_addsp_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 3);
}

/// ADDSP r4
/// Using r4
pub fn thumb_addsp_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 4);
}

/// ADDSP r5
/// Using r5
pub fn thumb_addsp_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 5);
}

/// ADDSP r6
/// Using r6
pub fn thumb_addsp_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 6);
}

/// ADDSP r7
/// Using r7
pub fn thumb_addsp_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_addsp(cpu, instr, 7);
}

/// ADDSP imm7
/// 7-bit immediate offset
pub fn thumb_addsp_imm7(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// [undefined] [undefined]
/// [undefined]
/// [undefined]
pub fn thumb_undefined(cpu: &mut ArmCpu, instr: u32) {
	cpu.on_undefined();
}

/// PUSH 
/// Store multiple words to memory (STMDB equivalent)
pub fn thumb_push(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// PUSH lr
/// Store multiple words to memory (STMDB equivalent)
/// Include r14
pub fn thumb_push_lr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// POP 
/// Load multiple words from memory (LDMIA equivalent)
pub fn thumb_pop(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// POP pc
/// Load multiple words from memory (LDMIA equivalent)
/// Include r15
pub fn thumb_pop_pc(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r0
/// Store multiple words, increment after
/// Using r0
pub fn thumb_stmia_r0(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r1
/// Store multiple words, increment after
/// Using r1
pub fn thumb_stmia_r1(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r2
/// Store multiple words, increment after
/// Using r2
pub fn thumb_stmia_r2(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r3
/// Store multiple words, increment after
/// Using r3
pub fn thumb_stmia_r3(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r4
/// Store multiple words, increment after
/// Using r4
pub fn thumb_stmia_r4(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r5
/// Store multiple words, increment after
/// Using r5
pub fn thumb_stmia_r5(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r6
/// Store multiple words, increment after
/// Using r6
pub fn thumb_stmia_r6(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STMIA r7
/// Store multiple words, increment after
/// Using r7
pub fn thumb_stmia_r7(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r0
/// Load multiple words, increment after
/// Using r0
pub fn thumb_ldmia_r0(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r1
/// Load multiple words, increment after
/// Using r1
pub fn thumb_ldmia_r1(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r2
/// Load multiple words, increment after
/// Using r2
pub fn thumb_ldmia_r2(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r3
/// Load multiple words, increment after
/// Using r3
pub fn thumb_ldmia_r3(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r4
/// Load multiple words, increment after
/// Using r4
pub fn thumb_ldmia_r4(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r5
/// Load multiple words, increment after
/// Using r5
pub fn thumb_ldmia_r5(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r6
/// Load multiple words, increment after
/// Using r6
pub fn thumb_ldmia_r6(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDMIA r7
/// Load multiple words, increment after
/// Using r7
pub fn thumb_ldmia_r7(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BEQ 
/// Branch if zero flag set
pub fn thumb_beq(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BNE 
/// Branch if zero flag clear
pub fn thumb_bne(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BCS 
/// Branch if carry flag set
pub fn thumb_bcs(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BCC 
/// Branch if carry flag clear
pub fn thumb_bcc(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BMI 
/// Branch if negative flag set
pub fn thumb_bmi(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BPL 
/// Branch if negative flag clear
pub fn thumb_bpl(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BVS 
/// Branch if overflow flag set
pub fn thumb_bvs(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BVC 
/// Branch if overflow flag clear
pub fn thumb_bvc(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BHI 
/// Branch if higher (unsigned)
pub fn thumb_bhi(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BLS 
/// Branch if lower or the same (unsigned)
pub fn thumb_bls(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BGE 
/// Branch if greater than or equal to
pub fn thumb_bge(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BLT 
/// Branch if less than
pub fn thumb_blt(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BGT 
/// Branch if greater than
pub fn thumb_bgt(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BLE 
/// Branch if less than or equal to
pub fn thumb_ble(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// SWI 
/// Software interrupt (enter supervisor mode)
pub fn thumb_swi(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// B 
/// Branch
pub fn thumb_b(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BL setup
/// Branch and link
/// Two-instruction branch, high 11 bits of offset
pub fn thumb_bl_setup(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// BL off
/// Branch and link
/// Two-instruction branch, low 11 bits of offset
pub fn thumb_bl_off(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

