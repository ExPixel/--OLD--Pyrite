use super::super::ArmCpu;
use super::super::super::memory::GbaMemory;
use super::functions::*;

const ARM_REG_MASK: u32 = 0xf;


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
    		let rn = (instr >> 16) & ARM_REG_MASK;
    		let rn_value = cpu.rget(rn);
    		let operand2 = $operand2_function(cpu, instr);
    		let result = $function(cpu, rn_value, operand2);
    		let rd = (instr >> 12) & ARM_REG_MASK;
    		cpu.rset(rd, result);
    	}
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
pub fn arm_strh_ptrm(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_ptrp(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_ofrm(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// TSTS lli
/// Test bits in register (Logical And), setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_tsts_lli, arm_fn_op2_lli_s, arm_fn_tst_s);

/// TSTS llr
/// Test bits in register (Logical And), setting flags
/// Logical shift-left by register
gen_dproc!(arm_tsts_llr, arm_fn_op2_llr_s, arm_fn_tst_s);

/// TSTS lri
/// Test bits in register (Logical And), setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_tsts_lri, arm_fn_op2_lri_s, arm_fn_tst_s);

/// TSTS lrr
/// Test bits in register (Logical And), setting flags
/// Logical shift-right by register
gen_dproc!(arm_tsts_lrr, arm_fn_op2_lrr_s, arm_fn_tst_s);

/// TSTS ari
/// Test bits in register (Logical And), setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_tsts_ari, arm_fn_op2_ari_s, arm_fn_tst_s);

/// TSTS arr
/// Test bits in register (Logical And), setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_tsts_arr, arm_fn_op2_arr_s, arm_fn_tst_s);

/// TSTS rri
/// Test bits in register (Logical And), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_tsts_rri, arm_fn_op2_rri_s, arm_fn_tst_s);

/// TSTS rrr
/// Test bits in register (Logical And), setting flags
/// Rotate right by register
gen_dproc!(arm_tsts_rrr, arm_fn_op2_rrr_s, arm_fn_tst_s);

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

/// TEQS lli
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_teqs_lli, arm_fn_op2_lli_s, arm_fn_teq_s);

/// TEQS llr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-left by register
gen_dproc!(arm_teqs_llr, arm_fn_op2_llr_s, arm_fn_teq_s);

/// TEQS lri
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_teqs_lri, arm_fn_op2_lri_s, arm_fn_teq_s);

/// TEQS lrr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-right by register
gen_dproc!(arm_teqs_lrr, arm_fn_op2_lrr_s, arm_fn_teq_s);

/// TEQS ari
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_teqs_ari, arm_fn_op2_ari_s, arm_fn_teq_s);

/// TEQS arr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_teqs_arr, arm_fn_op2_arr_s, arm_fn_teq_s);

/// TEQS rri
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_teqs_rri, arm_fn_op2_rri_s, arm_fn_teq_s);

/// TEQS rrr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Rotate right by register
gen_dproc!(arm_teqs_rrr, arm_fn_op2_rrr_s, arm_fn_teq_s);

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

/// CMPS lli
/// Compare register to value (Subtract), setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_cmps_lli, arm_fn_op2_lli_s, arm_fn_cmp_s);

/// CMPS llr
/// Compare register to value (Subtract), setting flags
/// Logical shift-left by register
gen_dproc!(arm_cmps_llr, arm_fn_op2_llr_s, arm_fn_cmp_s);

/// CMPS lri
/// Compare register to value (Subtract), setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_cmps_lri, arm_fn_op2_lri_s, arm_fn_cmp_s);

/// CMPS lrr
/// Compare register to value (Subtract), setting flags
/// Logical shift-right by register
gen_dproc!(arm_cmps_lrr, arm_fn_op2_lrr_s, arm_fn_cmp_s);

/// CMPS ari
/// Compare register to value (Subtract), setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_cmps_ari, arm_fn_op2_ari_s, arm_fn_cmp_s);

