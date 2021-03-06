use super::super::ArmCpu;
// use super::super::super::memory::GbaMemory;
use super::functions::*;
use super::super::arm::functions::*;

// #CLEANUP there is a lot of code duplication here.
// I should clean it up in the future.

const SP: u32 = 13; 

const ADDR_REG: fn(&ArmCpu, u32) -> u32 = thumb_sdt_addr_reg;
const ADDR_IMM5: fn(&ArmCpu, u32) -> u32 = thumb_sdt_addr_imm5;
const ADDR_IMM6: fn(&ArmCpu, u32) -> u32 = thumb_sdt_addr_imm6;
const ADDR_IMM7: fn(&ArmCpu, u32) -> u32 = thumb_sdt_addr_imm7;

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
		$load: expr, // true if this is a load instruction.
		$transfer_fn: ident,
		$addressing_fn: ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_thumb();
			if $load {
				cpu.clock.internal(1);
			}
			let address = $addressing_fn(cpu, instr); // handles clock thingy.
			let rd = instr & 0x7;
			$transfer_fn(cpu, address, rd);
		}
	)
}

/// LSL imm
/// Logical shift-left register
/// Immediate value
pub fn thumb_lsl_imm(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	let _rs = cpu.rget_lo(rs);
	let result = thumb_fn_lli(cpu, _rs, offset5);
	cpu.rset_lo(rd, result);
}

/// LSR imm
/// Logical shift-right register
/// Immediate value
pub fn thumb_lsr_imm(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	let _rs = cpu.rget_lo(rs);
	let result = thumb_fn_lri(cpu, _rs, offset5);
	cpu.rset(rd, result);
}

/// ASR imm
/// Arithmetic shift-right register
/// Immediate value
pub fn thumb_asr_imm(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	let _rs = cpu.rget_lo(rs);
	let result = thumb_fn_ari(cpu, _rs, offset5);
	cpu.rset_lo(rd, result);
}

/// ADD reg
/// Add to register
/// Register offset
pub fn thumb_add_reg(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let rn = (instr >> 6) & 0x7;
	let _rs = cpu.rget_lo(rs);
	let _rn = cpu.rget_lo(rn);
	let result = arm_fn_add_s(cpu, _rs, _rn);
	cpu.rset_lo(rd, result);
}

/// SUB reg
/// Subtract from register
/// Register offset
pub fn thumb_sub_reg(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let rn = (instr >> 6) & 0x7;
	let _rs = cpu.rget_lo(rs);
	let _rn = cpu.rget_lo(rn);
	let result = arm_fn_sub_s(cpu, _rs, _rn);
	cpu.rset_lo(rd, result);
}

/// ADD imm3
/// Add to register
/// 3-bit immediate offset
pub fn thumb_add_imm3(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset3 = (instr >> 6) & 0x7;
	let _rs = cpu.rget_lo(rs);
	let result = arm_fn_add_s(cpu, _rs, offset3);
	cpu.rset_lo(rd, result);
}

/// SUB imm3
/// Subtract from register
/// 3-bit immediate offset
pub fn thumb_sub_imm3(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let rd = instr & 0x7;
	let rs = (instr >> 3) & 0x7;
	let offset3 = (instr >> 6) & 0x7;
	let _rs = cpu.rget_lo(rs);
	let result = arm_fn_sub_s(cpu, _rs, offset3);
	cpu.rset_lo(rd, result);
}

/// Common MOV i8 function
pub fn thumb_mov_i8(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	cpu.clock_prefetch_thumb();
	let lhs = cpu.rget_lo(rd); // #TODO this is useless, arm_fn_mov_s just discards it anyway.
	let rhs = instr & 0xff;
	let result = arm_fn_mov_s(cpu, lhs, rhs);
	cpu.rset_lo(rd, result);
}

/// MOV i8r0
/// Move value to a register
/// 8-bit immediate offset, using r0
pub fn thumb_mov_i8r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 0);
}

/// MOV i8r1
/// Move value to a register
/// 8-bit immediate offset, using r1
pub fn thumb_mov_i8r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 1);
}

/// MOV i8r2
/// Move value to a register
/// 8-bit immediate offset, using r2
pub fn thumb_mov_i8r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 2);
}

/// MOV i8r3
/// Move value to a register
/// 8-bit immediate offset, using r3
pub fn thumb_mov_i8r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 3);
}

