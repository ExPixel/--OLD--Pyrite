use super::ArmCpu;
use super::super::registers::*;

macro_rules! pos {
	($n:expr) => ($n >> 31)
}

macro_rules! neg {
	($n:expr) => ((!$n) >> 31)
}

#[inline]
pub fn arm_alu_adc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	lhs + rhs + cpu.registers.get_flagi(REG_FLAG_C)
}

#[inline]
pub fn arm_alu_sbc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	lhs - rhs - ((!cpu.registers.get_flag(REG_FLAG_C)) as u32)
}

#[inline]
pub fn arm_alu_rsc(cpu: &ArmCpu, lhs: u32, rhs: u32) -> u32 {
	rhs - lhs - ((!cpu.registers.get_flag(REG_FLAG_C)) as u32)
}

#[inline]
pub fn arm_alu_adcs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_sbcs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_rscs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_adds(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_subs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_rsbs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_ands(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn arm_alu_orrs(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {0}

#[inline]
pub fn set_add_flags(cpu: &mut ArmCpu, lhs: u32, rhs: u32, res: u32) {
	cpu.registers.put_flag(REG_FLAG_N, (res as i32) < 0);
	cpu.registers.put_flag(REG_FLAG_Z, res == 0);
	// The following is ported from VBA
	// Theirs seems to work well.
	cpu.registers.put_flagi(REG_FLAG_V, ((neg!(lhs) & neg!(rhs) & pos!(res)) | (pos!(lhs) & pos!(rhs) & neg!(res))));
	cpu.registers.put_flagi(REG_FLAG_C, ((neg!(lhs) & neg!(rhs)) | (neg!(lhs) & pos!(res)) | (neg!(rhs) & pos!(res))));
}

#[inline]
pub fn set_sub_flags(cpu: &mut ArmCpu, lhs: u32, rhs: u32, res: u32) {
	cpu.registers.put_flag(REG_FLAG_N, (res as i32) < 0);
	cpu.registers.put_flag(REG_FLAG_Z, res == 0);
	// The following is ported from VBA
	// Theirs seems to work well.
	cpu.registers.put_flagi(REG_FLAG_V, ((neg!(lhs) & pos!(rhs) & pos!(res)) | (pos!(lhs) & neg!(rhs) & neg!(res))));
	cpu.registers.put_flagi(REG_FLAG_C, ((neg!(lhs) & pos!(rhs)) | (neg!(lhs) & pos!(res)) | (pos!(rhs) & pos!(res))));
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