use super::ArmCpu;
use super::super::registers::*;

macro_rules! pos {
	($n:expr) => ($n >> 31)
}

macro_rules! neg {
	($n:expr) => ((!$n) >> 31)
}

pub fn arm_alu_adc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	lhs + rhs + cpu.registers.getfi_c()
}

pub fn arm_alu_sbc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	lhs - rhs - ((!cpu.registers.getf_c()) as u32)
}

pub fn arm_alu_rsc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	rhs - lhs - ((!cpu.registers.getf_c()) as u32)
}

pub fn arm_alu_adcs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = arm_alu_adc(cpu, lhs, rhs);
	set_add_flags(cpu, lhs, rhs, res); // #FIXME this might not be right.
	res
}

pub fn arm_alu_sbcs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = arm_alu_sbc(cpu, lhs, rhs);
	set_sub_flags(cpu, lhs, rhs, res); // #FIXME this might not be right.
	res
}

pub fn arm_alu_rscs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = arm_alu_rsc(cpu, lhs, rhs);
	set_sub_flags(cpu, lhs, rhs, res); // #FIXME this might not be right.
	res
}

pub fn arm_alu_adds(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = lhs + rhs;
	set_add_flags(cpu, lhs, rhs, res);
	res
}

pub fn arm_alu_subs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = lhs - rhs;
	set_sub_flags(cpu, lhs, rhs, res);
	res
}

pub fn arm_alu_rsbs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = rhs - lhs;
	set_sub_flags(cpu, lhs, rhs, res);
	res
}

pub fn arm_alu_ands(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = lhs & rhs;
	set_nz_flags(cpu, res);
	res
}

pub fn arm_alu_orrs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = lhs | rhs;
	set_nz_flags(cpu, res);
	res
}

pub fn arm_alu_eors(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = lhs ^ rhs;
	set_nz_flags(cpu, res);
	res
}

pub fn arm_alu_bics(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let res = lhs & !rhs;
	set_nz_flags(cpu, res);
	res
}

pub fn set_nz_flags(cpu: &mut ArmCpu, res: u32) {
	cpu.registers.putf_n((res as i32) < 0);
	cpu.registers.putf_z(res == 0);
}

pub fn set_add_flags(cpu: &mut ArmCpu, lhs: u32, rhs: u32, res: u32) {
	cpu.registers.putf_n((res as i32) < 0);
	cpu.registers.putf_z(res == 0);
	// The following is ported from VBA
	// Theirs seems to work well.
	cpu.registers.putfi_v(((neg!(lhs) & neg!(rhs) & pos!(res)) | (pos!(lhs) & pos!(rhs) & neg!(res))));
	cpu.registers.putfi_c(((neg!(lhs) & neg!(rhs)) | (neg!(lhs) & pos!(res)) | (neg!(rhs) & pos!(res))));
}

pub fn set_sub_flags(cpu: &mut ArmCpu, lhs: u32, rhs: u32, res: u32) {
	cpu.registers.putf_n((res as i32) < 0);
	cpu.registers.putf_z(res == 0);
	// The following is ported from VBA
	// Theirs seems to work well.
	cpu.registers.putfi_v(((neg!(lhs) & pos!(rhs) & pos!(res)) | (pos!(lhs) & neg!(rhs) & neg!(res))));
	cpu.registers.putfi_c(((neg!(lhs) & pos!(rhs)) | (neg!(lhs) & pos!(res)) | (pos!(rhs) & pos!(res))));
}

// ---- ARM ALU SHIFTS ----
pub fn arm_alu_lli(lhs: u32, rhs: u32) -> u32 {
	// LSL #0 is a special case, where the shifter carry out is the old value of the CPSR C
	// flag. The contents of Rm are used directly as the second operand.
	if rhs == 0 { lhs }
	else { lhs.arm_lsl(rhs) }
}