/// MOV i8r4
/// Move value to a register
/// 8-bit immediate offset, using r4
pub fn thumb_mov_i8r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 4);
}

/// MOV i8r5
/// Move value to a register
/// 8-bit immediate offset, using r5
pub fn thumb_mov_i8r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 5);
}

/// MOV i8r6
/// Move value to a register
/// 8-bit immediate offset, using r6
pub fn thumb_mov_i8r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 6);
}

/// MOV i8r7
/// Move value to a register
/// 8-bit immediate offset, using r7
pub fn thumb_mov_i8r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_mov_i8(cpu, instr, 7);
}

pub fn thumb_cmp_i8(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	cpu.clock_prefetch_thumb();
	let lhs = cpu.rget_lo(rd);
	let rhs = instr & 0xff;
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// CMP i8r0
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r0
pub fn thumb_cmp_i8r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 0);
}

/// CMP i8r1
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r1
pub fn thumb_cmp_i8r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 1);
}

/// CMP i8r2
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r2
pub fn thumb_cmp_i8r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 2);
}

/// CMP i8r3
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r3
pub fn thumb_cmp_i8r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 3);
}

/// CMP i8r4
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r4
pub fn thumb_cmp_i8r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 4);
}

/// CMP i8r5
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r5
pub fn thumb_cmp_i8r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 5);
}

/// CMP i8r6
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r6
pub fn thumb_cmp_i8r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 6);
}

/// CMP i8r7
/// Compare register to value (Subtract)
/// 8-bit immediate offset, using r7
pub fn thumb_cmp_i8r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_cmp_i8(cpu, instr, 7);
}

/// Common add i8 function
pub fn thumb_add_i8(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	cpu.clock_prefetch_thumb();
	let lhs = cpu.rget_lo(rd);
	let rhs = instr & 0xff;
	let result = arm_fn_add_s(cpu, lhs, rhs);
	cpu.rset(rd, result);
}

/// ADD i8r0
/// Add to register
/// 8-bit immediate offset, using r0
pub fn thumb_add_i8r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 0);
}

/// ADD i8r1
/// Add to register
/// 8-bit immediate offset, using r1
pub fn thumb_add_i8r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 1);
}

/// ADD i8r2
/// Add to register
/// 8-bit immediate offset, using r2
pub fn thumb_add_i8r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 2);
}

/// ADD i8r3
/// Add to register
/// 8-bit immediate offset, using r3
pub fn thumb_add_i8r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 3);
}

/// ADD i8r4
/// Add to register
/// 8-bit immediate offset, using r4
pub fn thumb_add_i8r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 4);
}

/// ADD i8r5
/// Add to register
/// 8-bit immediate offset, using r5
pub fn thumb_add_i8r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 5);
}

/// ADD i8r6
/// Add to register
/// 8-bit immediate offset, using r6
pub fn thumb_add_i8r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 6);
}

/// ADD i8r7
/// Add to register
/// 8-bit immediate offset, using r7
pub fn thumb_add_i8r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_add_i8(cpu, instr, 7);
}

/// Common sub i8 funciton.
pub fn thumb_sub_i8(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	cpu.clock_prefetch_thumb();
	let lhs = cpu.rget_lo(rd);
	let rhs = instr & 0xff;
	let result = arm_fn_sub_s(cpu, lhs, rhs);
	cpu.rset(rd, result);
}

/// SUB i8r0
/// Subtract from register
/// 8-bit immediate offset, using r0
pub fn thumb_sub_i8r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 0);
}

/// SUB i8r1
/// Subtract from register
/// 8-bit immediate offset, using r1
pub fn thumb_sub_i8r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 1);
}

/// SUB i8r2
/// Subtract from register
/// 8-bit immediate offset, using r2
pub fn thumb_sub_i8r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 2);
}

/// SUB i8r3
/// Subtract from register
/// 8-bit immediate offset, using r3
pub fn thumb_sub_i8r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 3);
}

/// SUB i8r4
/// Subtract from register
/// 8-bit immediate offset, using r4
pub fn thumb_sub_i8r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 4);
}

/// SUB i8r5
/// Subtract from register
/// 8-bit immediate offset, using r5
pub fn thumb_sub_i8r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 5);
}

/// SUB i8r6
/// Subtract from register
/// 8-bit immediate offset, using r6
pub fn thumb_sub_i8r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 6);
}