/// CMPS arr
/// Compare register to value (Subtract), setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_cmps_arr, arm_fn_op2_arr_s, arm_fn_cmp_s);

/// CMPS rri
/// Compare register to value (Subtract), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_cmps_rri, arm_fn_op2_rri_s, arm_fn_cmp_s);

/// CMPS rrr
/// Compare register to value (Subtract), setting flags
/// Rotate right by register
gen_dproc!(arm_cmps_rrr, arm_fn_op2_rrr_s, arm_fn_cmp_s);

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

/// CMNS lli
/// Compare register to negation of value (Add), setting flags
/// Logical shift-left by immediate
gen_dproc!(arm_cmns_lli, arm_fn_op2_lli_s, arm_fn_cmn_s);

/// CMNS llr
/// Compare register to negation of value (Add), setting flags
/// Logical shift-left by register
gen_dproc!(arm_cmns_llr, arm_fn_op2_llr_s, arm_fn_cmn_s);

/// CMNS lri
/// Compare register to negation of value (Add), setting flags
/// Logical shift-right by immediate
gen_dproc!(arm_cmns_lri, arm_fn_op2_lri_s, arm_fn_cmn_s);

/// CMNS lrr
/// Compare register to negation of value (Add), setting flags
/// Logical shift-right by register
gen_dproc!(arm_cmns_lrr, arm_fn_op2_lrr_s, arm_fn_cmn_s);

/// CMNS ari
/// Compare register to negation of value (Add), setting flags
/// Arithmetic shift-right by immediate
gen_dproc!(arm_cmns_ari, arm_fn_op2_ari_s, arm_fn_cmn_s);

/// CMNS arr
/// Compare register to negation of value (Add), setting flags
/// Arithmetic shift-right by register
gen_dproc!(arm_cmns_arr, arm_fn_op2_arr_s, arm_fn_cmn_s);

/// CMNS rri
/// Compare register to negation of value (Add), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_cmns_rri, arm_fn_op2_rri_s, arm_fn_cmn_s);