pub fn arm_alu_llr(lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// LSL by 32 has result zero, carry out equal to bit 0 of Rm.
	// LSL by more than 32 has result zero, carry out zero.
	if rhs >= 32 { 0 }
	else { lhs.arm_lsl(rhs) }
}

pub fn arm_alu_lri(lhs: u32, rhs: u32) -> u32 {
	// The form of the shift field which might be expected to 
	// correspond to LSR #0 is used to encode LSR #32
	let rhs = if rhs == 0 { 32 } else { rhs };
	lhs.arm_lsr(rhs)
}

pub fn arm_alu_lrr(lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// LSR by 32 has result zero, carry out equal to bit 31 of Rm.
	// LSR by more than 32 has result zero, carry out zero.
	if rhs >= 32 { 0 }
	else { lhs.arm_lsr(rhs) }
}

pub fn arm_alu_ari(lhs: u32, rhs: u32) -> u32 {
	// The form of the shift field which might be expected to give ASR #0 
	// is used to encode ASR #32
	let rhs = if rhs == 0 { 32 } else { rhs };
	lhs.arm_asr(rhs)
}

pub fn arm_alu_arr(lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// ASR by 32 or more has result filled with and carry out equal to bit 31 of Rm.
	if rhs >= 32 {
		if (lhs & 0x80000000) == 0 { 0x00000000 }
		else { 0xffffffff }
	} else {
		lhs.arm_asr(rhs)
	}
}

pub fn arm_alu_rri(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// The form of the shift field which might be expected to give ROR #0 
	// is used to encode a special function of the barrel shifter, rotate right extended (RRX)
	if rhs == 0 { return arm_alu_rrx(cpu, lhs) }
	lhs.arm_ror(rhs)
}

pub fn arm_alu_rrr(lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// ROR by n where n is greater than 32 will give the same result and carry out as ROR by n-32; 
	// therefore repeatedly subtract 32 from n until the amount is in the range 1 to 32 and see above.
	let rhs = rhs & 31; // This might not be right?
	// ROR by 32 has result equal to Rm, carry out equal to bit 31 of Rm.
	if rhs == 32 { lhs }
	else { lhs.arm_ror(rhs) }
}

pub fn arm_alu_rrx(cpu: &ArmCpu, lhs: u32) -> u32 {
	let carry = cpu.registers.getf_c();
	lhs.arm_rrx(carry)
}

// ---- ARM ALU SHIFTS + FLAGS ----
pub fn arm_alu_lli_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// LSL #0 is a special case, where the shifter carry out is the old value of the CPSR C
	// flag. The contents of Rm are used directly as the second operand.
	if rhs == 0 { lhs }
	else { 
		cpu.registers.putfi_c((lhs >> (32 - rhs)) & 1);
		lhs.arm_lsl(rhs)
	}
}

pub fn arm_alu_llr_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// LSL by 32 has result zero, carry out equal to bit 0 of Rm.
	// LSL by more than 32 has result zero, carry out zero.
	if rhs == 32 { cpu.registers.putfi_c(lhs & 1); 0 }
	else if rhs > 32 { cpu.registers.clearf_c(); 0 }
	else { 
		cpu.registers.putfi_c((lhs >> (32 - rhs)) & 1);
		lhs.arm_lsl(rhs)
	}
}

pub fn arm_alu_lri_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// The form of the shift field which might be expected to 
	// correspond to LSR #0 is used to encode LSR #32
	let rhs = if rhs == 0 { 32 } else { rhs };
	cpu.registers.putfi_c((lhs >> (rhs - 1)) & 1);
	lhs.arm_lsr(rhs)
}

pub fn arm_alu_lrr_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// LSR by 32 has result zero, carry out equal to bit 31 of Rm.
	// LSR by more than 32 has result zero, carry out zero.
	if rhs == 32 { cpu.registers.putfi_c(lhs & 0x80000000); 0 }
	else if rhs > 32 { cpu.registers.clearf_c(); 0 }
	else {
		cpu.registers.putfi_c((lhs >> (rhs - 1)) & 1);
		lhs.arm_lsr(rhs)
	}
}

