use super::ArmCpu;

pub fn arm_alu_adc(cpu: &ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_sbc(cpu: &ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_rsc(cpu: &ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_adcs(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_sbcs(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_rscs(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_adds(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_subs(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_rsbs(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_ands(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

pub fn arm_alu_orrs(cpu: &mut ArmCpu, a: u32, b: u32) -> u32 {0}

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