/// SUB i8r7
/// Subtract from register
/// 8-bit immediate offset, using r7
pub fn thumb_sub_i8r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_sub_i8(cpu, instr, 7);
}

/// DP g1
/// Thumb data processing
/// Instruction group 1
pub fn thumb_dp_g1(cpu: &mut ArmCpu, instr: u32) {
	super::execute_thumb_dp_rc(cpu, instr, 0, (instr >> 6) & 0x3);
}

/// DP g2
/// Thumb data processing
/// Instruction group 2
pub fn thumb_dp_g2(cpu: &mut ArmCpu, instr: u32) {
	super::execute_thumb_dp_rc(cpu, instr, 1, (instr >> 6) & 0x3);
}

/// DP g3
/// Thumb data processing
/// Instruction group 3
pub fn thumb_dp_g3(cpu: &mut ArmCpu, instr: u32) {
	super::execute_thumb_dp_rc(cpu, instr, 2, (instr >> 6) & 0x3);
}

/// DP g4
/// Thumb data processing
/// Instruction group 4
pub fn thumb_dp_g4(cpu: &mut ArmCpu, instr: u32) {
	super::execute_thumb_dp_rc(cpu, instr, 3, (instr >> 6) & 0x3);
}

/// ADDH 
/// Add registers, select from all 16
pub fn thumb_addh(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut rd = instr & 0x7;
	if (instr & 0x80) != 0 { rd += 8; }
	let mut rs = (instr >> 3) & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let lhs = cpu.rget(rd);
	let rhs = cpu.rget(rs);
	let result = arm_fn_add(cpu, lhs, rhs);
	cpu.rset(rd, result);
}

/// CMPH 
/// Compare registers, select from all 16
pub fn thumb_cmph(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut rd = instr & 0x7;
	if (instr & 0x80) != 0 { rd += 8; }
	let mut rs = (instr >> 3) & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let lhs = cpu.rget(rd);
	let rhs = cpu.rget(rs);
	arm_fn_cmp_s(cpu, lhs, rhs);
}

/// MOVH 
/// Move to a register, select from all 16
pub fn thumb_movh(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut rd = instr & 0x7;
	if (instr & 0x80) != 0 { rd += 8; }
	let mut rs = (instr >> 3) & 0x7;
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
	cpu.clock_prefetch_thumb();
	let mut rs = (instr >> 3) & 0x7;
	if (instr & 0x40) != 0 { rs += 8; }
	let address = cpu.rget(rs);
	if (address & 1) == 1 {
		// Branch into thumb mode.
		cpu.set_pc(address & 0xFFFFFFFE);
		cpu.clock_branched_thumb();
	} else {
		// Branch into arm mode.
		cpu.registers.clearf_t();
		cpu.set_pc(address & 0xFFFFFFFC);
		cpu.clock_branched_arm();
	}
}

