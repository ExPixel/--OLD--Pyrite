use super::super::ArmCpu;
// use super::super::super::memory::GbaMemory;
use super::super::arm::functions::*;
use super::functions::*;

/// Generates a THUMB ALU operation.
macro_rules! gen_alu {
	(
		$instr_name:ident,
		$operation:ident
	) => (
    	pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_thumb();
			let rd = instr & 0x7;
			let rs = (instr >> 3) & 0x7;
			let _rd = cpu.rget(rd);
			let _rs = cpu.rget(rs);
			let result = $operation(cpu, _rd, _rs);
			cpu.rset(rd, result);
		}
	)
}

/// Generates a THUMB ALU operation that does not write back.
macro_rules! gen_alu_nw {
	(
		$instr_name:ident,
		$operation:ident
	) => (
    	pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_thumb();
			let rd = instr & 0x7;
			let rs = (instr >> 3) & 0x7;
			let _rd = cpu.rget(rd);
			let _rs = cpu.rget(rs);
			$operation(cpu, _rd, _rs)
			// ^ not including the semi-colon causes an error to occur
			// if I use a function that returns anything.
		}
	)
}

/// AND 
/// Logical And
gen_alu!(thumb_dp_and, arm_fn_and_s);

/// EOR 
/// Logical Exclusive-or
gen_alu!(thumb_dp_eor, arm_fn_eor_s);

/// LSL 
/// Logical Left-shift
gen_alu!(thumb_dp_lsl, thumb_fn_llr);

/// LSR 
/// Logical Right-shift
gen_alu!(thumb_dp_lsr, thumb_fn_lrr);

/// ASR 
/// Arithmetic Right-shift
gen_alu!(thumb_dp_asr, thumb_fn_arr);

/// ADC 
/// Add with carry
gen_alu!(thumb_dp_adc, arm_fn_adc_s);

/// SBC 
/// Subtract with carry
gen_alu!(thumb_dp_sbc, arm_fn_sbc_s);

/// ROR 
/// Rotate right
gen_alu!(thumb_dp_ror, thumb_fn_rrr);

/// TST 
/// Test Bits (Logical And)
gen_alu_nw!(thumb_dp_tst, arm_fn_tst_s);

/// NEG 
/// Negate (Subtract from zero)
gen_alu!(thumb_dp_neg, thumb_fn_neg);

/// CMP 
/// Compare (Subtract)
gen_alu_nw!(thumb_dp_cmp, arm_fn_cmp_s);

/// CMN 
/// Compare negative (Add)
gen_alu_nw!(thumb_dp_cmn, arm_fn_cmn_s);

/// ORR 
/// Logical Or
gen_alu!(thumb_dp_orr, arm_fn_orr_s);

/// MUL 
/// Multiply
gen_alu!(thumb_dp_mul, thumb_fn_mul); // thumb_fn_mul handles the MUL clock stuff on its own.

/// BIC 
/// Bit Clear (NAND)
gen_alu!(thumb_dp_bic, arm_fn_bic_s);

/// MVN 
/// Move negative (NOT)
gen_alu!(thumb_dp_mvn, arm_fn_mvn_s);

