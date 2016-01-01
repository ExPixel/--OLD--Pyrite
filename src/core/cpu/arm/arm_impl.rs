use super::super::ArmCpu;
use super::super::super::registers;
use super::super::super::memory::GbaMemory;
use super::functions::*;

/// Generates a function for a dataprocessing instruction.
///
/// Pass in the name of the instruction to generate,
/// The function used to retrieve the second operand
/// of the instruction, and a function to be applied
/// to both operands.
macro_rules! gen_dproc {
	(
		$instr_name:ident,
		$operand2_function:ident,
		$function:ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			let rn = (instr >> 16) & 0xf;
			let rn_value = cpu.rget(rn);
			let operand2 = $operand2_function(cpu, instr);
			let result = $function(cpu, rn_value, operand2);
			let rd = (instr >> 12) & 0xf;
			cpu.rset(rd, result);
		}
	)
}

/// Generates a function for a dataprocessing instruction
/// That does not write back to Rd.
///
/// Pass in the name of the instruction to generate,
/// The function used to retrieve the second operand
/// of the instruction, and a function to be applied
/// to both operands.
macro_rules! gen_dproc_nw {
	(
		$instr_name:ident,
		$operand2_function:ident,
		$function:ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			let rn = (instr >> 16) & 0xf;
			let rn_value = cpu.rget(rn);
			let operand2 = $operand2_function(cpu, instr);
			$function(cpu, rn_value, operand2);
		}
	)
}

// Used to decide whether IO indexing should be pre/post function.
const PRE: bool = true;
const POST: bool = false;

// Used to decide whether IO indexing should increment or decrement.
const DEC: bool = true;
const INC: bool = false;

// SDT Functions
const LDR: fn(&mut ArmCpu, u32, u32) = arm_fn_ldr;
const LDRB: fn(&mut ArmCpu, u32, u32) = arm_fn_ldrb;
const STR: fn(&mut ArmCpu, u32, u32) = arm_fn_str;
const STRB: fn(&mut ArmCpu, u32, u32) = arm_fn_strb;

// // HDT Functions
const LDRH: fn(&mut ArmCpu, u32, u32) = arm_fn_ldrh;
const STRH: fn(&mut ArmCpu, u32, u32) = arm_fn_strh;
const LDRSB: fn(&mut ArmCpu, u32, u32) = arm_fn_ldrsb;
const LDRSH: fn(&mut ArmCpu, u32, u32) = arm_fn_ldrsh;

const HDT_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_hdt_imm;
const HDT_REG: fn(&ArmCpu, u32) -> u32 = arm_fn_hdt_reg;
const HDT_NEG_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_hdt_neg_imm;
const HDT_NEG_REG: fn(&ArmCpu, u32) -> u32 = arm_fn_hdt_neg_reg;
const HDT_POS_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_hdt_pos_imm;
const HDT_POS_REG: fn(&ArmCpu, u32) -> u32 = arm_fn_hdt_pos_reg;

// Functions for calculating the offset of a single data transfer.
const SDT_IMM: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_imm;
const SDT_LSL: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_lsl;
const SDT_LSR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_lsr;
const SDT_ASR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_asr;
const SDT_ROR: fn(&ArmCpu, u32) -> u32 = arm_fn_sdt_ror;

// NEG & POS functions
// are pre indexed and don't write back.
// I'm separating them anyways for now though.
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

/// #TODO complete the remaining work on corner cases and the like,
/// such as the use of r15 in SDT instructions.
/// Generates a single data transfer instruction.
macro_rules! gen_sdt {
	(
		$instr_name:ident,	// the name of the instruction
		$function: ident,	// the function being used by the instruction.
		$index_pre: expr,	// boolean - true if this is pre-indexed, false otherwise
		$index_inc: expr,	// boolean - true if this is incrementing, false if decrementing
		$offset_fn: ident,	// the function used to generate an offset.
		$writeback: expr,	// boolean - true if this should writeback (still writes back if post indexed or user mode)
		$user: expr			// boolean - true if this should force user mode registers.
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			let last_mode = cpu.registers.mode;
			if $user {
				cpu.registers.mode = registers::MODE_USR;
			}

			let rn = (instr >> 16) & 0xf;
			let rd = (instr >> 12) & 0xf;
			let _rn = cpu.rget(rn); // base
			let offset = $offset_fn(cpu, instr);
			let address = if $index_pre {
				if $index_inc { _rn + offset }
				else { _rn - offset }
			} else { _rn };
			$function(cpu, address, rd);
			if $writeback || $user || !($index_pre) {
				cpu.rset(rn,
					if !($index_pre) {
						if $index_inc { _rn + offset }
						else { _rn - offset }
					} else { _rn }
				);
			}

			if $user {
				cpu.registers.mode = last_mode;
			}
		}
	)
}

macro_rules! gen_hdt {
	(
		$instr_name:ident,	// the name of the instruction
		$function: ident,	// the function being used by the instruction.
		$index_pre: expr,	// boolean - true if this is pre-indexed, false otherwise
		$index_inc: expr,	// boolean - true if this is incrementing, false if decrementing
		$offset_fn: ident,	// the function used to generate an offset.
		$writeback: expr	// boolean - true if this should writeback (still writes back if post indexed)
	) => (
		gen_sdt!($instr_name, $function, $index_pre, $index_inc, $offset_fn, $writeback, false);
	)
}


/// AND lli
/// Logical And
/// Logical shift-left by immediate
gen_dproc!(arm_and_lli, arm_fn_op2_lli, arm_fn_and);

/// AND llr
/// Logical And
/// Logical shift-left by register
gen_dproc!(arm_and_llr, arm_fn_op2_llr, arm_fn_and);

/// AND lri
/// Logical And
/// Logical shift-right by immediate
gen_dproc!(arm_and_lri, arm_fn_op2_lri, arm_fn_and);

/// AND lrr
/// Logical And
/// Logical shift-right by register
gen_dproc!(arm_and_lrr, arm_fn_op2_lrr, arm_fn_and);

/// AND ari
/// Logical And
/// Arithmetic shift-right by immediate
gen_dproc!(arm_and_ari, arm_fn_op2_ari, arm_fn_and);

/// AND arr
/// Logical And
/// Arithmetic shift-right by register
gen_dproc!(arm_and_arr, arm_fn_op2_arr, arm_fn_and);

/// AND rri
/// Logical And
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_and_rri, arm_fn_op2_rri, arm_fn_and);

/// AND rrr
/// Logical And
/// Rotate right by register
gen_dproc!(arm_and_rrr, arm_fn_op2_rrr, arm_fn_and);