/// Common ldrpc function.
fn thumb_ldrpc(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	cpu.clock_prefetch_thumb();
	let pc = cpu.get_pc() & 0xFFFFFFFC; // Has to be word aligned.
	let offset = (instr & 0xff) << 2;
	let address = pc + offset;
	let data = cpu.mread32_al(address);
	cpu.rset_lo(rd, data);
	cpu.clock.internal(1);
	cpu.clock.data_access32_nonseq(address);
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
gen_sdt!(thumb_str_reg, false, STR, ADDR_REG);

/// STRH reg
/// Store halfword
/// Register offset
gen_sdt!(thumb_strh_reg, false, STRH, ADDR_REG);

/// STRB reg
/// Store byte
/// Register offset
gen_sdt!(thumb_strb_reg, false, STRB, ADDR_REG);

/// LDRSB reg
/// Load signed byte
/// Register offset
gen_sdt!(thumb_ldrsb_reg, true, LDRSB, ADDR_REG);

/// LDR reg
/// Load word
/// Register offset
gen_sdt!(thumb_ldr_reg, true, LDR, ADDR_REG);

/// LDRH reg
/// Load halfword
/// Register offset
gen_sdt!(thumb_ldrh_reg, true, LDRH, ADDR_REG);

/// LDRB reg
/// Load byte
/// Register offset
gen_sdt!(thumb_ldrb_reg, true, LDRB, ADDR_REG);

/// LDRSH reg
/// Load signed halfword
/// Register offset
gen_sdt!(thumb_ldrsh_reg, true, LDRSH, ADDR_REG);

/// STR imm5
/// Store word
/// 5-bit immediate offset
gen_sdt!(thumb_str_imm5, false, STR, ADDR_IMM7);

/// LDR imm5
/// Load word
/// 5-bit immediate offset
gen_sdt!(thumb_ldr_imm5, true, LDR, ADDR_IMM7);

/// STRB imm5
/// Store byte
/// 5-bit immediate offset
gen_sdt!(thumb_strb_imm5, false, STRB, ADDR_IMM5);

/// LDRB imm5
/// Load byte
/// 5-bit immediate offset
gen_sdt!(thumb_ldrb_imm5, true, LDRB, ADDR_IMM5);

/// STRH imm5
/// Store halfword
/// 5-bit immediate offset
gen_sdt!(thumb_strh_imm5, false, STRH, ADDR_IMM6);

/// LDRH imm5
/// Load halfword
/// 5-bit immediate offset
gen_sdt!(thumb_ldrh_imm5, true, LDRH, ADDR_IMM6);

/// Common strsp function.
fn thumb_strsp(cpu: &mut ArmCpu, instr: u32, rd: u32) {
	cpu.clock_prefetch_thumb();
	let offset = (instr & 0xff) << 2;
	let base = cpu.rget(SP);
	let address = base + offset;
	let data = cpu.rget_lo(rd);
	cpu.mwrite32(address, data);
	cpu.clock.data_access32_nonseq(address);
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
	cpu.clock_prefetch_thumb();
	let offset = (instr & 0xff) << 2;
	let base = cpu.rget(SP);
	let address = base + offset;
	let data = cpu.mread32_al(address);
	cpu.rset_lo(rd, data);
	cpu.clock.internal(1);
	cpu.clock.data_access32_nonseq(address);
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
	cpu.clock_prefetch_thumb();
	let offset = (instr & 0xff) << 2;
	let pc = cpu.get_pc() & 0xFFFFFFFC; // Has to be word aligned.
	cpu.rset_lo(rd, pc + offset);
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
	cpu.clock_prefetch_thumb();
	let offset = (instr & 0xff) << 2;
	let sp = cpu.rget(SP);
	cpu.rset_lo(rd, sp + offset);
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
	cpu.clock_prefetch_thumb();
	let mut offset = ((instr & 0x7f) << 2) as i32;
	if (instr & 0x80) != 0 { offset = -offset; }
	let sp = cpu.rget(SP) as i32;
	let result = (sp + offset) as u32;
	cpu.rset(SP, result);
}

/// [undefined] [undefined]
/// [undefined]
/// [undefined]
pub fn thumb_undefined(cpu: &mut ArmCpu, _: u32) {
	cpu.on_undefined();
}

#[inline(always)]
fn thumb_stm_single(cpu: &mut ArmCpu, src_reg: u32, address: &mut u32) {
	let src_data = cpu.rget(src_reg);
	cpu.mwrite32(*address, src_data);
	*address += 4;
}

#[inline(always)]
fn thumb_ldm_single(cpu: &mut ArmCpu, dst_reg: u32, address: &mut u32) {
	let src_data = cpu.mread32_al(*address);
	cpu.rset(dst_reg, src_data);
	*address += 4;
}

/**
 * Should only be used with decrementing load/store instructions...of where there are two in THUMB mode.
 */
#[inline(always)]
fn thumb_stm_single_wb(cpu: &mut ArmCpu, src_reg: u32, wb_reg: u32, wroteback: &mut bool, address: &mut u32) {
	let src_data = cpu.rget(src_reg);
	cpu.mwrite32(*address, src_data);
	if !(*wroteback) { cpu.rset(wb_reg, *address); *wroteback = true;}
	*address += 4;
}

/// PUSH 
/// Store multiple words to memory (STMDB equivalent)
pub fn thumb_push(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut first_access = true;

	let mut address = cpu.rget(SP);
	let mut wroteback = false;

	// first cycle:
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			address -= 4;
		}
	}

	// second cycle:
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			thumb_stm_single_wb(cpu, r, 13, &mut wroteback, &mut address);

			if first_access {
				cpu.clock.data_access32_nonseq(address);
				first_access = false;
			} else {
				cpu.clock.data_access32_seq(address);
			}
		}
	}
}

