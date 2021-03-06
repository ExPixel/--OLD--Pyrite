use super::super::super::ArmCpu;
use super::super::super::alu::*;

fn sdt_reg_operands(cpu: &ArmCpu, instr: u32) -> (u32, u32) {
	let _rm = cpu.rget(instr & 0xf);
	let shift_amt = (instr >> 7) & 0x1f;
	(_rm, shift_amt)
}

pub fn arm_fn_ldrb(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread8_al(address) as u32;
	cpu.rset(rd, data);
	cpu.clock.data_access8_nonseq(address);
}

pub fn arm_fn_ldr(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread32_al(address);
	cpu.rset(rd, data);
	cpu.clock.data_access32_nonseq(address);
}

pub fn arm_fn_strb(cpu: &mut ArmCpu, address: u32, rd: u32) {
	// When R15 is the source register (Rd) of a register store (STR) instruction, 
	// the stored value will be address of the instruction plus 12
	let _src = if rd == 15 {
		cpu.get_pc() + 4
	} else {
		cpu.rget(rd)
	};
	let data = (_src & 0xff) as u8;
	cpu.mwrite8(address, data);
	cpu.clock.data_access8_nonseq(address);
}

pub fn arm_fn_str(cpu: &mut ArmCpu, address: u32, rd: u32) {
	// When R15 is the source register (Rd) of a register store (STR) instruction, 
	// the stored value will be address of the instruction plus 12
	let _src = if rd == 15 {
		cpu.get_pc() + 4
	} else {
		cpu.rget(rd)
	};
	let data = _src;
	cpu.mwrite32(address, data);
	cpu.clock.data_access32_nonseq(address);
}

pub fn arm_fn_ldrh(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread16_al(address) as u32;
	cpu.rset(rd, data);
	cpu.clock.data_access16_nonseq(address);
}

pub fn arm_fn_strh(cpu: &mut ArmCpu, address: u32, rd: u32) {
	// When R15 is the source register (Rd) of a register store (STR) instruction, 
	// the stored value will be address of the instruction plus 12
	let _src = if rd == 15 {
		cpu.get_pc() + 4
	} else {
		cpu.rget(rd)
	};
	let data = (_src & 0xffff) as u16;
	cpu.mwrite16(address, data);
	cpu.clock.data_access16_nonseq(address);
}

pub fn arm_fn_ldrsb(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread8_signed_al(address); // This is just a sign extension.
	cpu.rset(rd, data);
	cpu.clock.data_access8_nonseq(address);
}

pub fn arm_fn_ldrsh(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread16_signed_al(address); // This is just a sign extension.
	cpu.rset(rd, data);
	cpu.clock.data_access16_nonseq(address);
}

pub fn arm_fn_ldm_single(cpu: &mut ArmCpu, address: u32, dest_reg: u32) {
	let data = cpu.mread32_al(address);
	cpu.rset(dest_reg, data);
}

pub fn arm_fn_stm_single(cpu: &mut ArmCpu, address: u32, src_reg: u32) {
	let data = if src_reg == 15 {
		cpu.get_pc() + 4 // address of STM + 12
	} else {
		cpu.rget(src_reg)
	};
	cpu.mwrite32(address, data);
}

// the neg/pos versions of these functions
// are just used for the instructions that do not write back
// There is still some confusion so I'm keeping them for now
// and removing them when I have more information.
// They do the exact same thing as their non neg/pos counterparts though.

pub fn arm_fn_hdt_imm(_: &ArmCpu, instr: u32) -> u32 { ((instr >> 4) & 0xf0) | (instr & 0xf) }
pub fn arm_fn_hdt_reg(cpu: &ArmCpu, instr: u32) -> u32 { cpu.rget(instr & 0xf) }
pub fn arm_fn_hdt_neg_imm(cpu: &ArmCpu, instr: u32) -> u32 { arm_fn_hdt_imm(cpu, instr) }
pub fn arm_fn_hdt_neg_reg(cpu: &ArmCpu, instr: u32) -> u32 { arm_fn_hdt_reg(cpu, instr) }
pub fn arm_fn_hdt_pos_imm(cpu: &ArmCpu, instr: u32) -> u32 { arm_fn_hdt_imm(cpu, instr) }
pub fn arm_fn_hdt_pos_reg(cpu: &ArmCpu, instr: u32) -> u32 { arm_fn_hdt_reg(cpu, instr) }

pub fn arm_fn_sdt_imm(_: &ArmCpu, instr: u32) -> u32 {
	instr & 0xFFF
}

pub fn arm_fn_sdt_neg_imm(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_imm(cpu, instr)
}

pub fn arm_fn_sdt_pos_imm(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_imm(cpu, instr)
}

pub fn arm_fn_sdt_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_lli(lhs, rhs)
}

pub fn arm_fn_sdt_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_lri(lhs, rhs)
}

pub fn arm_fn_sdt_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_ari(lhs, rhs)
}

pub fn arm_fn_sdt_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_rri(cpu, lhs, rhs)
}

pub fn arm_fn_sdt_pos_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsl(cpu, instr)
}

pub fn arm_fn_sdt_pos_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsr(cpu, instr)
}

pub fn arm_fn_sdt_pos_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_asr(cpu, instr)
}

pub fn arm_fn_sdt_pos_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_ror(cpu, instr)
}

pub fn arm_fn_sdt_neg_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsl(cpu, instr)
}

pub fn arm_fn_sdt_neg_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsr(cpu, instr)
}

pub fn arm_fn_sdt_neg_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_asr(cpu, instr)
}

pub fn arm_fn_sdt_neg_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_ror(cpu, instr)
}