/// CMNS rrr
/// Compare register to negation of value (Add), setting flags
/// Rotate right by register
gen_dproc!(arm_cmns_rrr, arm_fn_op2_rrr_s, arm_fn_cmn_s);

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
pub fn arm_strh_ofrp(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_prrp(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_ofip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
pub fn arm_strh_prip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

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
gen_dproc!(arm_tsts_imm, arm_fn_op2_imm_s, arm_fn_tst_s);

/// MSR ic
/// Move value to status word
/// Immediate, CPSR
pub fn arm_msr_ic(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// TEQS imm
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Immediate value
gen_dproc!(arm_teqs_imm, arm_fn_op2_imm_s, arm_fn_teq_s);

/// CMPS imm
/// Compare register to value (Subtract), setting flags
/// Immediate value
gen_dproc!(arm_cmps_imm, arm_fn_op2_imm_s, arm_fn_cmp_s);

/// MSR is
/// Move value to status word
/// Immediate, SPSR
pub fn arm_msr_is(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// CMNS imm
/// Compare register to negation of value (Add), setting flags
/// Immediate value
gen_dproc!(arm_cmns_imm, arm_fn_op2_imm_s, arm_fn_cmn_s);

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
/// Store word
/// Immediate offset, post-decrement
pub fn arm_str_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptim
/// Load word
/// Immediate offset, post-decrement
pub fn arm_ldr_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptim
/// Store word from user-mode register
/// Immediate offset, post-decrement
pub fn arm_strt_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptim
/// Load word into user-mode register
/// Immediate offset, post-decrement
pub fn arm_ldrt_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptim
/// Store byte
/// Immediate offset, post-decrement
pub fn arm_strb_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptim
/// Load byte
/// Immediate offset, post-decrement
pub fn arm_ldrb_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptim
/// Store byte from user-mode register
/// Immediate offset, post-decrement
pub fn arm_strbt_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptim
/// Load byte into user-mode register
/// Immediate offset, post-decrement
pub fn arm_ldrbt_ptim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptip
/// Store word
/// Immediate offset, post-increment
pub fn arm_str_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptip
/// Load word
/// Immediate offset, post-increment
pub fn arm_ldr_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptip
/// Store word from user-mode register
/// Immediate offset, post-increment
pub fn arm_strt_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptip
/// Load word into user-mode register
/// Immediate offset, post-increment
pub fn arm_ldrt_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptip
/// Store byte
/// Immediate offset, post-increment
pub fn arm_strb_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptip
/// Load byte
/// Immediate offset, post-increment
pub fn arm_ldrb_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptip
/// Store byte from user-mode register
/// Immediate offset, post-increment
pub fn arm_strbt_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptip
/// Load byte into user-mode register
/// Immediate offset, post-increment
pub fn arm_ldrbt_ptip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofim
/// Store word
/// Negative immediate offset
pub fn arm_str_ofim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofim
/// Load word
/// Negative immediate offset
pub fn arm_ldr_ofim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prim
/// Store word
/// Immediate offset, pre-decrement
pub fn arm_str_prim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prim
/// Load word
/// Immediate offset, pre-decrement
pub fn arm_ldr_prim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofim
/// Store byte
/// Negative immediate offset
pub fn arm_strb_ofim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofim
/// Load byte
/// Negative immediate offset
pub fn arm_ldrb_ofim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prim
/// Store byte
/// Immediate offset, pre-decrement
pub fn arm_strb_prim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prim
/// Load byte
/// Immediate offset, pre-decrement
pub fn arm_ldrb_prim(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofip
/// Store word
/// Positive immediate offset
pub fn arm_str_ofip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofip
/// Load word
/// Positive immediate offset
pub fn arm_ldr_ofip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prip
/// Store word
/// Immediate offset, pre-increment
pub fn arm_str_prip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prip
/// Load word
/// Immediate offset, pre-increment
pub fn arm_ldr_prip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofip
/// Store byte
/// Positive immediate offset
pub fn arm_strb_ofip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofip
/// Load byte
/// Positive immediate offset
pub fn arm_ldrb_ofip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prip
/// Store byte
/// Immediate offset, pre-increment
pub fn arm_strb_prip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prip
/// Load byte
/// Immediate offset, pre-increment
pub fn arm_ldrb_prip(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrmll
/// Store word
/// Left-shifted register offset, post-decrement
pub fn arm_str_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrmlr
/// Store word
/// Right-shifted register offset, post-decrement
pub fn arm_str_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrmar
/// Store word
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_str_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrmrr
/// Store word
/// Right-rotated register offset, post-decrement
pub fn arm_str_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrmll
/// Load word
/// Left-shifted register offset, post-decrement
pub fn arm_ldr_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrmlr
/// Load word
/// Right-shifted register offset, post-decrement
pub fn arm_ldr_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrmar
/// Load word
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_ldr_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrmrr
/// Load word
/// Right-rotated register offset, post-decrement
pub fn arm_ldr_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrmll
/// Store word from user-mode register
/// Left-shifted register offset, post-decrement
pub fn arm_strt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrmlr
/// Store word from user-mode register
/// Right-shifted register offset, post-decrement
pub fn arm_strt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrmar
/// Store word from user-mode register
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_strt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrmrr
/// Store word from user-mode register
/// Right-rotated register offset, post-decrement
pub fn arm_strt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrmll
/// Load word into user-mode register
/// Left-shifted register offset, post-decrement
pub fn arm_ldrt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrmlr
/// Load word into user-mode register
/// Right-shifted register offset, post-decrement
pub fn arm_ldrt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrmar
/// Load word into user-mode register
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_ldrt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrmrr
/// Load word into user-mode register
/// Right-rotated register offset, post-decrement
pub fn arm_ldrt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrmll
/// Store byte
/// Left-shifted register offset, post-decrement
pub fn arm_strb_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrmlr
/// Store byte
/// Right-shifted register offset, post-decrement
pub fn arm_strb_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrmar
/// Store byte
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_strb_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrmrr
/// Store byte
/// Right-rotated register offset, post-decrement
pub fn arm_strb_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrmll
/// Load byte
/// Left-shifted register offset, post-decrement
pub fn arm_ldrb_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrmlr
/// Load byte
/// Right-shifted register offset, post-decrement
pub fn arm_ldrb_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrmar
/// Load byte
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_ldrb_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrmrr
/// Load byte
/// Right-rotated register offset, post-decrement
pub fn arm_ldrb_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrmll
/// Store byte from user-mode register
/// Left-shifted register offset, post-decrement
pub fn arm_strbt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrmlr
/// Store byte from user-mode register
/// Right-shifted register offset, post-decrement
pub fn arm_strbt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrmar
/// Store byte from user-mode register
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_strbt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrmrr
/// Store byte from user-mode register
/// Right-rotated register offset, post-decrement
pub fn arm_strbt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrmll
/// Load byte into user-mode register
/// Left-shifted register offset, post-decrement
pub fn arm_ldrbt_ptrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrmlr
/// Load byte into user-mode register
/// Right-shifted register offset, post-decrement
pub fn arm_ldrbt_ptrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrmar
/// Load byte into user-mode register
/// Arithmetic-right-shifted register offset, post-decrement
pub fn arm_ldrbt_ptrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrmrr
/// Load byte into user-mode register
/// Right-rotated register offset, post-decrement
pub fn arm_ldrbt_ptrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrpll
/// Store word
/// Left-shifted register offset, post-increment
pub fn arm_str_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrplr
/// Store word
/// Right-shifted register offset, post-increment
pub fn arm_str_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrpar
/// Store word
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_str_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ptrprr
/// Store word
/// Right-rotated register offset, post-increment
pub fn arm_str_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrpll
/// Load word
/// Left-shifted register offset, post-increment
pub fn arm_ldr_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrplr
/// Load word
/// Right-shifted register offset, post-increment
pub fn arm_ldr_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrpar
/// Load word
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_ldr_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ptrprr
/// Load word
/// Right-rotated register offset, post-increment
pub fn arm_ldr_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrpll
/// Store word from user-mode register
/// Left-shifted register offset, post-increment
pub fn arm_strt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrplr
/// Store word from user-mode register
/// Right-shifted register offset, post-increment
pub fn arm_strt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrpar
/// Store word from user-mode register
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_strt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRT ptrprr
/// Store word from user-mode register
/// Right-rotated register offset, post-increment
pub fn arm_strt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrpll
/// Load word into user-mode register
/// Left-shifted register offset, post-increment
pub fn arm_ldrt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrplr
/// Load word into user-mode register
/// Right-shifted register offset, post-increment
pub fn arm_ldrt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrpar
/// Load word into user-mode register
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_ldrt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRT ptrprr
/// Load word into user-mode register
/// Right-rotated register offset, post-increment
pub fn arm_ldrt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrpll
/// Store byte
/// Left-shifted register offset, post-increment
pub fn arm_strb_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrplr
/// Store byte
/// Right-shifted register offset, post-increment
pub fn arm_strb_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrpar
/// Store byte
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_strb_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ptrprr
/// Store byte
/// Right-rotated register offset, post-increment
pub fn arm_strb_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrpll
/// Load byte
/// Left-shifted register offset, post-increment
pub fn arm_ldrb_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrplr
/// Load byte
/// Right-shifted register offset, post-increment
pub fn arm_ldrb_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrpar
/// Load byte
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_ldrb_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ptrprr
/// Load byte
/// Right-rotated register offset, post-increment
pub fn arm_ldrb_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrpll
/// Store byte from user-mode register
/// Left-shifted register offset, post-increment
pub fn arm_strbt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrplr
/// Store byte from user-mode register
/// Right-shifted register offset, post-increment
pub fn arm_strbt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrpar
/// Store byte from user-mode register
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_strbt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRBT ptrprr
/// Store byte from user-mode register
/// Right-rotated register offset, post-increment
pub fn arm_strbt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrpll
/// Load byte into user-mode register
/// Left-shifted register offset, post-increment
pub fn arm_ldrbt_ptrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrplr
/// Load byte into user-mode register
/// Right-shifted register offset, post-increment
pub fn arm_ldrbt_ptrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrpar
/// Load byte into user-mode register
/// Arithmetic-right-shifted register offset, post-increment
pub fn arm_ldrbt_ptrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRBT ptrprr
/// Load byte into user-mode register
/// Right-rotated register offset, post-increment
pub fn arm_ldrbt_ptrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrmll
/// Store word
/// Negative left-shifted register offset
pub fn arm_str_ofrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrmlr
/// Store word
/// Negative right-shifted register offset
pub fn arm_str_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrmar
/// Store word
/// Negative arithmetic-right-shifted register offset
pub fn arm_str_ofrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrmrr
/// Store word
/// Negative right-rotated register offset
pub fn arm_str_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrmll
/// Load word
/// Negative left-shifted register offset
pub fn arm_ldr_ofrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrmlr
/// Load word
/// Negative right-shifted register offset
pub fn arm_ldr_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrmar
/// Load word
/// Negative arithmetic-right-shifted register offset
pub fn arm_ldr_ofrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrmrr
/// Load word
/// Negative right-rotated register offset
pub fn arm_ldr_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrmll
/// Store word
/// Left-shifted register offset, pre-decrement
pub fn arm_str_prrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrmlr
/// Store word
/// Right-shifted register offset, pre-decrement
pub fn arm_str_prrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrmar
/// Store word
/// Arithmetic-right-shifted register offset, pre-decrement
pub fn arm_str_prrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrmrr
/// Store word
/// Right-rotated register offset, pre-decrement
pub fn arm_str_prrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrmll
/// Load word
/// Left-shifted register offset, pre-decrement
pub fn arm_ldr_prrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrmlr
/// Load word
/// Right-shifted register offset, pre-decrement
pub fn arm_ldr_prrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrmar
/// Load word
/// Arithmetic-right-shifted register offset, pre-decrement
pub fn arm_ldr_prrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrmrr
/// Load word
/// Right-rotated register offset, pre-decrement
pub fn arm_ldr_prrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrmll
/// Store byte
/// Negative left-shifted register offset
pub fn arm_strb_ofrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrmlr
/// Store byte
/// Negative right-shifted register offset
pub fn arm_strb_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrmar
/// Store byte
/// Negative arithmetic-right-shifted register offset
pub fn arm_strb_ofrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrmrr
/// Store byte
/// Negative right-rotated register offset
pub fn arm_strb_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrmll
/// Load byte
/// Negative left-shifted register offset
pub fn arm_ldrb_ofrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrmlr
/// Load byte
/// Negative right-shifted register offset
pub fn arm_ldrb_ofrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrmar
/// Load byte
/// Negative arithmetic-right-shifted register offset
pub fn arm_ldrb_ofrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrmrr
/// Load byte
/// Negative right-rotated register offset
pub fn arm_ldrb_ofrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrmll
/// Store byte
/// Left-shifted register offset, pre-decrement
pub fn arm_strb_prrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrmlr
/// Store byte
/// Right-shifted register offset, pre-decrement
pub fn arm_strb_prrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrmar
/// Store byte
/// Arithmetic-right-shifted register offset, pre-decrement
pub fn arm_strb_prrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrmrr
/// Store byte
/// Right-rotated register offset, pre-decrement
pub fn arm_strb_prrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrmll
/// Load byte
/// Left-shifted register offset, pre-decrement
pub fn arm_ldrb_prrmll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrmlr
/// Load byte
/// Right-shifted register offset, pre-decrement
pub fn arm_ldrb_prrmlr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrmar
/// Load byte
/// Arithmetic-right-shifted register offset, pre-decrement
pub fn arm_ldrb_prrmar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrmrr
/// Load byte
/// Right-rotated register offset, pre-decrement
pub fn arm_ldrb_prrmrr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrpll
/// Store word
/// Positive left-shifted register offset
pub fn arm_str_ofrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrplr
/// Store word
/// Positive right-shifted register offset
pub fn arm_str_ofrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrpar
/// Store word
/// Positive arithmetic-right-shifted register offset
pub fn arm_str_ofrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR ofrprr
/// Store word
/// Positive right-rotated register offset
pub fn arm_str_ofrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrpll
/// Load word
/// Positive left-shifted register offset
pub fn arm_ldr_ofrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrplr
/// Load word
/// Positive right-shifted register offset
pub fn arm_ldr_ofrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrpar
/// Load word
/// Positive arithmetic-right-shifted register offset
pub fn arm_ldr_ofrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR ofrprr
/// Load word
/// Positive right-rotated register offset
pub fn arm_ldr_ofrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrpll
/// Store word
/// Left-shifted register offset, pre-increment
pub fn arm_str_prrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrplr
/// Store word
/// Right-shifted register offset, pre-increment
pub fn arm_str_prrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrpar
/// Store word
/// Arithmetic-right-shifted register offset, pre-increment
pub fn arm_str_prrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STR prrprr
/// Store word
/// Right-rotated register offset, pre-increment
pub fn arm_str_prrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrpll
/// Load word
/// Left-shifted register offset, pre-increment
pub fn arm_ldr_prrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrplr
/// Load word
/// Right-shifted register offset, pre-increment
pub fn arm_ldr_prrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrpar
/// Load word
/// Arithmetic-right-shifted register offset, pre-increment
pub fn arm_ldr_prrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDR prrprr
/// Load word
/// Right-rotated register offset, pre-increment
pub fn arm_ldr_prrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrpll
/// Store byte
/// Positive left-shifted register offset
pub fn arm_strb_ofrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrplr
/// Store byte
/// Positive right-shifted register offset
pub fn arm_strb_ofrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrpar
/// Store byte
/// Positive arithmetic-right-shifted register offset
pub fn arm_strb_ofrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB ofrprr
/// Store byte
/// Positive right-rotated register offset
pub fn arm_strb_ofrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrpll
/// Load byte
/// Positive left-shifted register offset
pub fn arm_ldrb_ofrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrplr
/// Load byte
/// Positive right-shifted register offset
pub fn arm_ldrb_ofrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrpar
/// Load byte
/// Positive arithmetic-right-shifted register offset
pub fn arm_ldrb_ofrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB ofrprr
/// Load byte
/// Positive right-rotated register offset
pub fn arm_ldrb_ofrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrpll
/// Store byte
/// Left-shifted register offset, pre-increment
pub fn arm_strb_prrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrplr
/// Store byte
/// Right-shifted register offset, pre-increment
pub fn arm_strb_prrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrpar
/// Store byte
/// Arithmetic-right-shifted register offset, pre-increment
pub fn arm_strb_prrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// STRB prrprr
/// Store byte
/// Right-rotated register offset, pre-increment
pub fn arm_strb_prrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrpll
/// Load byte
/// Left-shifted register offset, pre-increment
pub fn arm_ldrb_prrpll(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrplr
/// Load byte
/// Right-shifted register offset, pre-increment
pub fn arm_ldrb_prrplr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrpar
/// Load byte
/// Arithmetic-right-shifted register offset, pre-increment
pub fn arm_ldrb_prrpar(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
}

/// LDRB prrprr
/// Load byte
/// Right-rotated register offset, pre-increment
pub fn arm_ldrb_prrprr(cpu: &mut ArmCpu, instr: u32) {
	// #TODO
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