/// PUSH lr
/// Store multiple words to memory (STMDB equivalent)
/// Include r14
pub fn thumb_push_lr(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut first_access = true;

	let mut address = cpu.rget(SP);
	let mut wroteback = false;

	// first cycle:
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			address -= 4;
		}
	}
	address -= 4; // for lr

	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			thumb_stm_single_wb(cpu, r, 13, &mut wroteback, &mut address);

			if first_access {
				cpu.clock.data_access32_nonseq(address);
				first_access = false;
			} else {
				cpu.clock.data_access32_seq(address);
			}
		}
	}

	if first_access {
		cpu.clock.data_access32_nonseq(address);
	} else {
		cpu.clock.data_access32_seq(address);
	}

	thumb_stm_single_wb(cpu, 14, 13, &mut wroteback, &mut address);

}

/// POP 
/// Load multiple words from memory (LDMIA equivalent)
pub fn thumb_pop(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut first_access = true;

	let mut address = cpu.rget(SP);
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			if first_access {
				cpu.clock.data_access32_nonseq(address);
				first_access = false;
			} else {
				cpu.clock.data_access32_seq(address);
			}

			thumb_ldm_single(cpu, r, &mut address);
		}
	}
	cpu.rset(SP, address);
}

/// POP pc
/// Load multiple words from memory (LDMIA equivalent)
/// Include r15
pub fn thumb_pop_pc(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let mut first_access = true;

	let mut address = cpu.rget(SP);
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			if first_access {
				cpu.clock.data_access32_nonseq(address);
				first_access = false;
			} else {
				cpu.clock.data_access32_seq(address);
			}

			thumb_ldm_single(cpu, r, &mut address);
		}
	}


	if first_access {
		cpu.clock.data_access32_nonseq(address);
	} else {
		cpu.clock.data_access32_seq(address);
	}

	thumb_ldm_single(cpu, 15, &mut address);
	cpu.rset(SP, address);

	cpu.clock_branched_thumb();
}

/// Common STMIA instruction.
fn thumb_stmia(cpu: &mut ArmCpu, instr: u32, rb: u32) {
	cpu.clock_prefetch_thumb();
	let mut first_access = true;

	let mut address = cpu.rget(rb);

	// let rlist = instr & 0xff;
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			if first_access {
				cpu.clock.data_access32_nonseq(address);
				first_access = false;
			} else {
				cpu.clock.data_access32_seq(address);
			}

			thumb_stm_single(cpu, r, &mut address);
		}
	}
	cpu.rset(rb, address);
}

/// STMIA r0
/// Store multiple words, increment after
/// Using r0
pub fn thumb_stmia_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 0);
}

/// STMIA r1
/// Store multiple words, increment after
/// Using r1
pub fn thumb_stmia_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 1);
}

/// STMIA r2
/// Store multiple words, increment after
/// Using r2
pub fn thumb_stmia_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 2);
}

/// STMIA r3
/// Store multiple words, increment after
/// Using r3
pub fn thumb_stmia_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 3);
}

/// STMIA r4
/// Store multiple words, increment after
/// Using r4
pub fn thumb_stmia_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 4);
}

/// STMIA r5
/// Store multiple words, increment after
/// Using r5
pub fn thumb_stmia_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 5);
}

/// STMIA r6
/// Store multiple words, increment after
/// Using r6
pub fn thumb_stmia_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 6);
}

/// STMIA r7
/// Store multiple words, increment after
/// Using r7
pub fn thumb_stmia_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_stmia(cpu, instr, 7);
}

/// Common LDMIA instruction.
fn thumb_ldmia(cpu: &mut ArmCpu, instr: u32, rb: u32) {
	cpu.clock_prefetch_thumb();
	let mut first_access = true;

	let mut address = cpu.rget(rb);
	// let rlist = instr & 0xff;
	for r in 0..8 {
		if ((instr >> r) & 1) != 0 {
			if first_access {
				cpu.clock.data_access32_nonseq(address);
				first_access = false;
			} else {
				cpu.clock.data_access32_seq(address);
			}

			thumb_ldm_single(cpu, r, &mut address);
		}
	}
	cpu.rset_lo(rb, address);
}

/// LDMIA r0
/// Load multiple words, increment after
/// Using r0
pub fn thumb_ldmia_r0(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 0);
}

/// LDMIA r1
/// Load multiple words, increment after
/// Using r1
pub fn thumb_ldmia_r1(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 1);
}