pub fn arm_alu_ari_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// The form of the shift field which might be expected to give ASR #0 
	// is used to encode ASR #32
	let rhs = if rhs == 0 { 32 } else { rhs };
	cpu.registers.putfi_c((lhs >> (rhs - 1)) & 1);
	lhs.arm_asr(rhs)
}

pub fn arm_alu_arr_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// ASR by 32 or more has result filled with and carry out equal to bit 31 of Rm.
	if rhs >= 32 {
		cpu.registers.putfi_c(lhs & 0x80000000);
		if (lhs & 0x80000000) == 0 { 0x00000000 }
		else { 0xffffffff }
	} else {
		cpu.registers.putfi_c((lhs >> (rhs - 1)) & 1);
		lhs.arm_asr(rhs)
	}
}

pub fn arm_alu_rri_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// The form of the shift field which might be expected to give ROR #0 
	// is used to encode a special function of the barrel shifter, rotate right extended (RRX)
	if rhs == 0 { return arm_alu_rrx(cpu, lhs) }
	lhs.arm_ror(rhs)
}

pub fn arm_alu_rrr_s(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	// If this byte is zero, the unchanged contents of Rm will be used as the second operand, 
	// and the old value of the CPSR C flag will be passed on as the shifter carry output.
	if rhs == 0 { return lhs }
	// ROR by n where n is greater than 32 will give the same result and carry out as ROR by n-32; 
	// therefore repeatedly subtract 32 from n until the amount is in the range 1 to 32 and see above.
	let rhs = rhs & 31; // This might not be right?
	// ROR by 32 has result equal to Rm, carry out equal to bit 31 of Rm.
	if rhs == 32 { cpu.registers.putfi_c(lhs & 0x80000000); lhs }
	else {
		cpu.registers.putfi_c((lhs >> (rhs - 1)) & 1);
		lhs.arm_ror(rhs)
	}
}

pub fn arm_alu_rrx_s(cpu: &mut ArmCpu, lhs: u32) -> u32 {
	let carry = cpu.registers.getf_c();
	cpu.registers.putfi_c(lhs & 1);
	lhs.arm_rrx(carry)
}

/// Clearer versions of shifts
pub trait ClearArmShifts {
	/// Logical Shift Left
	#[inline(always)]
	fn arm_lsl(self, shift: Self) -> Self;

	/// Logical Shift Right
	#[inline(always)]
	fn arm_lsr(self, shift: Self) -> Self;

	/// Arithmetic Shift Right
	#[inline(always)]
	fn arm_asr(self, shift: Self) -> Self;

	/// Rotate Right
	#[inline(always)]
	fn arm_ror(self, shift: Self) -> Self;

	/// Rotate Right Extended
	#[inline(always)]
	fn arm_rrx(self, carry: bool) -> Self;
}

impl ClearArmShifts for u32 {
	/// Logical Shift Left
	#[inline(always)]
	fn arm_lsl(self, shift: u32) -> u32 { self << shift }

	/// Logical Shift Right
	#[inline(always)]
	fn arm_lsr(self, shift: u32) -> u32 { self >> shift }

	/// Arithmetic Shift Right
	#[inline(always)]
	fn arm_asr(self, shift: u32) -> u32 { ((self as i32) >> shift) as u32 }

	/// Rotate Right
	#[inline(always)]
	fn arm_ror(self, shift: u32) -> u32 {
		// This does become an ROR instruction at the end. :P
		// rorl	%cl, %edi
		(self << (32 - shift)) | (self >> shift)
	}

	/// Rotate Right Extended
	#[inline(always)]
	fn arm_rrx(self, carry: bool) -> u32 {
		let carry_shift = if carry { 1 } else { 0 };
		((self << (32 - carry_shift)) | (self >> carry_shift))
	}
}