/// MUL
/// Multiply registers
pub fn arm_mul(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRH ptrm
/// Store halfword
/// Register offset, post-decrement
gen_hdt!(arm_strh_ptrm, STRH, POST, DEC, HDT_REG, false);


/// UNDEFINED
/// just increments the clock
pub fn arm_undefined(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// ANDS lli
/// Logical And, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_ands_lli, arm_fn_op2_lli_s, arm_fn_and_s);

/// ANDS llr
/// Logical And, setting flags
/// Logical shift-left by register
gen_dproc!(arm_ands_llr, arm_fn_op2_llr_s, arm_fn_and_s);

/// ANDS lri
/// Logical And, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_ands_lri, arm_fn_op2_lri_s, arm_fn_and_s);

/// ANDS lrr
/// Logical And, setting flags
/// Logical shift-right by register
gen_dproc!(arm_ands_lrr, arm_fn_op2_lrr_s, arm_fn_and_s);

/// ANDS ari
/// Logical And, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_ands_ari, arm_fn_op2_ari_s, arm_fn_and_s);

/// ANDS arr
/// Logical And, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_ands_arr, arm_fn_op2_arr_s, arm_fn_and_s);

/// ANDS rri
/// Logical And, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_ands_rri, arm_fn_op2_rri_s, arm_fn_and_s);

/// ANDS rrr
/// Logical And, setting flags
/// Rotate right by register
gen_dproc!(arm_ands_rrr, arm_fn_op2_rrr_s, arm_fn_and_s);

/// MULS
/// Multiply registers, setting flags
pub fn arm_muls(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRH ptrm
/// Load halfword
/// Register offset, post-decrement
gen_hdt!(arm_ldrh_ptrm, LDRH, POST, DEC, HDT_REG, false);

/// LDRSB ptrm
/// Load signed byte
/// Register offset, post-decrement
gen_hdt!(arm_ldrsb_ptrm, LDRSB, POST, DEC, HDT_REG, false);

/// LDRSH ptrm
/// Load signed halfword
/// Register offset, post-decrement
gen_hdt!(arm_ldrsh_ptrm, LDRSH, POST, DEC, HDT_REG, false);

/// EOR lli
/// Logical Exclusive-or
/// Logical shift-left by immediate
gen_dproc!(arm_eor_lli, arm_fn_op2_lli, arm_fn_eor);

/// EOR llr
/// Logical Exclusive-or
/// Logical shift-left by register
gen_dproc!(arm_eor_llr, arm_fn_op2_llr, arm_fn_eor);

/// EOR lri
/// Logical Exclusive-or
/// Logical shift-right by immediate
gen_dproc!(arm_eor_lri, arm_fn_op2_lri, arm_fn_eor);

/// EOR lrr
/// Logical Exclusive-or
/// Logical shift-right by register
gen_dproc!(arm_eor_lrr, arm_fn_op2_lrr, arm_fn_eor);

/// EOR ari
/// Logical Exclusive-or
/// Arithmetic shift-right by immediate
gen_dproc!(arm_eor_ari, arm_fn_op2_ari, arm_fn_eor);

/// EOR arr
/// Logical Exclusive-or
/// Arithmetic shift-right by register
gen_dproc!(arm_eor_arr, arm_fn_op2_arr, arm_fn_eor);

/// EOR rri
/// Logical Exclusive-or
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_eor_rri, arm_fn_op2_rri, arm_fn_eor);

/// EOR rrr
/// Logical Exclusive-or
/// Rotate right by register
gen_dproc!(arm_eor_rrr, arm_fn_op2_rrr, arm_fn_eor);

/// MLA
/// Multiply and accumulate registers
pub fn arm_mla(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// EORS lli
/// Logical Exclusive-or, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_eors_lli, arm_fn_op2_lli_s, arm_fn_eor_s);

/// EORS llr
/// Logical Exclusive-or, setting flags
/// Logical shift-left by register
gen_dproc!(arm_eors_llr, arm_fn_op2_llr_s, arm_fn_eor_s);

/// EORS lri
/// Logical Exclusive-or, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_eors_lri, arm_fn_op2_lri_s, arm_fn_eor_s);

/// EORS lrr
/// Logical Exclusive-or, setting flags
/// Logical shift-right by register
gen_dproc!(arm_eors_lrr, arm_fn_op2_lrr_s, arm_fn_eor_s);

/// EORS ari
/// Logical Exclusive-or, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_eors_ari, arm_fn_op2_ari_s, arm_fn_eor_s);

/// EORS arr
/// Logical Exclusive-or, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_eors_arr, arm_fn_op2_arr_s, arm_fn_eor_s);

/// EORS rri
/// Logical Exclusive-or, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_eors_rri, arm_fn_op2_rri_s, arm_fn_eor_s);

/// EORS rrr
/// Logical Exclusive-or, setting flags
/// Rotate right by register
gen_dproc!(arm_eors_rrr, arm_fn_op2_rrr_s, arm_fn_eor_s);

/// MLAS
/// Multiply and accumulate registers, setting flags
pub fn arm_mlas(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// SUB lli
/// Subtract from register
/// Logical shift-left by immediate
gen_dproc!(arm_sub_lli, arm_fn_op2_lli, arm_fn_sub);

/// SUB llr
/// Subtract from register
/// Logical shift-left by register
gen_dproc!(arm_sub_llr, arm_fn_op2_llr, arm_fn_sub);

/// SUB lri
/// Subtract from register
/// Logical shift-right by immediate
gen_dproc!(arm_sub_lri, arm_fn_op2_lri, arm_fn_sub);

/// SUB lrr
/// Subtract from register
/// Logical shift-right by register
gen_dproc!(arm_sub_lrr, arm_fn_op2_lrr, arm_fn_sub);

/// SUB ari
/// Subtract from register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_sub_ari, arm_fn_op2_ari, arm_fn_sub);

/// SUB arr
/// Subtract from register
/// Arithmetic shift-right by register
gen_dproc!(arm_sub_arr, arm_fn_op2_arr, arm_fn_sub);

/// SUB rri
/// Subtract from register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_sub_rri, arm_fn_op2_rri, arm_fn_sub);

/// SUB rrr
/// Subtract from register
/// Rotate right by register
gen_dproc!(arm_sub_rrr, arm_fn_op2_rrr, arm_fn_sub);

/// STRH ptim
/// Store halfword
/// Immediate offset, post-decrement
gen_hdt!(arm_strh_ptim, STRH, POST, DEC, HDT_IMM, false);

/// SUBS lli
/// Subtract, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_subs_lli, arm_fn_op2_lli_s, arm_fn_sub_s);

/// SUBS llr
/// Subtract, setting flags
/// Logical shift-left by register
gen_dproc!(arm_subs_llr, arm_fn_op2_llr_s, arm_fn_sub_s);

/// SUBS lri
/// Subtract, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_subs_lri, arm_fn_op2_lri_s, arm_fn_sub_s);

/// SUBS lrr
/// Subtract, setting flags
/// Logical shift-right by register
gen_dproc!(arm_subs_lrr, arm_fn_op2_lrr_s, arm_fn_sub_s);

/// SUBS ari
/// Subtract, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_subs_ari, arm_fn_op2_ari_s, arm_fn_sub_s);

/// SUBS arr
/// Subtract, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_subs_arr, arm_fn_op2_arr_s, arm_fn_sub_s);

/// SUBS rri
/// Subtract, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_subs_rri, arm_fn_op2_rri_s, arm_fn_sub_s);

/// SUBS rrr
/// Subtract, setting flags
/// Rotate right by register
gen_dproc!(arm_subs_rrr, arm_fn_op2_rrr_s, arm_fn_sub_s);

/// LDRH ptim
/// Load halfword
/// Immediate offset, post-decrement
gen_hdt!(arm_ldrh_ptim, LDRH, POST, DEC, HDT_IMM, false);

/// LDRSB ptim
/// Load signed byte
/// Immediate offset, post-decrement
gen_hdt!(arm_ldrsb_ptim, LDRSB, POST, DEC, HDT_IMM, false);

/// LDRSH ptim
/// Load signed halfword
/// Immediate offset, post-decrement
gen_hdt!(arm_ldrsh_ptim, LDRSH, POST, DEC, HDT_IMM, false);

/// RSB lli
/// Subtract register from value
/// Logical shift-left by immediate
gen_dproc!(arm_rsb_lli, arm_fn_op2_lli, arm_fn_rsb);

/// RSB llr
/// Subtract register from value
/// Logical shift-left by register
gen_dproc!(arm_rsb_llr, arm_fn_op2_llr, arm_fn_rsb);

/// RSB lri
/// Subtract register from value
/// Logical shift-right by immediate
gen_dproc!(arm_rsb_lri, arm_fn_op2_lri, arm_fn_rsb);

/// RSB lrr
/// Subtract register from value
/// Logical shift-right by register
gen_dproc!(arm_rsb_lrr, arm_fn_op2_lrr, arm_fn_rsb);

/// RSB ari
/// Subtract register from value
/// Arithmetic shift-right by immediate
gen_dproc!(arm_rsb_ari, arm_fn_op2_ari, arm_fn_rsb);

/// RSB arr
/// Subtract register from value
/// Arithmetic shift-right by register
gen_dproc!(arm_rsb_arr, arm_fn_op2_arr, arm_fn_rsb);

/// RSB rri
/// Subtract register from value
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_rsb_rri, arm_fn_op2_rri, arm_fn_rsb);

/// RSB rrr
/// Subtract register from value
/// Rotate right by register
gen_dproc!(arm_rsb_rrr, arm_fn_op2_rrr, arm_fn_rsb);

/// RSBS lli
/// Reverse Subtract, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_rsbs_lli, arm_fn_op2_lli_s, arm_fn_rsb_s);

/// RSBS llr
/// Reverse Subtract, setting flags
/// Logical shift-left by register
gen_dproc!(arm_rsbs_llr, arm_fn_op2_llr_s, arm_fn_rsb_s);

/// RSBS lri
/// Reverse Subtract, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_rsbs_lri, arm_fn_op2_lri_s, arm_fn_rsb_s);

/// RSBS lrr
/// Reverse Subtract, setting flags
/// Logical shift-right by register
gen_dproc!(arm_rsbs_lrr, arm_fn_op2_lrr_s, arm_fn_rsb_s);

/// RSBS ari
/// Reverse Subtract, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_rsbs_ari, arm_fn_op2_ari_s, arm_fn_rsb_s);

/// RSBS arr
/// Reverse Subtract, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_rsbs_arr, arm_fn_op2_arr_s, arm_fn_rsb_s);

/// RSBS rri
/// Reverse Subtract, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_rsbs_rri, arm_fn_op2_rri_s, arm_fn_rsb_s);

/// RSBS rrr
/// Reverse Subtract, setting flags
/// Rotate right by register
gen_dproc!(arm_rsbs_rrr, arm_fn_op2_rrr_s, arm_fn_rsb_s);

/// ADD lli
/// Add to register
/// Logical shift-left by immediate
gen_dproc!(arm_add_lli, arm_fn_op2_lli, arm_fn_add);

/// ADD llr
/// Add to register
/// Logical shift-left by register
gen_dproc!(arm_add_llr, arm_fn_op2_llr, arm_fn_add);

/// ADD lri
/// Add to register
/// Logical shift-right by immediate
gen_dproc!(arm_add_lri, arm_fn_op2_lri, arm_fn_add);

/// ADD lrr
/// Add to register
/// Logical shift-right by register
gen_dproc!(arm_add_lrr, arm_fn_op2_lrr, arm_fn_add);

/// ADD ari
/// Add to register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_add_ari, arm_fn_op2_ari, arm_fn_add);

/// ADD arr
/// Add to register
/// Arithmetic shift-right by register
gen_dproc!(arm_add_arr, arm_fn_op2_arr, arm_fn_add);

/// ADD rri
/// Add to register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_add_rri, arm_fn_op2_rri, arm_fn_add);

/// ADD rrr
/// Add to register
/// Rotate right by register
gen_dproc!(arm_add_rrr, arm_fn_op2_rrr, arm_fn_add);

/// UMULL
/// Unsigned long multiply (32x32 to 64)
pub fn arm_umull(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRH ptrp
/// Store halfword
/// Register offset, post-increment
gen_hdt!(arm_strh_ptrp, STRH, POST, INC, HDT_REG, false);

/// ADDS lli
/// Add to register, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_adds_lli, arm_fn_op2_lli_s, arm_fn_add_s);

/// ADDS llr
/// Add to register, setting flags
/// Logical shift-left by register
gen_dproc!(arm_adds_llr, arm_fn_op2_llr_s, arm_fn_add_s);

/// ADDS lri
/// Add to register, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_adds_lri, arm_fn_op2_lri_s, arm_fn_add_s);

/// ADDS lrr
/// Add to register, setting flags
/// Logical shift-right by register
gen_dproc!(arm_adds_lrr, arm_fn_op2_lrr_s, arm_fn_add_s);

/// ADDS ari
/// Add to register, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_adds_ari, arm_fn_op2_ari_s, arm_fn_add_s);

/// ADDS arr
/// Add to register, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_adds_arr, arm_fn_op2_arr_s, arm_fn_add_s);

/// ADDS rri
/// Add to register, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_adds_rri, arm_fn_op2_rri_s, arm_fn_add_s);

/// ADDS rrr
/// Add to register, setting flags
/// Rotate right by register
gen_dproc!(arm_adds_rrr, arm_fn_op2_rrr_s, arm_fn_add_s);

/// UMULLS
/// Unsigned long multiply, setting flags
pub fn arm_umulls(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRH ptrp
/// Load halfword
/// Register offset, post-increment
gen_hdt!(arm_ldrh_ptrp, LDRH, POST, INC, HDT_REG, false);

/// LDRSB ptrp
/// Load signed byte
/// Register offset, post-increment
gen_hdt!(arm_ldrsb_ptrp, LDRSB, POST, INC, HDT_REG, false);

/// LDRSH ptrp
/// Load signed halfword
/// Register offset, post-increment
gen_hdt!(arm_ldrsh_ptrp, LDRSH, POST, INC, HDT_REG, false);

/// ADC lli
/// Add to register with carry
/// Logical shift-left by immediate
gen_dproc!(arm_adc_lli, arm_fn_op2_lli, arm_fn_adc);

/// ADC llr
/// Add to register with carry
/// Logical shift-left by register
gen_dproc!(arm_adc_llr, arm_fn_op2_llr, arm_fn_adc);

/// ADC lri
/// Add to register with carry
/// Logical shift-right by immediate
gen_dproc!(arm_adc_lri, arm_fn_op2_lri, arm_fn_adc);

/// ADC lrr
/// Add to register with carry
/// Logical shift-right by register
gen_dproc!(arm_adc_lrr, arm_fn_op2_lrr, arm_fn_adc);

/// ADC ari
/// Add to register with carry
/// Arithmetic shift-right by immediate
gen_dproc!(arm_adc_ari, arm_fn_op2_ari, arm_fn_adc);

/// ADC arr
/// Add to register with carry
/// Arithmetic shift-right by register
gen_dproc!(arm_adc_arr, arm_fn_op2_arr, arm_fn_adc);

/// ADC rri
/// Add to register with carry
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_adc_rri, arm_fn_op2_rri, arm_fn_adc);

/// ADC rrr
/// Add to register with carry
/// Rotate right by register
gen_dproc!(arm_adc_rrr, arm_fn_op2_rrr, arm_fn_adc);

/// UMLAL
/// Unsigned long multiply and accumulate
pub fn arm_umlal(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// ADCS lli
/// Add to register with carry, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_adcs_lli, arm_fn_op2_lli_s, arm_fn_adc_s);

/// ADCS llr
/// Add to register with carry, setting flags
/// Logical shift-left by register
gen_dproc!(arm_adcs_llr, arm_fn_op2_llr_s, arm_fn_adc_s);

/// ADCS lri
/// Add to register with carry, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_adcs_lri, arm_fn_op2_lri_s, arm_fn_adc_s);

/// ADCS lrr
/// Add to register with carry, setting flags
/// Logical shift-right by register
gen_dproc!(arm_adcs_lrr, arm_fn_op2_lrr_s, arm_fn_adc_s);

/// ADCS ari
/// Add to register with carry, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_adcs_ari, arm_fn_op2_ari_s, arm_fn_adc_s);

/// ADCS arr
/// Add to register with carry, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_adcs_arr, arm_fn_op2_arr_s, arm_fn_adc_s);

/// ADCS rri
/// Add to register with carry, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_adcs_rri, arm_fn_op2_rri_s, arm_fn_adc_s);

/// ADCS rrr
/// Add to register with carry, setting flags
/// Rotate right by register
gen_dproc!(arm_adcs_rrr, arm_fn_op2_rrr_s, arm_fn_adc_s);

/// UMLALS
/// Unsigned long multiply and accumulate, setting flags
pub fn arm_umlals(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// SBC lli
/// Subtract from register with borrow
/// Logical shift-left by immediate
gen_dproc!(arm_sbc_lli, arm_fn_op2_lli, arm_fn_sbc);

/// SBC llr
/// Subtract from register with borrow
/// Logical shift-left by register
gen_dproc!(arm_sbc_llr, arm_fn_op2_llr, arm_fn_sbc);

/// SBC lri
/// Subtract from register with borrow
/// Logical shift-right by immediate
gen_dproc!(arm_sbc_lri, arm_fn_op2_lri, arm_fn_sbc);

/// SBC lrr
/// Subtract from register with borrow
/// Logical shift-right by register
gen_dproc!(arm_sbc_lrr, arm_fn_op2_lrr, arm_fn_sbc);

/// SBC ari
/// Subtract from register with borrow
/// Arithmetic shift-right by immediate
gen_dproc!(arm_sbc_ari, arm_fn_op2_ari, arm_fn_sbc);

/// SBC arr
/// Subtract from register with borrow
/// Arithmetic shift-right by register
gen_dproc!(arm_sbc_arr, arm_fn_op2_arr, arm_fn_sbc);

/// SBC rri
/// Subtract from register with borrow
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_sbc_rri, arm_fn_op2_rri, arm_fn_sbc);

/// SBC rrr
/// Subtract from register with borrow
/// Rotate right by register
gen_dproc!(arm_sbc_rrr, arm_fn_op2_rrr, arm_fn_sbc);

/// SMULL
/// Signed long multiply (32x32 to 64)
pub fn arm_smull(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRH ptip
/// Store halfword
/// Immediate offset, post-increment
gen_hdt!(arm_strh_ptip, STRH, POST, INC, HDT_IMM, false);

/// SBCS lli
/// Subtract from register with borrow, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_sbcs_lli, arm_fn_op2_lli_s, arm_fn_sbc_s);

/// SBCS llr
/// Subtract from register with borrow, setting flags
/// Logical shift-left by register
gen_dproc!(arm_sbcs_llr, arm_fn_op2_llr_s, arm_fn_sbc_s);

/// SBCS lri
/// Subtract from register with borrow, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_sbcs_lri, arm_fn_op2_lri_s, arm_fn_sbc_s);

/// SBCS lrr
/// Subtract from register with borrow, setting flags
/// Logical shift-right by register
gen_dproc!(arm_sbcs_lrr, arm_fn_op2_lrr_s, arm_fn_sbc_s);

/// SBCS ari
/// Subtract from register with borrow, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_sbcs_ari, arm_fn_op2_ari_s, arm_fn_sbc_s);

/// SBCS arr
/// Subtract from register with borrow, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_sbcs_arr, arm_fn_op2_arr_s, arm_fn_sbc_s);

/// SBCS rri
/// Subtract from register with borrow, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_sbcs_rri, arm_fn_op2_rri_s, arm_fn_sbc_s);

/// SBCS rrr
/// Subtract from register with borrow, setting flags
/// Rotate right by register
gen_dproc!(arm_sbcs_rrr, arm_fn_op2_rrr_s, arm_fn_sbc_s);

/// SMULLS
/// Signed long multiply, setting flags
pub fn arm_smulls(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRH ptip
/// Load halfword
/// Immediate offset, post-increment
gen_hdt!(arm_ldrh_ptip, LDRH, POST, INC, HDT_IMM, false);

/// LDRSB ptip
/// Load signed byte
/// Immediate offset, post-increment
gen_hdt!(arm_ldrsb_ptip, LDRSB, POST, INC, HDT_IMM, false);

/// LDRSH ptip
/// Load signed halfword
/// Immediate offset, post-increment
gen_hdt!(arm_ldrsh_ptip, LDRSH, POST, INC, HDT_IMM, false);

/// RSC lli
/// Subtract register from value with borrow
/// Logical shift-left by immediate
gen_dproc!(arm_rsc_lli, arm_fn_op2_lli, arm_fn_rsc);

/// RSC llr
/// Subtract register from value with borrow
/// Logical shift-left by register
gen_dproc!(arm_rsc_llr, arm_fn_op2_llr, arm_fn_rsc);

/// RSC lri
/// Subtract register from value with borrow
/// Logical shift-right by immediate
gen_dproc!(arm_rsc_lri, arm_fn_op2_lri, arm_fn_rsc);

/// RSC lrr
/// Subtract register from value with borrow
/// Logical shift-right by register
gen_dproc!(arm_rsc_lrr, arm_fn_op2_lrr, arm_fn_rsc);

/// RSC ari
/// Subtract register from value with borrow
/// Arithmetic shift-right by immediate
gen_dproc!(arm_rsc_ari, arm_fn_op2_ari, arm_fn_rsc);

/// RSC arr
/// Subtract register from value with borrow
/// Arithmetic shift-right by register
gen_dproc!(arm_rsc_arr, arm_fn_op2_arr, arm_fn_rsc);

/// RSC rri
/// Subtract register from value with borrow
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_rsc_rri, arm_fn_op2_rri, arm_fn_rsc);

/// RSC rrr
/// Subtract register from value with borrow
/// Rotate right by register
gen_dproc!(arm_rsc_rrr, arm_fn_op2_rrr, arm_fn_rsc);

/// SMLAL
/// Signed long multiply and accumulate
pub fn arm_smlal(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// RSCS lli
/// Subtract register from value with borrow, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_rscs_lli, arm_fn_op2_lli_s, arm_fn_rsc_s);

/// RSCS llr
/// Subtract register from value with borrow, setting flags
/// Logical shift-left by register
gen_dproc!(arm_rscs_llr, arm_fn_op2_llr_s, arm_fn_rsc_s);

/// RSCS lri
/// Subtract register from value with borrow, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_rscs_lri, arm_fn_op2_lri_s, arm_fn_rsc_s);

/// RSCS lrr
/// Subtract register from value with borrow, setting flags
/// Logical shift-right by register
gen_dproc!(arm_rscs_lrr, arm_fn_op2_lrr_s, arm_fn_rsc_s);

/// RSCS ari
/// Subtract register from value with borrow, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_rscs_ari, arm_fn_op2_ari_s, arm_fn_rsc_s);

/// RSCS arr
/// Subtract register from value with borrow, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_rscs_arr, arm_fn_op2_arr_s, arm_fn_rsc_s);

/// RSCS rri
/// Subtract register from value with borrow, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_rscs_rri, arm_fn_op2_rri_s, arm_fn_rsc_s);

/// RSCS rrr
/// Subtract register from value with borrow, setting flags
/// Rotate right by register
gen_dproc!(arm_rscs_rrr, arm_fn_op2_rrr_s, arm_fn_rsc_s);

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
gen_hdt!(arm_strh_ofrm, STRH, PRE, DEC, HDT_REG, false);

/// TSTS lli
/// Test bits in register (Logical And), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_tsts_lli, arm_fn_op2_lli_s, arm_fn_tst_s);

/// TSTS llr
/// Test bits in register (Logical And), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_tsts_llr, arm_fn_op2_llr_s, arm_fn_tst_s);

/// TSTS lri
/// Test bits in register (Logical And), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_tsts_lri, arm_fn_op2_lri_s, arm_fn_tst_s);

/// TSTS lrr
/// Test bits in register (Logical And), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_tsts_lrr, arm_fn_op2_lrr_s, arm_fn_tst_s);

/// TSTS ari
/// Test bits in register (Logical And), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_tsts_ari, arm_fn_op2_ari_s, arm_fn_tst_s);

/// TSTS arr
/// Test bits in register (Logical And), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_tsts_arr, arm_fn_op2_arr_s, arm_fn_tst_s);

/// TSTS rri
/// Test bits in register (Logical And), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_tsts_rri, arm_fn_op2_rri_s, arm_fn_tst_s);

/// TSTS rrr
/// Test bits in register (Logical And), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_tsts_rrr, arm_fn_op2_rrr_s, arm_fn_tst_s);

/// LDRH ofrm
/// Load halfword
/// Negative register offset
gen_hdt!(arm_ldrh_ofrm, LDRH, PRE, DEC, HDT_REG, false);

/// LDRSB ofrm
/// Load signed byte
/// Negative register offset
gen_hdt!(arm_ldrsb_ofrm, LDRSB, PRE, DEC, HDT_REG, false);

/// LDRSH ofrm
/// Load signed halfword
/// Negative register offset
gen_hdt!(arm_ldrsh_ofrm, LDRSH, PRE, DEC, HDT_REG, false);

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
gen_hdt!(arm_strh_prrm, STRH, PRE, DEC, HDT_REG, true);

/// TEQS lli
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_teqs_lli, arm_fn_op2_lli_s, arm_fn_teq_s);

/// TEQS llr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_teqs_llr, arm_fn_op2_llr_s, arm_fn_teq_s);

/// TEQS lri
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_teqs_lri, arm_fn_op2_lri_s, arm_fn_teq_s);

/// TEQS lrr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_teqs_lrr, arm_fn_op2_lrr_s, arm_fn_teq_s);

/// TEQS ari
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_teqs_ari, arm_fn_op2_ari_s, arm_fn_teq_s);

/// TEQS arr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_teqs_arr, arm_fn_op2_arr_s, arm_fn_teq_s);

/// TEQS rri
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_teqs_rri, arm_fn_op2_rri_s, arm_fn_teq_s);

/// TEQS rrr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_teqs_rrr, arm_fn_op2_rrr_s, arm_fn_teq_s);

/// LDRH prrm
/// Load halfword
/// Register offset, pre-decrement
gen_hdt!(arm_ldrh_prrm, LDRH, PRE, DEC, HDT_REG, true);

/// LDRSB prrm
/// Load signed byte
/// Register offset, pre-decrement
gen_hdt!(arm_ldrsb_prrm, LDRSB, PRE, DEC, HDT_REG, true);

/// LDRSH prrm
/// Load signed halfword
/// Register offset, pre-decrement
gen_hdt!(arm_ldrsh_prrm, LDRSH, PRE, DEC, HDT_REG, true);

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
gen_hdt!(arm_strh_ofim, STRH, PRE, DEC, HDT_IMM, false);

/// CMPS lli
/// Compare register to value (Subtract), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_cmps_lli, arm_fn_op2_lli_s, arm_fn_cmp_s);

/// CMPS llr
/// Compare register to value (Subtract), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_cmps_llr, arm_fn_op2_llr_s, arm_fn_cmp_s);

/// CMPS lri
/// Compare register to value (Subtract), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_cmps_lri, arm_fn_op2_lri_s, arm_fn_cmp_s);

/// CMPS lrr
/// Compare register to value (Subtract), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_cmps_lrr, arm_fn_op2_lrr_s, arm_fn_cmp_s);

/// CMPS ari
/// Compare register to value (Subtract), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_cmps_ari, arm_fn_op2_ari_s, arm_fn_cmp_s);

/// CMPS arr
/// Compare register to value (Subtract), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_cmps_arr, arm_fn_op2_arr_s, arm_fn_cmp_s);

/// CMPS rri
/// Compare register to value (Subtract), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_cmps_rri, arm_fn_op2_rri_s, arm_fn_cmp_s);

/// CMPS rrr
/// Compare register to value (Subtract), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_cmps_rrr, arm_fn_op2_rrr_s, arm_fn_cmp_s);

/// LDRH ofim
/// Load halfword
/// Negative immediate offset
gen_hdt!(arm_ldrh_ofim, LDRH, PRE, DEC, HDT_IMM, false);

/// LDRSB ofim
/// Load signed byte
/// Negative immediate offset
gen_hdt!(arm_ldrsb_ofim, LDRSB, PRE, DEC, HDT_IMM, false);

/// LDRSH ofim
/// Load signed halfword
/// Negative immediate offset
gen_hdt!(arm_ldrsh_ofim, LDRSH, PRE, DEC, HDT_IMM, false);

/// MSR rs
/// Move value to status word
/// Register, SPSR
pub fn arm_msr_rs(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRH prim
/// Store halfword
/// Immediate offset, pre-decrement
gen_hdt!(arm_strh_prim, STRH, PRE, DEC, HDT_IMM, true);

/// CMNS lli
/// Compare register to negation of value (Add), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_cmns_lli, arm_fn_op2_lli_s, arm_fn_cmn_s);

/// CMNS llr
/// Compare register to negation of value (Add), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_cmns_llr, arm_fn_op2_llr_s, arm_fn_cmn_s);

/// CMNS lri
/// Compare register to negation of value (Add), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_cmns_lri, arm_fn_op2_lri_s, arm_fn_cmn_s);

/// CMNS lrr
/// Compare register to negation of value (Add), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_cmns_lrr, arm_fn_op2_lrr_s, arm_fn_cmn_s);

/// CMNS ari
/// Compare register to negation of value (Add), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_cmns_ari, arm_fn_op2_ari_s, arm_fn_cmn_s);

/// CMNS arr
/// Compare register to negation of value (Add), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_cmns_arr, arm_fn_op2_arr_s, arm_fn_cmn_s);

/// CMNS rri
/// Compare register to negation of value (Add), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_cmns_rri, arm_fn_op2_rri_s, arm_fn_cmn_s);

/// CMNS rrr
/// Compare register to negation of value (Add), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_cmns_rrr, arm_fn_op2_rrr_s, arm_fn_cmn_s);

/// LDRH prim
/// Load halfword
/// Immediate offset, pre-decrement
gen_hdt!(arm_ldrh_prim, LDRH, PRE, DEC, HDT_IMM, true);

/// LDRSB prim
/// Load signed byte
/// Immediate offset, pre-decrement
gen_hdt!(arm_ldrsb_prim, LDRSB, PRE, DEC, HDT_IMM, true);

/// LDRSH prim
/// Load signed halfword
/// Immediate offset, pre-decrement
gen_hdt!(arm_ldrsh_prim, LDRSH, PRE, DEC, HDT_IMM, true);

/// ORR lli
/// Logical Or
/// Logical shift-left by immediate
gen_dproc!(arm_orr_lli, arm_fn_op2_lli, arm_fn_orr);

/// ORR llr
/// Logical Or
/// Logical shift-left by register
gen_dproc!(arm_orr_llr, arm_fn_op2_llr, arm_fn_orr);

/// ORR lri
/// Logical Or
/// Logical shift-right by immediate
gen_dproc!(arm_orr_lri, arm_fn_op2_lri, arm_fn_orr);

/// ORR lrr
/// Logical Or
/// Logical shift-right by register
gen_dproc!(arm_orr_lrr, arm_fn_op2_lrr, arm_fn_orr);

/// ORR ari
/// Logical Or
/// Arithmetic shift-right by immediate
gen_dproc!(arm_orr_ari, arm_fn_op2_ari, arm_fn_orr);

/// ORR arr
/// Logical Or
/// Arithmetic shift-right by register
gen_dproc!(arm_orr_arr, arm_fn_op2_arr, arm_fn_orr);

/// ORR rri
/// Logical Or
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_orr_rri, arm_fn_op2_rri, arm_fn_orr);

/// ORR rrr
/// Logical Or
/// Rotate right by register
gen_dproc!(arm_orr_rrr, arm_fn_op2_rrr, arm_fn_orr);

/// STRH ofrp
/// Store halfword
/// Positive register offset
gen_hdt!(arm_strh_ofrp, STRH, PRE, INC, HDT_REG, false);

/// ORRS lli
/// Logical Or, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_orrs_lli, arm_fn_op2_lli_s, arm_fn_orr_s);

/// ORRS llr
/// Logical Or, setting flags
/// Logical shift-left by register
gen_dproc!(arm_orrs_llr, arm_fn_op2_llr_s, arm_fn_orr_s);

/// ORRS lri
/// Logical Or, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_orrs_lri, arm_fn_op2_lri_s, arm_fn_orr_s);

/// ORRS lrr
/// Logical Or, setting flags
/// Logical shift-right by register
gen_dproc!(arm_orrs_lrr, arm_fn_op2_lrr_s, arm_fn_orr_s);

/// ORRS ari
/// Logical Or, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_orrs_ari, arm_fn_op2_ari_s, arm_fn_orr_s);

/// ORRS arr
/// Logical Or, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_orrs_arr, arm_fn_op2_arr_s, arm_fn_orr_s);

/// ORRS rri
/// Logical Or, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_orrs_rri, arm_fn_op2_rri_s, arm_fn_orr_s);

/// ORRS rrr
/// Logical Or, setting flags
/// Rotate right by register
gen_dproc!(arm_orrs_rrr, arm_fn_op2_rrr_s, arm_fn_orr_s);

/// LDRH ofrp
/// Load halfword
/// Positive register offset
gen_hdt!(arm_ldrh_ofrp, LDRH, PRE, INC, HDT_REG, false);

/// LDRSB ofrp
/// Load signed byte
/// Positive register offset
gen_hdt!(arm_ldrsb_ofrp, LDRSB, PRE, INC, HDT_REG, false);

/// LDRSH ofrp
/// Load signed halfword
/// Positive register offset
gen_hdt!(arm_ldrsh_ofrp, LDRSH, PRE, INC, HDT_REG, false);

/// MOV lli
/// Move value to a register
/// Logical shift-left by immediate
gen_dproc!(arm_mov_lli, arm_fn_op2_lli, arm_fn_mov);

/// MOV llr
/// Move value to a register
/// Logical shift-left by register
gen_dproc!(arm_mov_llr, arm_fn_op2_llr, arm_fn_mov);

/// MOV lri
/// Move value to a register
/// Logical shift-right by immediate
gen_dproc!(arm_mov_lri, arm_fn_op2_lri, arm_fn_mov);

/// MOV lrr
/// Move value to a register
/// Logical shift-right by register
gen_dproc!(arm_mov_lrr, arm_fn_op2_lrr, arm_fn_mov);

/// MOV ari
/// Move value to a register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_mov_ari, arm_fn_op2_ari, arm_fn_mov);

/// MOV arr
/// Move value to a register
/// Arithmetic shift-right by register
gen_dproc!(arm_mov_arr, arm_fn_op2_arr, arm_fn_mov);

/// MOV rri
/// Move value to a register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_mov_rri, arm_fn_op2_rri, arm_fn_mov);

/// MOV rrr
/// Move value to a register
/// Rotate right by register
gen_dproc!(arm_mov_rrr, arm_fn_op2_rrr, arm_fn_mov);

/// STRH prrp
/// Store halfword
/// Register offset, pre-increment
gen_hdt!(arm_strh_prrp, STRH, PRE, INC, HDT_REG, true);

/// MOVS lli
/// Move value to a register, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_movs_lli, arm_fn_op2_lli_s, arm_fn_mov_s);

/// MOVS llr
/// Move value to a register, setting flags
/// Logical shift-left by register
gen_dproc!(arm_movs_llr, arm_fn_op2_llr_s, arm_fn_mov_s);

/// MOVS lri
/// Move value to a register, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_movs_lri, arm_fn_op2_lri_s, arm_fn_mov_s);

/// MOVS lrr
/// Move value to a register, setting flags
/// Logical shift-right by register
gen_dproc!(arm_movs_lrr, arm_fn_op2_lrr_s, arm_fn_mov_s);

/// MOVS ari
/// Move value to a register, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_movs_ari, arm_fn_op2_ari_s, arm_fn_mov_s);

/// MOVS arr
/// Move value to a register, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_movs_arr, arm_fn_op2_arr_s, arm_fn_mov_s);

/// MOVS rri
/// Move value to a register, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_movs_rri, arm_fn_op2_rri_s, arm_fn_mov_s);

/// MOVS rrr
/// Move value to a register, setting flags
/// Rotate right by register
gen_dproc!(arm_movs_rrr, arm_fn_op2_rrr_s, arm_fn_mov_s);

/// LDRH prrp
/// Load halfword
/// Register offset, pre-increment
gen_hdt!(arm_ldrh_prrp, LDRH, PRE, INC, HDT_REG, true);

/// LDRSB prrp
/// Load signed byte
/// Register offset, pre-increment
gen_hdt!(arm_ldrsb_prrp, LDRSB, PRE, INC, HDT_REG, true);

/// LDRSH prrp
/// Load signed halfword
/// Register offset, pre-increment
gen_hdt!(arm_ldrsh_prrp, LDRSH, PRE, INC, HDT_REG, true);

/// BIC lli
/// Clear bits in register (NAND)
/// Logical shift-left by immediate
gen_dproc!(arm_bic_lli, arm_fn_op2_lli, arm_fn_bic);

/// BIC llr
/// Clear bits in register (NAND)
/// Logical shift-left by register
gen_dproc!(arm_bic_llr, arm_fn_op2_llr, arm_fn_bic);

/// BIC lri
/// Clear bits in register (NAND)
/// Logical shift-right by immediate
gen_dproc!(arm_bic_lri, arm_fn_op2_lri, arm_fn_bic);

/// BIC lrr
/// Clear bits in register (NAND)
/// Logical shift-right by register
gen_dproc!(arm_bic_lrr, arm_fn_op2_lrr, arm_fn_bic);

/// BIC ari
/// Clear bits in register (NAND)
/// Arithmetic shift-right by immediate
gen_dproc!(arm_bic_ari, arm_fn_op2_ari, arm_fn_bic);

/// BIC arr
/// Clear bits in register (NAND)
/// Arithmetic shift-right by register
gen_dproc!(arm_bic_arr, arm_fn_op2_arr, arm_fn_bic);

/// BIC rri
/// Clear bits in register (NAND)
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_bic_rri, arm_fn_op2_rri, arm_fn_bic);

/// BIC rrr
/// Clear bits in register (NAND)
/// Rotate right by register
gen_dproc!(arm_bic_rrr, arm_fn_op2_rrr, arm_fn_bic);

/// STRH ofip
/// Store halfword
/// Positive immediate offset
gen_hdt!(arm_strh_ofip, STRH, PRE, INC, HDT_IMM, false);

/// BICS lli
/// Clear bits in register (NAND), setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_bics_lli, arm_fn_op2_lli_s, arm_fn_bic_s);

/// BICS llr
/// Clear bits in register (NAND), setting flags
/// Logical shift-left by register
gen_dproc!(arm_bics_llr, arm_fn_op2_llr_s, arm_fn_bic_s);

/// BICS lri
/// Clear bits in register (NAND), setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_bics_lri, arm_fn_op2_lri_s, arm_fn_bic_s);

/// BICS lrr
/// Clear bits in register (NAND), setting flags
/// Logical shift-right by register
gen_dproc!(arm_bics_lrr, arm_fn_op2_lrr_s, arm_fn_bic_s);

/// BICS ari
/// Clear bits in register (NAND), setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_bics_ari, arm_fn_op2_ari_s, arm_fn_bic_s);

/// BICS arr
/// Clear bits in register (NAND), setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_bics_arr, arm_fn_op2_arr_s, arm_fn_bic_s);

/// BICS rri
/// Clear bits in register (NAND), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_bics_rri, arm_fn_op2_rri_s, arm_fn_bic_s);

/// BICS rrr
/// Clear bits in register (NAND), setting flags
/// Rotate right by register
gen_dproc!(arm_bics_rrr, arm_fn_op2_rrr_s, arm_fn_bic_s);

/// LDRH ofip
/// Load halfword
/// Positive immediate offset
gen_hdt!(arm_ldrh_ofip, LDRH, PRE, INC, HDT_IMM, false);

/// LDRSB ofip
/// Load signed byte
/// Positive immediate offset
gen_hdt!(arm_ldrsb_ofip, LDRSB, PRE, INC, HDT_IMM, false);

/// LDRSH ofip
/// Load signed halfword
/// Positive immediate offset
gen_hdt!(arm_ldrsh_ofip, LDRSH, PRE, INC, HDT_IMM, false);

/// MVN lli
/// Move negation of value to a register
/// Logical shift-left by immediate
gen_dproc!(arm_mvn_lli, arm_fn_op2_lli, arm_fn_mvn);

/// MVN llr
/// Move negation of value to a register
/// Logical shift-left by register
gen_dproc!(arm_mvn_llr, arm_fn_op2_llr, arm_fn_mvn);

/// MVN lri
/// Move negation of value to a register
/// Logical shift-right by immediate
gen_dproc!(arm_mvn_lri, arm_fn_op2_lri, arm_fn_mvn);

/// MVN lrr
/// Move negation of value to a register
/// Logical shift-right by register
gen_dproc!(arm_mvn_lrr, arm_fn_op2_lrr, arm_fn_mvn);

/// MVN ari
/// Move negation of value to a register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_mvn_ari, arm_fn_op2_ari, arm_fn_mvn);

/// MVN arr
/// Move negation of value to a register
/// Arithmetic shift-right by register
gen_dproc!(arm_mvn_arr, arm_fn_op2_arr, arm_fn_mvn);

/// MVN rri
/// Move negation of value to a register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_mvn_rri, arm_fn_op2_rri, arm_fn_mvn);

/// MVN rrr
/// Move negation of value to a register
/// Rotate right by register
gen_dproc!(arm_mvn_rrr, arm_fn_op2_rrr, arm_fn_mvn);

/// STRH prip
/// Store halfword
/// Immediate offset, pre-increment
gen_hdt!(arm_strh_prip, STRH, PRE, INC, HDT_IMM, true);

/// MVNS lli
/// Move negation of value to a register, setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_mvns_lli, arm_fn_op2_lli_s, arm_fn_mvn_s);

/// MVNS llr
/// Move negation of value to a register, setting flags
/// Logical shift-left by register
gen_dproc!(arm_mvns_llr, arm_fn_op2_llr_s, arm_fn_mvn_s);

/// MVNS lri
/// Move negation of value to a register, setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_mvns_lri, arm_fn_op2_lri_s, arm_fn_mvn_s);

/// MVNS lrr
/// Move negation of value to a register, setting flags
/// Logical shift-right by register
gen_dproc!(arm_mvns_lrr, arm_fn_op2_lrr_s, arm_fn_mvn_s);

/// MVNS ari
/// Move negation of value to a register, setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_mvns_ari, arm_fn_op2_ari_s, arm_fn_mvn_s);

/// MVNS arr
/// Move negation of value to a register, setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_mvns_arr, arm_fn_op2_arr_s, arm_fn_mvn_s);

/// MVNS rri
/// Move negation of value to a register, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_mvns_rri, arm_fn_op2_rri_s, arm_fn_mvn_s);

/// MVNS rrr
/// Move negation of value to a register, setting flags
/// Rotate right by register
gen_dproc!(arm_mvns_rrr, arm_fn_op2_rrr_s, arm_fn_mvn_s);

/// LDRH prip
/// Load halfword
/// Immediate offset, pre-increment
gen_hdt!(arm_ldrh_prip, LDRH, PRE, INC, HDT_IMM, true);

/// LDRSB prip
/// Load signed byte
/// Immediate offset, pre-increment
gen_hdt!(arm_ldrsb_prip, LDRSB, PRE, INC, HDT_IMM, true);

/// LDRSH prip
/// Load signed halfword
/// Immediate offset, pre-increment
gen_hdt!(arm_ldrsh_prip, LDRSH, PRE, INC, HDT_IMM, true);

/// AND imm
/// Logical And
/// Immediate value
gen_dproc!(arm_and_imm, arm_fn_op2_imm, arm_fn_and);

/// ANDS imm
/// Logical And, setting flags
/// Immediate value
gen_dproc!(arm_ands_imm, arm_fn_op2_imm_s, arm_fn_and_s);

/// EOR imm
/// Logical Exclusive-or
/// Immediate value
gen_dproc!(arm_eor_imm, arm_fn_op2_imm, arm_fn_eor);

/// EORS imm
/// Logical Exclusive-or, setting flags
/// Immediate value
gen_dproc!(arm_eors_imm, arm_fn_op2_imm_s, arm_fn_eor_s);

/// SUB imm
/// Subtract from register
/// Immediate value
gen_dproc!(arm_sub_imm, arm_fn_op2_imm, arm_fn_sub);

/// SUBS imm
/// Subtract, setting flags
/// Immediate value
gen_dproc!(arm_subs_imm, arm_fn_op2_imm_s, arm_fn_sub_s);

/// RSB imm
/// Subtract register from value
/// Immediate value
gen_dproc!(arm_rsb_imm, arm_fn_op2_imm, arm_fn_rsb);

/// RSBS imm
/// Reverse Subtract, setting flags
/// Immediate value
gen_dproc!(arm_rsbs_imm, arm_fn_op2_imm_s, arm_fn_rsb_s);

/// ADD imm
/// Add to register
/// Immediate value
gen_dproc!(arm_add_imm, arm_fn_op2_imm, arm_fn_add);

/// ADDS imm
/// Add to register, setting flags
/// Immediate value
gen_dproc!(arm_adds_imm, arm_fn_op2_imm_s, arm_fn_add_s);

/// ADC imm
/// Add to register with carry
/// Immediate value
gen_dproc!(arm_adc_imm, arm_fn_op2_imm, arm_fn_adc);

/// ADCS imm
/// Add to register with carry, setting flags
/// Immediate value
gen_dproc!(arm_adcs_imm, arm_fn_op2_imm_s, arm_fn_adc_s);

/// SBC imm
/// Subtract from register with borrow
/// Immediate value
gen_dproc!(arm_sbc_imm, arm_fn_op2_imm, arm_fn_sbc);

/// SBCS imm
/// Subtract from register with borrow, setting flags
/// Immediate value
gen_dproc!(arm_sbcs_imm, arm_fn_op2_imm_s, arm_fn_sbc_s);

/// RSC imm
/// Subtract register from value with borrow
/// Immediate value
gen_dproc!(arm_rsc_imm, arm_fn_op2_imm, arm_fn_rsc);

/// RSCS imm
/// Subtract register from value with borrow, setting flags
/// Immediate value
gen_dproc!(arm_rscs_imm, arm_fn_op2_imm_s, arm_fn_rsc_s);

/// TSTS imm
/// Test bits in register (Logical And), setting flags
/// Immediate value
gen_dproc_nw!(arm_tsts_imm, arm_fn_op2_imm_s, arm_fn_tst_s);

/// MSR ic
/// Move value to status word
/// Immediate, CPSR
pub fn arm_msr_ic(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// TEQS imm
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Immediate value
gen_dproc_nw!(arm_teqs_imm, arm_fn_op2_imm_s, arm_fn_teq_s);

/// CMPS imm
/// Compare register to value (Subtract), setting flags
/// Immediate value
gen_dproc_nw!(arm_cmps_imm, arm_fn_op2_imm_s, arm_fn_cmp_s);

/// MSR is
/// Move value to status word
/// Immediate, SPSR
pub fn arm_msr_is(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// CMNS imm
/// Compare register to negation of value (Add), setting flags
/// Immediate value
gen_dproc_nw!(arm_cmns_imm, arm_fn_op2_imm_s, arm_fn_cmn_s);

/// ORR imm
/// Logical Or
/// Immediate value
gen_dproc!(arm_orr_imm, arm_fn_op2_imm, arm_fn_orr);

/// ORRS imm
/// Logical Or, setting flags
/// Immediate value
gen_dproc!(arm_orrs_imm, arm_fn_op2_imm_s, arm_fn_orr_s);

/// MOV imm
/// Move value to a register
/// Immediate value
gen_dproc!(arm_mov_imm, arm_fn_op2_imm, arm_fn_mov);

/// MOVS imm
/// Move value to a register, setting flags
/// Immediate value
gen_dproc!(arm_movs_imm, arm_fn_op2_imm_s, arm_fn_mov_s);

/// BIC imm
/// Clear bits in register (NAND)
/// Immediate value
gen_dproc!(arm_bic_imm, arm_fn_op2_imm, arm_fn_bic);

/// BICS imm
/// Clear bits in register (NAND), setting flags
/// Immediate value
gen_dproc!(arm_bics_imm, arm_fn_op2_imm_s, arm_fn_bic_s);

/// MVN imm
/// Move negation of value to a register
/// Immediate value
gen_dproc!(arm_mvn_imm, arm_fn_op2_imm, arm_fn_mvn);

/// MVNS imm
/// Move negation of value to a register, setting flags
/// Immediate value
gen_dproc!(arm_mvns_imm, arm_fn_op2_imm_s, arm_fn_mvn_s);

/// STR ptim
gen_sdt!(arm_str_ptim, STR, POST, DEC, SDT_IMM, false, false);

/// LDR ptim
gen_sdt!(arm_ldr_ptim, LDR, POST, DEC, SDT_IMM, false, false);

/// STRT ptim
gen_sdt!(arm_strt_ptim, STR, POST, DEC, SDT_IMM, false, true);

/// LDRT ptim
gen_sdt!(arm_ldrt_ptim, LDR, POST, DEC, SDT_IMM, false, true);

/// STRB ptim
gen_sdt!(arm_strb_ptim, STRB, POST, DEC, SDT_IMM, false, false);

/// LDRB ptim
gen_sdt!(arm_ldrb_ptim, LDRB, POST, DEC, SDT_IMM, false, false);

/// STRBT ptim
gen_sdt!(arm_strbt_ptim, STRB, POST, DEC, SDT_IMM, false, true);

/// LDRBT ptim
gen_sdt!(arm_ldrbt_ptim, LDRB, POST, DEC, SDT_IMM, false, true);

/// STR ptip
gen_sdt!(arm_str_ptip, STR, POST, INC, SDT_IMM, false, false);

/// LDR ptip
gen_sdt!(arm_ldr_ptip, LDR, POST, INC, SDT_IMM, false, false);

/// STRT ptip
gen_sdt!(arm_strt_ptip, STR, POST, INC, SDT_IMM, false, true);

/// LDRT ptip
gen_sdt!(arm_ldrt_ptip, LDR, POST, INC, SDT_IMM, false, true);

/// STRB ptip
gen_sdt!(arm_strb_ptip, STRB, POST, INC, SDT_IMM, false, false);

/// LDRB ptip
gen_sdt!(arm_ldrb_ptip, LDRB, POST, INC, SDT_IMM, false, false);

/// STRBT ptip
gen_sdt!(arm_strbt_ptip, STRB, POST, INC, SDT_IMM, false, true);

/// LDRBT ptip
gen_sdt!(arm_ldrbt_ptip, LDRB, POST, INC, SDT_IMM, false, true);

/// STR ofim
gen_sdt!(arm_str_ofim, STR, PRE, DEC, SDT_NEG_IMM, false, false);

/// LDR ofim
gen_sdt!(arm_ldr_ofim, LDR, PRE, DEC, SDT_NEG_IMM, false, false);

/// STR prim
gen_sdt!(arm_str_prim, STR, PRE, DEC, SDT_IMM, true, false);

/// LDR prim
gen_sdt!(arm_ldr_prim, LDR, PRE, DEC, SDT_IMM, true, false);

/// STRB ofim
gen_sdt!(arm_strb_ofim, STRB, PRE, DEC, SDT_NEG_IMM, false, false);

/// LDRB ofim
gen_sdt!(arm_ldrb_ofim, LDRB, PRE, DEC, SDT_NEG_IMM, false, false);

/// STRB prim
gen_sdt!(arm_strb_prim, STRB, PRE, DEC, SDT_IMM, true, false);

/// LDRB prim
gen_sdt!(arm_ldrb_prim, LDRB, PRE, DEC, SDT_IMM, true, false);

/// STR ofip
gen_sdt!(arm_str_ofip, STR, PRE, INC, SDT_POS_IMM, false, false);

/// LDR ofip
gen_sdt!(arm_ldr_ofip, LDR, PRE, INC, SDT_POS_IMM, false, false);

/// STR prip
gen_sdt!(arm_str_prip, STR, PRE, INC, SDT_IMM, true, false);

/// LDR prip
gen_sdt!(arm_ldr_prip, LDR, PRE, INC, SDT_IMM, true, false);

/// STRB ofip
gen_sdt!(arm_strb_ofip, STRB, PRE, INC, SDT_POS_IMM, false, false);

/// LDRB ofip
gen_sdt!(arm_ldrb_ofip, LDRB, PRE, INC, SDT_POS_IMM, false, false);

/// STRB prip
gen_sdt!(arm_strb_prip, STRB, PRE, INC, SDT_IMM, true, false);

/// LDRB prip
gen_sdt!(arm_ldrb_prip, LDRB, PRE, INC, SDT_IMM, true, false);

/// STR ptrmll
gen_sdt!(arm_str_ptrmll, STR, POST, DEC, SDT_LSL, false, false);

/// STR ptrmlr
gen_sdt!(arm_str_ptrmlr, STR, POST, DEC, SDT_LSR, false, false);

/// STR ptrmar
gen_sdt!(arm_str_ptrmar, STR, POST, DEC, SDT_ASR, false, false);

/// STR ptrmrr
gen_sdt!(arm_str_ptrmrr, STR, POST, DEC, SDT_ROR, false, false);

/// LDR ptrmll
gen_sdt!(arm_ldr_ptrmll, LDR, POST, DEC, SDT_LSL, false, false);

/// LDR ptrmlr
gen_sdt!(arm_ldr_ptrmlr, LDR, POST, DEC, SDT_LSR, false, false);

/// LDR ptrmar
gen_sdt!(arm_ldr_ptrmar, LDR, POST, DEC, SDT_ASR, false, false);

/// LDR ptrmrr
gen_sdt!(arm_ldr_ptrmrr, LDR, POST, DEC, SDT_ROR, false, false);

/// STRT ptrmll
gen_sdt!(arm_strt_ptrmll, STR, POST, DEC, SDT_LSL, false, true);

/// STRT ptrmlr
gen_sdt!(arm_strt_ptrmlr, STR, POST, DEC, SDT_LSR, false, true);

/// STRT ptrmar
gen_sdt!(arm_strt_ptrmar, STR, POST, DEC, SDT_ASR, false, true);

/// STRT ptrmrr
gen_sdt!(arm_strt_ptrmrr, STR, POST, DEC, SDT_ROR, false, true);

/// LDRT ptrmll
gen_sdt!(arm_ldrt_ptrmll, LDR, POST, DEC, SDT_LSL, false, true);

/// LDRT ptrmlr
gen_sdt!(arm_ldrt_ptrmlr, LDR, POST, DEC, SDT_LSR, false, true);

/// LDRT ptrmar
gen_sdt!(arm_ldrt_ptrmar, LDR, POST, DEC, SDT_ASR, false, true);

/// LDRT ptrmrr
gen_sdt!(arm_ldrt_ptrmrr, LDR, POST, DEC, SDT_ROR, false, true);

/// STRB ptrmll
gen_sdt!(arm_strb_ptrmll, STRB, POST, DEC, SDT_LSL, false, false);

/// STRB ptrmlr
gen_sdt!(arm_strb_ptrmlr, STRB, POST, DEC, SDT_LSR, false, false);

/// STRB ptrmar
gen_sdt!(arm_strb_ptrmar, STRB, POST, DEC, SDT_ASR, false, false);

/// STRB ptrmrr
gen_sdt!(arm_strb_ptrmrr, STRB, POST, DEC, SDT_ROR, false, false);

/// LDRB ptrmll
gen_sdt!(arm_ldrb_ptrmll, LDRB, POST, DEC, SDT_LSL, false, false);

/// LDRB ptrmlr
gen_sdt!(arm_ldrb_ptrmlr, LDRB, POST, DEC, SDT_LSR, false, false);

/// LDRB ptrmar
gen_sdt!(arm_ldrb_ptrmar, LDRB, POST, DEC, SDT_ASR, false, false);

/// LDRB ptrmrr
gen_sdt!(arm_ldrb_ptrmrr, LDRB, POST, DEC, SDT_ROR, false, false);

/// STRBT ptrmll
gen_sdt!(arm_strbt_ptrmll, STRB, POST, DEC, SDT_LSL, false, true);

/// STRBT ptrmlr
gen_sdt!(arm_strbt_ptrmlr, STRB, POST, DEC, SDT_LSR, false, true);

/// STRBT ptrmar
gen_sdt!(arm_strbt_ptrmar, STRB, POST, DEC, SDT_ASR, false, true);

/// STRBT ptrmrr
gen_sdt!(arm_strbt_ptrmrr, STRB, POST, DEC, SDT_ROR, false, true);

/// LDRBT ptrmll
gen_sdt!(arm_ldrbt_ptrmll, LDRB, POST, DEC, SDT_LSL, false, true);

/// LDRBT ptrmlr
gen_sdt!(arm_ldrbt_ptrmlr, LDRB, POST, DEC, SDT_LSR, false, true);

/// LDRBT ptrmar
gen_sdt!(arm_ldrbt_ptrmar, LDRB, POST, DEC, SDT_ASR, false, true);

/// LDRBT ptrmrr
gen_sdt!(arm_ldrbt_ptrmrr, LDRB, POST, DEC, SDT_ROR, false, true);

/// STR ptrpll
gen_sdt!(arm_str_ptrpll, STR, POST, INC, SDT_LSL, false, false);

/// STR ptrplr
gen_sdt!(arm_str_ptrplr, STR, POST, INC, SDT_LSR, false, false);

/// STR ptrpar
gen_sdt!(arm_str_ptrpar, STR, POST, INC, SDT_ASR, false, false);

/// STR ptrprr
gen_sdt!(arm_str_ptrprr, STR, POST, INC, SDT_ROR, false, false);

/// LDR ptrpll
gen_sdt!(arm_ldr_ptrpll, LDR, POST, INC, SDT_LSL, false, false);

/// LDR ptrplr
gen_sdt!(arm_ldr_ptrplr, LDR, POST, INC, SDT_LSR, false, false);

/// LDR ptrpar
gen_sdt!(arm_ldr_ptrpar, LDR, POST, INC, SDT_ASR, false, false);

/// LDR ptrprr
gen_sdt!(arm_ldr_ptrprr, LDR, POST, INC, SDT_ROR, false, false);

/// STRT ptrpll
gen_sdt!(arm_strt_ptrpll, STR, POST, INC, SDT_LSL, false, true);

/// STRT ptrplr
gen_sdt!(arm_strt_ptrplr, STR, POST, INC, SDT_LSR, false, true);

/// STRT ptrpar
gen_sdt!(arm_strt_ptrpar, STR, POST, INC, SDT_ASR, false, true);

/// STRT ptrprr
gen_sdt!(arm_strt_ptrprr, STR, POST, INC, SDT_ROR, false, true);

/// LDRT ptrpll
gen_sdt!(arm_ldrt_ptrpll, LDR, POST, INC, SDT_LSL, false, true);

/// LDRT ptrplr
gen_sdt!(arm_ldrt_ptrplr, LDR, POST, INC, SDT_LSR, false, true);

/// LDRT ptrpar
gen_sdt!(arm_ldrt_ptrpar, LDR, POST, INC, SDT_ASR, false, true);

/// LDRT ptrprr
gen_sdt!(arm_ldrt_ptrprr, LDR, POST, INC, SDT_ROR, false, true);

/// STRB ptrpll
gen_sdt!(arm_strb_ptrpll, STRB, POST, INC, SDT_LSL, false, false);

/// STRB ptrplr
gen_sdt!(arm_strb_ptrplr, STRB, POST, INC, SDT_LSR, false, false);

/// STRB ptrpar
gen_sdt!(arm_strb_ptrpar, STRB, POST, INC, SDT_ASR, false, false);

/// STRB ptrprr
gen_sdt!(arm_strb_ptrprr, STRB, POST, INC, SDT_ROR, false, false);

/// LDRB ptrpll
gen_sdt!(arm_ldrb_ptrpll, LDRB, POST, INC, SDT_LSL, false, false);

/// LDRB ptrplr
gen_sdt!(arm_ldrb_ptrplr, LDRB, POST, INC, SDT_LSR, false, false);

/// LDRB ptrpar
gen_sdt!(arm_ldrb_ptrpar, LDRB, POST, INC, SDT_ASR, false, false);

/// LDRB ptrprr
gen_sdt!(arm_ldrb_ptrprr, LDRB, POST, INC, SDT_ROR, false, false);

/// STRBT ptrpll
gen_sdt!(arm_strbt_ptrpll, STRB, POST, INC, SDT_LSL, false, true);

/// STRBT ptrplr
gen_sdt!(arm_strbt_ptrplr, STRB, POST, INC, SDT_LSR, false, true);

/// STRBT ptrpar
gen_sdt!(arm_strbt_ptrpar, STRB, POST, INC, SDT_ASR, false, true);

/// STRBT ptrprr
gen_sdt!(arm_strbt_ptrprr, STRB, POST, INC, SDT_ROR, false, true);

/// LDRBT ptrpll
gen_sdt!(arm_ldrbt_ptrpll, LDRB, POST, INC, SDT_LSL, false, true);

/// LDRBT ptrplr
gen_sdt!(arm_ldrbt_ptrplr, LDRB, POST, INC, SDT_LSR, false, true);

/// LDRBT ptrpar
gen_sdt!(arm_ldrbt_ptrpar, LDRB, POST, INC, SDT_ASR, false, true);

/// LDRBT ptrprr
gen_sdt!(arm_ldrbt_ptrprr, LDRB, POST, INC, SDT_ROR, false, true);

/// STR ofrmll
gen_sdt!(arm_str_ofrmll, STR, PRE, DEC, SDT_NEG_LSL, false, false);

/// STR ofrmlr
gen_sdt!(arm_str_ofrmlr, STR, PRE, DEC, SDT_NEG_LSR, false, false);

/// STR ofrmar
gen_sdt!(arm_str_ofrmar, STR, PRE, DEC, SDT_NEG_ASR, false, false);

/// STR ofrmrr
gen_sdt!(arm_str_ofrmrr, STR, PRE, DEC, SDT_NEG_ROR, false, false);

/// LDR ofrmll
gen_sdt!(arm_ldr_ofrmll, LDR, PRE, DEC, SDT_NEG_LSL, false, false);

/// LDR ofrmlr
gen_sdt!(arm_ldr_ofrmlr, LDR, PRE, DEC, SDT_NEG_LSR, false, false);

/// LDR ofrmar
gen_sdt!(arm_ldr_ofrmar, LDR, PRE, DEC, SDT_NEG_ASR, false, false);

/// LDR ofrmrr
gen_sdt!(arm_ldr_ofrmrr, LDR, PRE, DEC, SDT_NEG_ROR, false, false);

/// STR prrmll
gen_sdt!(arm_str_prrmll, STR, PRE, DEC, SDT_LSL, true, false);

/// STR prrmlr
gen_sdt!(arm_str_prrmlr, STR, PRE, DEC, SDT_LSR, true, false);

/// STR prrmar
gen_sdt!(arm_str_prrmar, STR, PRE, DEC, SDT_ASR, true, false);

/// STR prrmrr
gen_sdt!(arm_str_prrmrr, STR, PRE, DEC, SDT_ROR, true, false);

/// LDR prrmll
gen_sdt!(arm_ldr_prrmll, LDR, PRE, DEC, SDT_LSL, true, false);

/// LDR prrmlr
gen_sdt!(arm_ldr_prrmlr, LDR, PRE, DEC, SDT_LSR, true, false);

/// LDR prrmar
gen_sdt!(arm_ldr_prrmar, LDR, PRE, DEC, SDT_ASR, true, false);

/// LDR prrmrr
gen_sdt!(arm_ldr_prrmrr, LDR, PRE, DEC, SDT_ROR, true, false);

/// STRB ofrmll
gen_sdt!(arm_strb_ofrmll, STRB, PRE, DEC, SDT_NEG_LSL, false, false);

/// STRB ofrmlr
gen_sdt!(arm_strb_ofrmlr, STRB, PRE, DEC, SDT_NEG_LSR, false, false);

/// STRB ofrmar
gen_sdt!(arm_strb_ofrmar, STRB, PRE, DEC, SDT_NEG_ASR, false, false);

/// STRB ofrmrr
gen_sdt!(arm_strb_ofrmrr, STRB, PRE, DEC, SDT_NEG_ROR, false, false);

/// LDRB ofrmll
gen_sdt!(arm_ldrb_ofrmll, LDRB, PRE, DEC, SDT_NEG_LSL, false, false);

/// LDRB ofrmlr
gen_sdt!(arm_ldrb_ofrmlr, LDRB, PRE, DEC, SDT_NEG_LSR, false, false);

/// LDRB ofrmar
gen_sdt!(arm_ldrb_ofrmar, LDRB, PRE, DEC, SDT_NEG_ASR, false, false);

/// LDRB ofrmrr
gen_sdt!(arm_ldrb_ofrmrr, LDRB, PRE, DEC, SDT_NEG_ROR, false, false);

/// STRB prrmll
gen_sdt!(arm_strb_prrmll, STRB, PRE, DEC, SDT_LSL, true, false);

/// STRB prrmlr
gen_sdt!(arm_strb_prrmlr, STRB, PRE, DEC, SDT_LSR, true, false);

/// STRB prrmar
gen_sdt!(arm_strb_prrmar, STRB, PRE, DEC, SDT_ASR, true, false);

/// STRB prrmrr
gen_sdt!(arm_strb_prrmrr, STRB, PRE, DEC, SDT_ROR, true, false);

/// LDRB prrmll
gen_sdt!(arm_ldrb_prrmll, LDRB, PRE, DEC, SDT_LSL, true, false);

/// LDRB prrmlr
gen_sdt!(arm_ldrb_prrmlr, LDRB, PRE, DEC, SDT_LSR, true, false);

/// LDRB prrmar
gen_sdt!(arm_ldrb_prrmar, LDRB, PRE, DEC, SDT_ASR, true, false);

/// LDRB prrmrr
gen_sdt!(arm_ldrb_prrmrr, LDRB, PRE, DEC, SDT_ROR, true, false);

/// STR ofrpll
gen_sdt!(arm_str_ofrpll, STR, PRE, INC, SDT_POS_LSL, false, false);

/// STR ofrplr
gen_sdt!(arm_str_ofrplr, STR, PRE, INC, SDT_POS_LSR, false, false);

/// STR ofrpar
gen_sdt!(arm_str_ofrpar, STR, PRE, INC, SDT_POS_ASR, false, false);

/// STR ofrprr
gen_sdt!(arm_str_ofrprr, STR, PRE, INC, SDT_POS_ROR, false, false);

/// LDR ofrpll
gen_sdt!(arm_ldr_ofrpll, LDR, PRE, INC, SDT_POS_LSL, false, false);

/// LDR ofrplr
gen_sdt!(arm_ldr_ofrplr, LDR, PRE, INC, SDT_POS_LSR, false, false);

/// LDR ofrpar
gen_sdt!(arm_ldr_ofrpar, LDR, PRE, INC, SDT_POS_ASR, false, false);

/// LDR ofrprr
gen_sdt!(arm_ldr_ofrprr, LDR, PRE, INC, SDT_POS_ROR, false, false);

/// STR prrpll
gen_sdt!(arm_str_prrpll, STR, PRE, INC, SDT_LSL, true, false);

/// STR prrplr
gen_sdt!(arm_str_prrplr, STR, PRE, INC, SDT_LSR, true, false);

/// STR prrpar
gen_sdt!(arm_str_prrpar, STR, PRE, INC, SDT_ASR, true, false);

/// STR prrprr
gen_sdt!(arm_str_prrprr, STR, PRE, INC, SDT_ROR, true, false);

/// LDR prrpll
gen_sdt!(arm_ldr_prrpll, LDR, PRE, INC, SDT_LSL, true, false);

/// LDR prrplr
gen_sdt!(arm_ldr_prrplr, LDR, PRE, INC, SDT_LSR, true, false);

/// LDR prrpar
gen_sdt!(arm_ldr_prrpar, LDR, PRE, INC, SDT_ASR, true, false);

/// LDR prrprr
gen_sdt!(arm_ldr_prrprr, LDR, PRE, INC, SDT_ROR, true, false);

/// STRB ofrpll
gen_sdt!(arm_strb_ofrpll, STRB, PRE, INC, SDT_POS_LSL, false, false);

/// STRB ofrplr
gen_sdt!(arm_strb_ofrplr, STRB, PRE, INC, SDT_POS_LSR, false, false);

/// STRB ofrpar
gen_sdt!(arm_strb_ofrpar, STRB, PRE, INC, SDT_POS_ASR, false, false);

/// STRB ofrprr
gen_sdt!(arm_strb_ofrprr, STRB, PRE, INC, SDT_POS_ROR, false, false);

/// LDRB ofrpll
gen_sdt!(arm_ldrb_ofrpll, LDRB, PRE, INC, SDT_POS_LSL, false, false);

/// LDRB ofrplr
gen_sdt!(arm_ldrb_ofrplr, LDRB, PRE, INC, SDT_POS_LSR, false, false);

/// LDRB ofrpar
gen_sdt!(arm_ldrb_ofrpar, LDRB, PRE, INC, SDT_POS_ASR, false, false);

/// LDRB ofrprr
gen_sdt!(arm_ldrb_ofrprr, LDRB, PRE, INC, SDT_POS_ROR, false, false);

/// STRB prrpll
gen_sdt!(arm_strb_prrpll, STRB, PRE, INC, SDT_LSL, true, false);

/// STRB prrplr
gen_sdt!(arm_strb_prrplr, STRB, PRE, INC, SDT_LSR, true, false);

/// STRB prrpar
gen_sdt!(arm_strb_prrpar, STRB, PRE, INC, SDT_ASR, true, false);

/// STRB prrprr
gen_sdt!(arm_strb_prrprr, STRB, PRE, INC, SDT_ROR, true, false);

/// LDRB prrpll
gen_sdt!(arm_ldrb_prrpll, LDRB, PRE, INC, SDT_LSL, true, false);

/// LDRB prrplr
gen_sdt!(arm_ldrb_prrplr, LDRB, PRE, INC, SDT_LSR, true, false);

/// LDRB prrpar
gen_sdt!(arm_ldrb_prrpar, LDRB, PRE, INC, SDT_ASR, true, false);

/// LDRB prrprr
gen_sdt!(arm_ldrb_prrprr, LDRB, PRE, INC, SDT_ROR, true, false);

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