/// LDMIA r2
/// Load multiple words, increment after
/// Using r2
pub fn thumb_ldmia_r2(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 2);
}

/// LDMIA r3
/// Load multiple words, increment after
/// Using r3
pub fn thumb_ldmia_r3(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 3);
}

/// LDMIA r4
/// Load multiple words, increment after
/// Using r4
pub fn thumb_ldmia_r4(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 4);
}

/// LDMIA r5
/// Load multiple words, increment after
/// Using r5
pub fn thumb_ldmia_r5(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 5);
}

/// LDMIA r6
/// Load multiple words, increment after
/// Using r6
pub fn thumb_ldmia_r6(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 6);
}

/// LDMIA r7
/// Load multiple words, increment after
/// Using r7
pub fn thumb_ldmia_r7(cpu: &mut ArmCpu, instr: u32) {
	thumb_ldmia(cpu, instr, 7);
}

/// Branch that occurs if the condition of the thumb
/// conditional branch passes.
#[inline(always)]
fn thumb_b_cond_passed(cpu: &mut ArmCpu, instr: u32) {
	let mut offset = instr & 0xff;
	offset = (((offset as i32) << 24) >> 24) as u32;
	offset <<= 1;
	let pc = cpu.get_pc() + offset;
	cpu.set_pc(pc);
	cpu.clock_branched_thumb();
}

/// BEQ 
/// Branch if zero flag set
pub fn thumb_beq(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_z() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BNE 
/// Branch if zero flag clear
pub fn thumb_bne(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if !cpu.registers.getf_z() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BCS 
/// Branch if carry flag set
pub fn thumb_bcs(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_c() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BCC 
/// Branch if carry flag clear
pub fn thumb_bcc(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if !cpu.registers.getf_c() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BMI 
/// Branch if negative flag set
pub fn thumb_bmi(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_n() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BPL 
/// Branch if negative flag clear
pub fn thumb_bpl(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if !cpu.registers.getf_n() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BVS 
/// Branch if overflow flag set
pub fn thumb_bvs(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_v() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BVC 
/// Branch if overflow flag clear
pub fn thumb_bvc(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if !cpu.registers.getf_v() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BHI 
/// Branch if higher (unsigned)
pub fn thumb_bhi(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_c() & !cpu.registers.getf_z() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BLS 
/// Branch if lower or the same (unsigned)
pub fn thumb_bls(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if !cpu.registers.getf_c() || cpu.registers.getf_z() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BGE 
/// Branch if greater than or equal to
pub fn thumb_bge(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_n() == cpu.registers.getf_v() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BLT 
/// Branch if less than
pub fn thumb_blt(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_n() != cpu.registers.getf_v() {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BGT 
/// Branch if greater than
pub fn thumb_bgt(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if !cpu.registers.getf_z() && (cpu.registers.getf_n() == cpu.registers.getf_v()) {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// BLE 
/// Branch if less than or equal to
pub fn thumb_ble(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	if cpu.registers.getf_z() || (cpu.registers.getf_n() != cpu.registers.getf_v()) {
		thumb_b_cond_passed(cpu, instr);
	}
}

/// SWI 
/// Software interrupt (enter supervisor mode)
pub fn thumb_swi(cpu: &mut ArmCpu, _: u32) {
	cpu.thumb_swi();
}

/// B 
/// Branch
pub fn thumb_b(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let soffset12 = ((((instr & 0x7ff) as i32) << 21) >> 20) as u32;
	let pc = cpu.get_pc() + soffset12;
	cpu.set_pc(pc);
	cpu.clock_branched_thumb();
}

/// BL setup
/// Branch and link
/// Two-instruction branch, high 11 bits of offset
pub fn thumb_bl_setup(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let soffset11 = ((((instr & 0x7ff) as i32) << 21) >> 9) as u32;
	let pc = cpu.get_pc();
	cpu.rset(14, pc + soffset11);
}

/// BL off
/// Branch and link
/// Two-instruction branch, low 11 bits of offset
pub fn thumb_bl_off(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_thumb();
	let offset12 = (instr & 0x7ff) << 1;
	let lr = cpu.rget(14);
	let address = lr + offset12;
	let next = cpu.get_pc() - 2;
	cpu.rset(14, next | 1); // bit 0 of lr must be set.
	cpu.set_pc(address);
	cpu.clock_branched_thumb();
}

