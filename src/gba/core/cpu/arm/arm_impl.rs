// #FIXME This entire file basically relies on the optimizer removing
// if false {} blocks and the else blocks of if true {} else {} blocks.
// That might be entirely okay and if that proves to be the case, I can make
// some adjustments or run some type of "optimizer" of my own that just expands
// all of the macros.

use super::super::ArmCpu;
use super::super::registers;
use super::functions::*;
use super::super::alu;

// Used to decide whether IO indexing should be pre/post function.
const PRE: bool = true;
const POST: bool = false;

// Used to decide whether IO indexing should increment or decrement.
const DEC: bool = false;
const INC: bool = true;

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

// Block Data Transfer Functions
const LDM: fn(&mut ArmCpu, u32, u32) = arm_fn_ldm_single;
const STM: fn(&mut ArmCpu, u32, u32) = arm_fn_stm_single;

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
		$operand2_fn_reg: expr, // true if the operand 2 function is a register shift.
		$operation:ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let rn = (instr >> 16) & 0xf;
			let rd = (instr >> 12) & 0xf;
			let rn_value = cpu.rget(rn);
			let operand2 = $operand2_function(cpu, instr);

			if $operand2_fn_reg {
				cpu.clock.internal(1); // internal cycle for register shift.
			}

			let result = $operation(cpu, rn_value, operand2);
			cpu.rset(rd, result);

			if rd == 15 {
				cpu.clock_branched_arm();
			}
		}
	);

	(
		$instr_name:ident,
		$operand2_function:ident,
		$operation:ident
	) => (
		gen_dproc!($instr_name, $operand2_function, false, $operation);
	);
}

/// Generates a function for a dataprocessing instruction that sets flags..
///
/// Pass in the name of the instruction to generate,
/// The function used to retrieve the second operand
/// of the instruction, and a function to be applied
/// to both operands.
macro_rules! gen_dproc_sf {
	(
		$instr_name:ident,
		$operand2_function:ident,
		$operand2_fn_reg: expr, // true if the operand 2 function is a register shift.
		$operation:ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let rn = (instr >> 16) & 0xf;
			let saved_cpsr = cpu.registers.get_cpsr();
			let rn_value = cpu.rget(rn);
			let operand2 = $operand2_function(cpu, instr);

			if $operand2_fn_reg {
				cpu.clock.internal(1); // internal cycle for register shift.
			}

			let result = $operation(cpu, rn_value, operand2);
			let rd = (instr >> 12) & 0xf;

			// When Rd is R15 and the S flag is set the result of the operation is placed in R15 and the 
			// SPSR corresponding to the current mode is moved to the CPSR. 
			// This allows state changes which atomically restore both PC and CPSR. 
			// This form of instruction should not be used in User mode.
			if rd == 15 { 
				cpu.set_pc(result);
				cpu.registers.set_cpsr(saved_cpsr); // to make it seem as if there was no changes to the flags.
				cpu.registers.spsr_to_cpsr();

				cpu.clock_branched_arm();
			} else {
				cpu.rset(rd, result);
			}
		}
	);

	(
		$instr_name:ident,
		$operand2_function:ident,
		$operation:ident
	) => (
		gen_dproc_sf!($instr_name, $operand2_function, false, $operation);
	);
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
		$operand2_fn_reg: expr, // true if the operand 2 function is a register shift.
		$operation:ident
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let rn = (instr >> 16) & 0xf;
			let rn_value = cpu.rget(rn);
			let operand2 = $operand2_function(cpu, instr);

			if $operand2_fn_reg {
				cpu.clock.internal(1); // internal cycle for register shift.
			}

			$operation(cpu, rn_value, operand2);
		}
	);

	(
		$instr_name:ident,
		$operand2_function:ident,
		$operation:ident
	) => (
		gen_dproc_nw!($instr_name, $operand2_function, false, $operation);
	);
}

/// Generates a multiply instruction.
macro_rules! gen_mul {
	(
		$instr_name: ident,
		$operation: ident,
		$accumulate: expr,
		$set_condition: expr
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let rm = instr & 0xf;
			let rs = (instr >> 8) & 0xf;
			let rn = (instr >> 12) & 0xf;
			let rd = (instr >> 16) & 0xf;
			let _rs = cpu.rget(rs); // clock needs this.
			let result = $operation(cpu.rget(rm), _rs, cpu.rget(rn));
			if $set_condition { alu::set_nz_flags(cpu, result); }
			cpu.rset(rd, result);

			if (_rs & 0xffffff00) == 0 { cpu.clock.internal(1) }
			else if (_rs & 0xffff0000) == 0 { cpu.clock.internal(2) }
			else if (_rs & 0xff000000) == 0 { cpu.clock.internal(3) }

			if $accumulate {
				cpu.clock.internal(1);
			}
		}
	);


	(
		$instr_name: ident,
		$operation: ident,
		$set_condition: expr
	) => (
		gen_mul!($instr_name, $operation, false, $set_condition);
	);
}


/// Generates a multiply long instruction.
macro_rules! gen_mull {
	(
		$instr_name: ident,
		$operation: ident,
		$accumulate: expr,
		$set_condition: expr
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let rm = instr & 0xf;
			let rs = (instr >> 8) & 0xf;
			let rd_lo = (instr >> 12) & 0xf;
			let rd_hi = (instr >> 16) & 0xf;
			let _rs = cpu.rget(rs); // clock needs this.
			let result = $operation(
				cpu.rget(rm),
				_rs,
				cpu.rget(rd_hi),
				cpu.rget(rd_lo)
			);
			cpu.rset(rd_lo, (result & 0xffffffff) as u32);
			cpu.rset(rd_hi, ((result >> 32) & 0xffffffff) as u32);
			if $set_condition { alu::set_nz_flags64(cpu, result); }

			if (_rs & 0xffffff00) == 0 { cpu.clock.internal(1) }
			else if (_rs & 0xffff0000) == 0 { cpu.clock.internal(2) }
			else if (_rs & 0xff000000) == 0 { cpu.clock.internal(3) }

			if $accumulate {
				cpu.clock.internal(2);
			}
		}
	);

	(
		$instr_name: ident,
		$operation: ident,
		$set_condition: expr
	) => (
		gen_mull!($instr_name, $operation, false, $set_condition);
	);
}

/// Generates a single data transfer instruction.
macro_rules! gen_str {
	(
		$instr_name:ident,	// the name of the instruction
		$transfer: ident,	// the function being used by the instruction to transfer data.
		$index_pre: expr,	// boolean - true if this is pre-indexed, false otherwise
		$index_inc: expr,	// boolean - true if this is incrementing, false if decrementing
		$offset_fn: ident,	// the function used to generate an offset.
		$writeback: expr,	// boolean - true if this should writeback (still writes back if post indexed or user mode)
		$user: expr			// boolean - true if this should force user mode registers.
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let last_mode = cpu.registers.get_mode();
			if $user {
				cpu.registers.set_mode(registers::MODE_USR);
			}

			let rn = (instr >> 16) & 0xf;
			let rd = (instr >> 12) & 0xf;
			let _rn = cpu.rget(rn); // base
			let offset = $offset_fn(cpu, instr);

			let mut address = _rn;
			if $index_pre {
				if $index_inc { address += offset; }
				else { address -= offset; }
			}

			$transfer(cpu, address, rd);

			if $writeback || $user || !($index_pre) {
				if !($index_pre) {
					if $index_inc { address += offset; }
					else { address -= offset; }
				}
				cpu.rset(rn, address);
			}

			if $user {
				cpu.registers.set_mode(last_mode);
			}

			// NOTE: Clock is incremented inside of the transfer functions!
		}
	)
}


/// Generates a single data transfer instruction.
macro_rules! gen_ldr {
	(
		$instr_name:ident,	// the name of the instruction
		$transfer: ident,	// the function being used by the instruction to transfer data.
		$index_pre: expr,	// boolean - true if this is pre-indexed, false otherwise
		$index_inc: expr,	// boolean - true if this is incrementing, false if decrementing
		$offset_fn: ident,	// the function used to generate an offset.
		$writeback: expr,	// boolean - true if this should writeback (still writes back if post indexed or user mode)
		$user: expr			// boolean - true if this should force user mode registers.
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();
			cpu.clock.internal(1);

			let last_mode = cpu.registers.get_mode();
			if $user {
				cpu.registers.set_mode(registers::MODE_USR);
			}

			let rn = (instr >> 16) & 0xf;
			let rd = (instr >> 12) & 0xf;
			let _rn = cpu.rget(rn); // base
			let offset = $offset_fn(cpu, instr);

			let mut address = _rn;
			if $index_pre {
				if $index_inc { address += offset; }
				else { address -= offset; }
			}

			$transfer(cpu, address, rd);

			if $writeback || $user || !($index_pre) {
				if rn != rd { // LDR data overrides the writeback data.
					if !($index_pre) {
						if $index_inc { address += offset; }
						else { address -= offset; }
					}
					cpu.rset(rn, address);
				}
			}

			if rd == 15 {
				cpu.clock_branched_arm();
			}

			if $user {
				cpu.registers.set_mode(last_mode);
			}

			// NOTE: Clock is incremented inside of the transfer functions!
		}
	)
}

/// Generates a Half Word Load instruction
macro_rules! gen_hldr {
	(
		$instr_name:ident,	// the name of the instruction
		$transfer: ident,	// the function being used by the instruction to transfer data.
		$index_pre: expr,	// boolean - true if this is pre-indexed, false otherwise
		$index_inc: expr,	// boolean - true if this is incrementing, false if decrementing
		$offset_fn: ident,	// the function used to generate an offset.
		$writeback: expr	// boolean - true if this should writeback (still writes back if post indexed)
	) => (
		gen_ldr!($instr_name, $transfer, $index_pre, $index_inc, $offset_fn, $writeback, false);
	)
}


/// Generates a Half Word Store instruction
macro_rules! gen_hstr {
	(
		$instr_name:ident,	// the name of the instruction
		$transfer: ident,	// the function being used by the instruction to transfer data.
		$index_pre: expr,	// boolean - true if this is pre-indexed, false otherwise
		$index_inc: expr,	// boolean - true if this is incrementing, false if decrementing
		$offset_fn: ident,	// the function used to generate an offset.
		$writeback: expr	// boolean - true if this should writeback (still writes back if post indexed)
	) => (
		gen_str!($instr_name, $transfer, $index_pre, $index_inc, $offset_fn, $writeback, false);
	)
}

macro_rules! gen_stm {
	(
		$instr_name: ident,
		$index_pre: expr,
		$index_inc: expr,
		$writeback: expr,
		$user: expr
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();
			let last_mode = cpu.registers.get_mode();
			if $user { cpu.registers.set_mode(registers::MODE_USR); }

			let rn = (instr >> 16) & 0xf;
			let mut address = cpu.rget(rn);

			// println!("rn: {}", rn);

			let mut first_transfer = true;

			if $index_inc {
				for r in 0..16 {
					if ((instr >> r) & 1) == 1 {
						if $index_pre { address += 4; } // pre index
						STM(cpu, address, r);

						if first_transfer {
							cpu.clock.data_access32_nonseq(address);
							first_transfer = false;
						} else {
							cpu.clock.data_access32_seq(address);
						}

						if !$index_pre { address += 4; } // post index
						if $writeback { cpu.rset(rn, address) } // writeback at the end of the second cycle.
					}
				}
			} else {
				for r in 0..16 {
					if ((instr >> r) & 1) == 1 {
						address -= 4;
					}
				}

				let ending_addr = address;

				let mut wroteback = false;
				for r in 0..16 {
					if ((instr >> r) & 1) == 1 {
						if !$index_pre { address += 4; }
						STM(cpu, address, r);

						if first_transfer {
							cpu.clock.data_access32_nonseq(address);
							first_transfer = false;
						} else {
							cpu.clock.data_access32_seq(address);
						}

						if $writeback && !wroteback { cpu.rset(rn, ending_addr); wroteback = true; } // writeback at the end of the second cycle.
						if $index_pre { address += 4; }
					}
				}
			}

			if $user { cpu.registers.set_mode(last_mode); } // restore the register state
		}
	)
}

macro_rules! gen_ldm {
	(
		$instr_name: ident,
		$index_pre: expr,
		$index_inc: expr,
		$writeback: expr,
		$user: expr
	) => (
		pub fn $instr_name(cpu: &mut ArmCpu, instr: u32) {
			cpu.clock_prefetch_arm();

			let psr_transfer = ((instr >> 15) & 1) == 1 && $user; // r15 is in the transfer list and user mode.
			let last_mode = cpu.registers.get_mode();
			if $user { cpu.registers.set_mode(registers::MODE_USR); }

			let rn = (instr >> 16) & 0xf;
			let mut address = cpu.rget(rn);

			let mut first_transfer = true;

			if $index_inc {
				for r in 0..16 {
					if ((instr >> r) & 1) == 1 {
						if $index_pre { address += 4; } // pre index
						LDM(cpu, address, r);
						if r == 15 {
							if psr_transfer {cpu.registers.spsr_to_cpsr();} // this loads the spsr in to the cpsr.}
							cpu.clock_branched_arm();
						}

						if first_transfer {
							cpu.clock.data_access32_nonseq(address);
							first_transfer = false;
						} else {
							cpu.clock.data_access32_seq(address);
						}

						if !$index_pre { address += 4; } // post index
						if $writeback { cpu.rset(rn, address) } // writeback at the end of the second cycle.
					}
				}
			} else {
				let mut allow_writeback = true;
				for r in 0..16 {
					if ((instr >> r) & 1) == 1 {
						if allow_writeback && r == rn { allow_writeback = false; }
						address -= 4;
					}
				}

				let ending_addr = address;

				let mut wroteback = false;
				for r in 0..16 {
					if ((instr >> r) & 1) == 1 {
						if !$index_pre { address += 4; }
						LDM(cpu, address, r);
						if r == 15 {
							if psr_transfer {cpu.registers.spsr_to_cpsr();} // this loads the spsr in to the cpsr.}
							cpu.clock_branched_arm();
						}

						if first_transfer {
							cpu.clock.data_access32_nonseq(address);
							first_transfer = false;
						} else {
							cpu.clock.data_access32_seq(address);
						}

						if $writeback && !wroteback && allow_writeback { cpu.rset(rn, ending_addr); wroteback = true; } // writeback at the end of the second cycle.
						if $index_pre { address += 4; }
					}
				}
			}

			if $user && !psr_transfer { cpu.registers.set_mode(last_mode); } // restore the register state
		}
	)
}


/// This is now obsolete but, I might try to make
/// the optimization at some point anyways. LDM with
/// the S bit set and r15 in the transfer list, also does
/// a PSR transfer when r15 is transferred.
macro_rules! gen_ldm_u {
	(
		$instr_name:ident,
		$index_pre: expr,
		$index_inc: expr,
		$writeback: expr
	) => ( 
		gen_ldm!($instr_name, $index_pre, $index_inc, $writeback, true); 
	)
}

/// B 
/// Branch
pub fn arm_b(cpu: &mut ArmCpu, instr: u32) {
	let pc = cpu.get_pc();
	cpu.clock_prefetch_arm();

	let mut offset = instr & 0xffffff;
	offset <<= 2;
	offset = (((offset as i32) << 6) >> 6) as u32;
	let destination = pc + offset;
	cpu.set_pc(destination);

	cpu.clock_branched_arm();
}

/// BL 
/// Branch and link
pub fn arm_bl(cpu: &mut ArmCpu, instr: u32) {
	let pc = cpu.get_pc();
	cpu.clock_prefetch_arm();

	cpu.rset(14, (pc - 4) & 0xFFFFFFFC);
	let mut offset = instr & 0xffffff;
	offset <<= 2;
	offset = (((offset as i32) << 6) >> 6) as u32;
	cpu.set_pc(pc + offset);
	let destination = pc + offset;
	cpu.set_pc(destination);

	cpu.clock_branched_arm();
}

/// BX
/// Branch and switch execution modes
pub fn arm_bx(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();

	let rn = instr & 0xf;
	let address = cpu.rget(rn);
	if (address & 1) == 1 {
		// branch into thumb mode.
		let destination = address & 0xFFFFFFFE;
		cpu.registers.setf_t();
		cpu.set_pc(destination);

		cpu.clock_branched_thumb();
	} else {
		// branch into arm mode.
		let destination = address & 0xFFFFFFFC;
		cpu.set_pc(destination);

		cpu.clock_branched_arm();
	}
}

/// MSR rc
/// Move value to status word
/// Register, CPSR
pub fn arm_msr_rc(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let rm = instr & 0xf;
	let _rm = cpu.rget(rm);
	if ((instr >> 16) & 1) == 1 {
		cpu.registers.set_cpsr_safe(_rm);
	} else {
		cpu.registers.set_cpsr_flags(_rm);
	}
}

/// MRS rc
/// Move status word to register
/// Register, CPSR
pub fn arm_mrs_rc(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let rd = (instr >> 12) & 0xf;
	let cpsr = cpu.registers.get_cpsr();
	cpu.rset(rd, cpsr);
}

/// MRS rs
/// Move status word to register
/// Register, SPSR
pub fn arm_mrs_rs(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let rd = (instr >> 12) & 0xf;
	let spsr = cpu.registers.get_spsr();
	cpu.rset(rd, spsr);
}

/// MSR rs
/// Move value to status word
/// Register, SPSR
pub fn arm_msr_rs(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let rm = instr & 0xf;
	let _rm = cpu.rget(rm);
	if ((instr >> 16) & 1) == 1 {
		cpu.registers.set_spsr_safe(_rm);
	} else {
		cpu.registers.set_spsr_flags(_rm);
	}
}

/// MSR is
/// Move value to status word
/// Immediate, SPSR
pub fn arm_msr_is(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let immediate = arm_fn_op2_imm(cpu, instr);
	cpu.registers.set_spsr_flags(immediate);
}

/// MSR ic
/// Move value to status word
/// Immediate, CPSR
pub fn arm_msr_ic(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let immediate = arm_fn_op2_imm(cpu, instr);
	cpu.registers.set_cpsr_flags(immediate);
}

/// SWP
/// Swap registers with memory word
pub fn arm_swp(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let rm = instr & 0xf;
	let rd = (instr >> 12) & 0xf;
	let rn = (instr >> 16) & 0xf;
	let source = cpu.rget(rm);
	let address = cpu.rget(rn);
	let temp = cpu.mread32_al(address);
	cpu.mwrite32(address, source);
	cpu.rset(rd, temp);
	cpu.clock.internal(1);
	cpu.clock.data_access32_nonseq(address);
	cpu.clock.data_access32_nonseq(address);
}

/// SWPB
/// Swap registers with memory byte
pub fn arm_swpb(cpu: &mut ArmCpu, instr: u32) {
	cpu.clock_prefetch_arm();
	let rm = instr & 0xf;
	let rd = (instr >> 12) & 0xf;
	let rn = (instr >> 16) & 0xf;
	let source = cpu.rget(rm);
	let address = cpu.rget(rn);
	let temp = cpu.mread8_al(address);
	cpu.mwrite8(address, (source & 0xff) as u8);
	cpu.rset(rd, temp);
	cpu.clock.internal(1);
	cpu.clock.data_access8_nonseq(address);
	cpu.clock.data_access8_nonseq(address);
}

/// SWI 
/// Software interrupt (enter supervisor mode)
pub fn arm_swi(cpu: &mut ArmCpu, instr: u32) {
	cpu.arm_swi(instr);
}

/// UNDEFINED
/// just increments the clock
pub fn arm_undefined(cpu: &mut ArmCpu, _: u32) {
	cpu.on_undefined();
}

/// AND lli
/// Logical And
/// Logical shift-left by immediate
gen_dproc!(arm_and_lli, arm_fn_op2_lli, arm_fn_and);

/// AND llr
/// Logical And
/// Logical shift-left by register
gen_dproc!(arm_and_llr, arm_fn_op2_llr, true, arm_fn_and);

/// AND lri
/// Logical And
/// Logical shift-right by immediate
gen_dproc!(arm_and_lri, arm_fn_op2_lri, arm_fn_and);

/// AND lrr
/// Logical And
/// Logical shift-right by register
gen_dproc!(arm_and_lrr, arm_fn_op2_lrr, true, arm_fn_and);

/// AND ari
/// Logical And
/// Arithmetic shift-right by immediate
gen_dproc!(arm_and_ari, arm_fn_op2_ari, arm_fn_and);

/// AND arr
/// Logical And
/// Arithmetic shift-right by register
gen_dproc!(arm_and_arr, arm_fn_op2_arr, true, arm_fn_and);

/// AND rri
/// Logical And
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_and_rri, arm_fn_op2_rri, arm_fn_and);

/// AND rrr
/// Logical And
/// Rotate right by register
gen_dproc!(arm_and_rrr, arm_fn_op2_rrr, true, arm_fn_and);

/// MUL
/// Multiply registers
gen_mul!(arm_mul, arm_fn_mul, false);

/// STRH ptrm
/// Store halfword
/// Register offset, post-decrement
gen_hstr!(arm_strh_ptrm, STRH, POST, DEC, HDT_REG, false);

/// ANDS lli
/// Logical And, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_ands_lli, arm_fn_op2_lli_s, arm_fn_and_s);

/// ANDS llr
/// Logical And, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_ands_llr, arm_fn_op2_llr_s, true, arm_fn_and_s);

/// ANDS lri
/// Logical And, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_ands_lri, arm_fn_op2_lri_s, arm_fn_and_s);

/// ANDS lrr
/// Logical And, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_ands_lrr, arm_fn_op2_lrr_s, true, arm_fn_and_s);

/// ANDS ari
/// Logical And, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_ands_ari, arm_fn_op2_ari_s, arm_fn_and_s);

/// ANDS arr
/// Logical And, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_ands_arr, arm_fn_op2_arr_s, true, arm_fn_and_s);

/// ANDS rri
/// Logical And, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_ands_rri, arm_fn_op2_rri_s, arm_fn_and_s);

/// ANDS rrr
/// Logical And, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_ands_rrr, arm_fn_op2_rrr_s, true, arm_fn_and_s);

/// MULS
/// Multiply registers, setting flags
gen_mul!(arm_muls, arm_fn_mul, true);

/// LDRH ptrm
/// Load halfword
/// Register offset, post-decrement
gen_hldr!(arm_ldrh_ptrm, LDRH, POST, DEC, HDT_REG, false);

/// LDRSB ptrm
/// Load signed byte
/// Register offset, post-decrement
gen_hldr!(arm_ldrsb_ptrm, LDRSB, POST, DEC, HDT_REG, false);

/// LDRSH ptrm
/// Load signed halfword
/// Register offset, post-decrement
gen_hldr!(arm_ldrsh_ptrm, LDRSH, POST, DEC, HDT_REG, false);

/// EOR lli
/// Logical Exclusive-or
/// Logical shift-left by immediate
gen_dproc!(arm_eor_lli, arm_fn_op2_lli, arm_fn_eor);

/// EOR llr
/// Logical Exclusive-or
/// Logical shift-left by register
gen_dproc!(arm_eor_llr, arm_fn_op2_llr, true, arm_fn_eor);

/// EOR lri
/// Logical Exclusive-or
/// Logical shift-right by immediate
gen_dproc!(arm_eor_lri, arm_fn_op2_lri, arm_fn_eor);

/// EOR lrr
/// Logical Exclusive-or
/// Logical shift-right by register
gen_dproc!(arm_eor_lrr, arm_fn_op2_lrr, true, arm_fn_eor);

/// EOR ari
/// Logical Exclusive-or
/// Arithmetic shift-right by immediate
gen_dproc!(arm_eor_ari, arm_fn_op2_ari, arm_fn_eor);

/// EOR arr
/// Logical Exclusive-or
/// Arithmetic shift-right by register
gen_dproc!(arm_eor_arr, arm_fn_op2_arr, true, arm_fn_eor);

/// EOR rri
/// Logical Exclusive-or
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_eor_rri, arm_fn_op2_rri, arm_fn_eor);

/// EOR rrr
/// Logical Exclusive-or
/// Rotate right by register
gen_dproc!(arm_eor_rrr, arm_fn_op2_rrr, true, arm_fn_eor);

/// MLA
/// Multiply and accumulate registers
gen_mul!(arm_mla, arm_fn_mla, true, false);

/// EORS lli
/// Logical Exclusive-or, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_eors_lli, arm_fn_op2_lli_s, arm_fn_eor_s);

/// EORS llr
/// Logical Exclusive-or, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_eors_llr, arm_fn_op2_llr_s, true, arm_fn_eor_s);

/// EORS lri
/// Logical Exclusive-or, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_eors_lri, arm_fn_op2_lri_s, arm_fn_eor_s);

/// EORS lrr
/// Logical Exclusive-or, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_eors_lrr, arm_fn_op2_lrr_s, true, arm_fn_eor_s);

/// EORS ari
/// Logical Exclusive-or, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_eors_ari, arm_fn_op2_ari_s, arm_fn_eor_s);

/// EORS arr
/// Logical Exclusive-or, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_eors_arr, arm_fn_op2_arr_s, true, arm_fn_eor_s);

/// EORS rri
/// Logical Exclusive-or, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_eors_rri, arm_fn_op2_rri_s, arm_fn_eor_s);

/// EORS rrr
/// Logical Exclusive-or, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_eors_rrr, arm_fn_op2_rrr_s, true, arm_fn_eor_s);

/// MLAS
/// Multiply and accumulate registers, setting flags
gen_mul!(arm_mlas, arm_fn_mla, true, true);

/// SUB lli
/// Subtract from register
/// Logical shift-left by immediate
gen_dproc!(arm_sub_lli, arm_fn_op2_lli, arm_fn_sub);

/// SUB llr
/// Subtract from register
/// Logical shift-left by register
gen_dproc!(arm_sub_llr, arm_fn_op2_llr, true, arm_fn_sub);

/// SUB lri
/// Subtract from register
/// Logical shift-right by immediate
gen_dproc!(arm_sub_lri, arm_fn_op2_lri, arm_fn_sub);

/// SUB lrr
/// Subtract from register
/// Logical shift-right by register
gen_dproc!(arm_sub_lrr, arm_fn_op2_lrr, true, arm_fn_sub);

/// SUB ari
/// Subtract from register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_sub_ari, arm_fn_op2_ari, arm_fn_sub);

/// SUB arr
/// Subtract from register
/// Arithmetic shift-right by register
gen_dproc!(arm_sub_arr, arm_fn_op2_arr, true, arm_fn_sub);

/// SUB rri
/// Subtract from register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_sub_rri, arm_fn_op2_rri, arm_fn_sub);

/// SUB rrr
/// Subtract from register
/// Rotate right by register
gen_dproc!(arm_sub_rrr, arm_fn_op2_rrr, true, arm_fn_sub);

/// STRH ptim
/// Store halfword
/// Immediate offset, post-decrement
gen_hstr!(arm_strh_ptim, STRH, POST, DEC, HDT_IMM, false);

/// SUBS lli
/// Subtract, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_subs_lli, arm_fn_op2_lli_s, arm_fn_sub_s);

/// SUBS llr
/// Subtract, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_subs_llr, arm_fn_op2_llr_s, true, arm_fn_sub_s);

/// SUBS lri
/// Subtract, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_subs_lri, arm_fn_op2_lri_s, arm_fn_sub_s);

/// SUBS lrr
/// Subtract, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_subs_lrr, arm_fn_op2_lrr_s, true, arm_fn_sub_s);

/// SUBS ari
/// Subtract, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_subs_ari, arm_fn_op2_ari_s, arm_fn_sub_s);

/// SUBS arr
/// Subtract, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_subs_arr, arm_fn_op2_arr_s, true, arm_fn_sub_s);

/// SUBS rri
/// Subtract, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_subs_rri, arm_fn_op2_rri_s, arm_fn_sub_s);

/// SUBS rrr
/// Subtract, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_subs_rrr, arm_fn_op2_rrr_s, true, arm_fn_sub_s);

/// LDRH ptim
/// Load halfword
/// Immediate offset, post-decrement
gen_hldr!(arm_ldrh_ptim, LDRH, POST, DEC, HDT_IMM, false);

/// LDRSB ptim
/// Load signed byte
/// Immediate offset, post-decrement
gen_hldr!(arm_ldrsb_ptim, LDRSB, POST, DEC, HDT_IMM, false);

/// LDRSH ptim
/// Load signed halfword
/// Immediate offset, post-decrement
gen_hldr!(arm_ldrsh_ptim, LDRSH, POST, DEC, HDT_IMM, false);

/// RSB lli
/// Subtract register from value
/// Logical shift-left by immediate
gen_dproc!(arm_rsb_lli, arm_fn_op2_lli, arm_fn_rsb);

/// RSB llr
/// Subtract register from value
/// Logical shift-left by register
gen_dproc!(arm_rsb_llr, arm_fn_op2_llr, true, arm_fn_rsb);

/// RSB lri
/// Subtract register from value
/// Logical shift-right by immediate
gen_dproc!(arm_rsb_lri, arm_fn_op2_lri, arm_fn_rsb);

/// RSB lrr
/// Subtract register from value
/// Logical shift-right by register
gen_dproc!(arm_rsb_lrr, arm_fn_op2_lrr, true, arm_fn_rsb);

/// RSB ari
/// Subtract register from value
/// Arithmetic shift-right by immediate
gen_dproc!(arm_rsb_ari, arm_fn_op2_ari, arm_fn_rsb);

/// RSB arr
/// Subtract register from value
/// Arithmetic shift-right by register
gen_dproc!(arm_rsb_arr, arm_fn_op2_arr, true, arm_fn_rsb);

/// RSB rri
/// Subtract register from value
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_rsb_rri, arm_fn_op2_rri, arm_fn_rsb);

/// RSB rrr
/// Subtract register from value
/// Rotate right by register
gen_dproc!(arm_rsb_rrr, arm_fn_op2_rrr, true, arm_fn_rsb);

/// RSBS lli
/// Reverse Subtract, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_rsbs_lli, arm_fn_op2_lli_s, arm_fn_rsb_s);

/// RSBS llr
/// Reverse Subtract, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_rsbs_llr, arm_fn_op2_llr_s, true, arm_fn_rsb_s);

/// RSBS lri
/// Reverse Subtract, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_rsbs_lri, arm_fn_op2_lri_s, arm_fn_rsb_s);

/// RSBS lrr
/// Reverse Subtract, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_rsbs_lrr, arm_fn_op2_lrr_s, true, arm_fn_rsb_s);

/// RSBS ari
/// Reverse Subtract, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_rsbs_ari, arm_fn_op2_ari_s, arm_fn_rsb_s);

/// RSBS arr
/// Reverse Subtract, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_rsbs_arr, arm_fn_op2_arr_s, true, arm_fn_rsb_s);

/// RSBS rri
/// Reverse Subtract, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_rsbs_rri, arm_fn_op2_rri_s, arm_fn_rsb_s);

/// RSBS rrr
/// Reverse Subtract, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_rsbs_rrr, arm_fn_op2_rrr_s, true, arm_fn_rsb_s);

/// ADD lli
/// Add to register
/// Logical shift-left by immediate
gen_dproc!(arm_add_lli, arm_fn_op2_lli, arm_fn_add);

/// ADD llr
/// Add to register
/// Logical shift-left by register
gen_dproc!(arm_add_llr, arm_fn_op2_llr, true, arm_fn_add);

/// ADD lri
/// Add to register
/// Logical shift-right by immediate
gen_dproc!(arm_add_lri, arm_fn_op2_lri, arm_fn_add);

/// ADD lrr
/// Add to register
/// Logical shift-right by register
gen_dproc!(arm_add_lrr, arm_fn_op2_lrr, true, arm_fn_add);

/// ADD ari
/// Add to register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_add_ari, arm_fn_op2_ari, arm_fn_add);

/// ADD arr
/// Add to register
/// Arithmetic shift-right by register
gen_dproc!(arm_add_arr, arm_fn_op2_arr, true, arm_fn_add);

/// ADD rri
/// Add to register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_add_rri, arm_fn_op2_rri, arm_fn_add);

/// ADD rrr
/// Add to register
/// Rotate right by register
gen_dproc!(arm_add_rrr, arm_fn_op2_rrr, true, arm_fn_add);

/// UMULL
/// Unsigned long multiply (32x32 to 64)
gen_mull!(arm_umull, arm_fn_umull, false);

/// STRH ptrp
/// Store halfword
/// Register offset, post-increment
gen_hstr!(arm_strh_ptrp, STRH, POST, INC, HDT_REG, false);

/// ADDS lli
/// Add to register, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_adds_lli, arm_fn_op2_lli_s, arm_fn_add_s);

/// ADDS llr
/// Add to register, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_adds_llr, arm_fn_op2_llr_s, true, arm_fn_add_s);

/// ADDS lri
/// Add to register, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_adds_lri, arm_fn_op2_lri_s, arm_fn_add_s);

/// ADDS lrr
/// Add to register, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_adds_lrr, arm_fn_op2_lrr_s, true, arm_fn_add_s);

/// ADDS ari
/// Add to register, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_adds_ari, arm_fn_op2_ari_s, arm_fn_add_s);

/// ADDS arr
/// Add to register, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_adds_arr, arm_fn_op2_arr_s, true, arm_fn_add_s);

/// ADDS rri
/// Add to register, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_adds_rri, arm_fn_op2_rri_s, arm_fn_add_s);

/// ADDS rrr
/// Add to register, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_adds_rrr, arm_fn_op2_rrr_s, true, arm_fn_add_s);

/// UMULLS
/// Unsigned long multiply, setting flags
gen_mull!(arm_umulls, arm_fn_umull, true);

/// LDRH ptrp
/// Load halfword
/// Register offset, post-increment
gen_hldr!(arm_ldrh_ptrp, LDRH, POST, INC, HDT_REG, false);

/// LDRSB ptrp
/// Load signed byte
/// Register offset, post-increment
gen_hldr!(arm_ldrsb_ptrp, LDRSB, POST, INC, HDT_REG, false);

/// LDRSH ptrp
/// Load signed halfword
/// Register offset, post-increment
gen_hldr!(arm_ldrsh_ptrp, LDRSH, POST, INC, HDT_REG, false);

/// ADC lli
/// Add to register with carry
/// Logical shift-left by immediate
gen_dproc!(arm_adc_lli, arm_fn_op2_lli, arm_fn_adc);

/// ADC llr
/// Add to register with carry
/// Logical shift-left by register
gen_dproc!(arm_adc_llr, arm_fn_op2_llr, true, arm_fn_adc);

/// ADC lri
/// Add to register with carry
/// Logical shift-right by immediate
gen_dproc!(arm_adc_lri, arm_fn_op2_lri, arm_fn_adc);

/// ADC lrr
/// Add to register with carry
/// Logical shift-right by register
gen_dproc!(arm_adc_lrr, arm_fn_op2_lrr, true, arm_fn_adc);

/// ADC ari
/// Add to register with carry
/// Arithmetic shift-right by immediate
gen_dproc!(arm_adc_ari, arm_fn_op2_ari, arm_fn_adc);

/// ADC arr
/// Add to register with carry
/// Arithmetic shift-right by register
gen_dproc!(arm_adc_arr, arm_fn_op2_arr, true, arm_fn_adc);

/// ADC rri
/// Add to register with carry
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_adc_rri, arm_fn_op2_rri, arm_fn_adc);

/// ADC rrr
/// Add to register with carry
/// Rotate right by register
gen_dproc!(arm_adc_rrr, arm_fn_op2_rrr, true, arm_fn_adc);

/// UMLAL
/// Unsigned long multiply and accumulate
gen_mull!(arm_umlal, arm_fn_umlal, true, false);

/// ADCS lli
/// Add to register with carry, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_adcs_lli, arm_fn_op2_lli_s, arm_fn_adc_s);

/// ADCS llr
/// Add to register with carry, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_adcs_llr, arm_fn_op2_llr_s, true, arm_fn_adc_s);

/// ADCS lri
/// Add to register with carry, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_adcs_lri, arm_fn_op2_lri_s, arm_fn_adc_s);

/// ADCS lrr
/// Add to register with carry, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_adcs_lrr, arm_fn_op2_lrr_s, true, arm_fn_adc_s);

/// ADCS ari
/// Add to register with carry, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_adcs_ari, arm_fn_op2_ari_s, arm_fn_adc_s);

/// ADCS arr
/// Add to register with carry, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_adcs_arr, arm_fn_op2_arr_s, true, arm_fn_adc_s);

/// ADCS rri
/// Add to register with carry, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_adcs_rri, arm_fn_op2_rri_s, arm_fn_adc_s);

/// ADCS rrr
/// Add to register with carry, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_adcs_rrr, arm_fn_op2_rrr_s, true, arm_fn_adc_s);

/// UMLALS
/// Unsigned long multiply and accumulate, setting flags
gen_mull!(arm_umlals, arm_fn_umlal, true, true);

/// SBC lli
/// Subtract from register with borrow
/// Logical shift-left by immediate
gen_dproc!(arm_sbc_lli, arm_fn_op2_lli, arm_fn_sbc);

/// SBC llr
/// Subtract from register with borrow
/// Logical shift-left by register
gen_dproc!(arm_sbc_llr, arm_fn_op2_llr, true, arm_fn_sbc);

/// SBC lri
/// Subtract from register with borrow
/// Logical shift-right by immediate
gen_dproc!(arm_sbc_lri, arm_fn_op2_lri, arm_fn_sbc);

/// SBC lrr
/// Subtract from register with borrow
/// Logical shift-right by register
gen_dproc!(arm_sbc_lrr, arm_fn_op2_lrr, true, arm_fn_sbc);

/// SBC ari
/// Subtract from register with borrow
/// Arithmetic shift-right by immediate
gen_dproc!(arm_sbc_ari, arm_fn_op2_ari, arm_fn_sbc);

/// SBC arr
/// Subtract from register with borrow
/// Arithmetic shift-right by register
gen_dproc!(arm_sbc_arr, arm_fn_op2_arr, true, arm_fn_sbc);

/// SBC rri
/// Subtract from register with borrow
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_sbc_rri, arm_fn_op2_rri, arm_fn_sbc);

/// SBC rrr
/// Subtract from register with borrow
/// Rotate right by register
gen_dproc!(arm_sbc_rrr, arm_fn_op2_rrr, true, arm_fn_sbc);

/// SMULL
/// Signed long multiply (32x32 to 64)
gen_mull!(arm_smull, arm_fn_smull, false);

/// STRH ptip
/// Store halfword
/// Immediate offset, post-increment
gen_hstr!(arm_strh_ptip, STRH, POST, INC, HDT_IMM, false);

/// SBCS lli
/// Subtract from register with borrow, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_sbcs_lli, arm_fn_op2_lli_s, arm_fn_sbc_s);

/// SBCS llr
/// Subtract from register with borrow, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_sbcs_llr, arm_fn_op2_llr_s, true, arm_fn_sbc_s);

/// SBCS lri
/// Subtract from register with borrow, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_sbcs_lri, arm_fn_op2_lri_s, arm_fn_sbc_s);

/// SBCS lrr
/// Subtract from register with borrow, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_sbcs_lrr, arm_fn_op2_lrr_s, true, arm_fn_sbc_s);

/// SBCS ari
/// Subtract from register with borrow, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_sbcs_ari, arm_fn_op2_ari_s, arm_fn_sbc_s);

/// SBCS arr
/// Subtract from register with borrow, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_sbcs_arr, arm_fn_op2_arr_s, true, arm_fn_sbc_s);

/// SBCS rri
/// Subtract from register with borrow, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_sbcs_rri, arm_fn_op2_rri_s, arm_fn_sbc_s);

/// SBCS rrr
/// Subtract from register with borrow, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_sbcs_rrr, arm_fn_op2_rrr_s, true, arm_fn_sbc_s);

/// SMULLS
/// Signed long multiply, setting flags
gen_mull!(arm_smulls, arm_fn_smull, true);

/// LDRH ptip
/// Load halfword
/// Immediate offset, post-increment
gen_hldr!(arm_ldrh_ptip, LDRH, POST, INC, HDT_IMM, false);

/// LDRSB ptip
/// Load signed byte
/// Immediate offset, post-increment
gen_hldr!(arm_ldrsb_ptip, LDRSB, POST, INC, HDT_IMM, false);

/// LDRSH ptip
/// Load signed halfword
/// Immediate offset, post-increment
gen_hldr!(arm_ldrsh_ptip, LDRSH, POST, INC, HDT_IMM, false);

/// RSC lli
/// Subtract register from value with borrow
/// Logical shift-left by immediate
gen_dproc!(arm_rsc_lli, arm_fn_op2_lli, arm_fn_rsc);

/// RSC llr
/// Subtract register from value with borrow
/// Logical shift-left by register
gen_dproc!(arm_rsc_llr, arm_fn_op2_llr, true, arm_fn_rsc);

/// RSC lri
/// Subtract register from value with borrow
/// Logical shift-right by immediate
gen_dproc!(arm_rsc_lri, arm_fn_op2_lri, arm_fn_rsc);

/// RSC lrr
/// Subtract register from value with borrow
/// Logical shift-right by register
gen_dproc!(arm_rsc_lrr, arm_fn_op2_lrr, true, arm_fn_rsc);

/// RSC ari
/// Subtract register from value with borrow
/// Arithmetic shift-right by immediate
gen_dproc!(arm_rsc_ari, arm_fn_op2_ari, arm_fn_rsc);

/// RSC arr
/// Subtract register from value with borrow
/// Arithmetic shift-right by register
gen_dproc!(arm_rsc_arr, arm_fn_op2_arr, true, arm_fn_rsc);

/// RSC rri
/// Subtract register from value with borrow
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_rsc_rri, arm_fn_op2_rri, arm_fn_rsc);

/// RSC rrr
/// Subtract register from value with borrow
/// Rotate right by register
gen_dproc!(arm_rsc_rrr, arm_fn_op2_rrr, true, arm_fn_rsc);

/// SMLAL
/// Signed long multiply and accumulate
gen_mull!(arm_smlal, arm_fn_smlal, true, false);

/// RSCS lli
/// Subtract register from value with borrow, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_rscs_lli, arm_fn_op2_lli_s, arm_fn_rsc_s);

/// RSCS llr
/// Subtract register from value with borrow, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_rscs_llr, arm_fn_op2_llr_s, true, arm_fn_rsc_s);

/// RSCS lri
/// Subtract register from value with borrow, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_rscs_lri, arm_fn_op2_lri_s, arm_fn_rsc_s);

/// RSCS lrr
/// Subtract register from value with borrow, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_rscs_lrr, arm_fn_op2_lrr_s, true, arm_fn_rsc_s);

/// RSCS ari
/// Subtract register from value with borrow, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_rscs_ari, arm_fn_op2_ari_s, arm_fn_rsc_s);

/// RSCS arr
/// Subtract register from value with borrow, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_rscs_arr, arm_fn_op2_arr_s, true, arm_fn_rsc_s);

/// RSCS rri
/// Subtract register from value with borrow, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_rscs_rri, arm_fn_op2_rri_s, arm_fn_rsc_s);

/// RSCS rrr
/// Subtract register from value with borrow, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_rscs_rrr, arm_fn_op2_rrr_s, true, arm_fn_rsc_s);

/// SMLALS
/// Signed long multiply and accumulate, setting flags
gen_mull!(arm_smlals, arm_fn_smlal, true, true);

/// STRH ofrm
/// Store halfword
/// Negative register offset
gen_hstr!(arm_strh_ofrm, STRH, PRE, DEC, HDT_REG, false);

/// TSTS lli
/// Test bits in register (Logical And), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_tsts_lli, arm_fn_op2_lli_s, arm_fn_tst_s);

/// TSTS llr
/// Test bits in register (Logical And), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_tsts_llr, arm_fn_op2_llr_s, true, arm_fn_tst_s);

/// TSTS lri
/// Test bits in register (Logical And), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_tsts_lri, arm_fn_op2_lri_s, arm_fn_tst_s);

/// TSTS lrr
/// Test bits in register (Logical And), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_tsts_lrr, arm_fn_op2_lrr_s, true, arm_fn_tst_s);

/// TSTS ari
/// Test bits in register (Logical And), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_tsts_ari, arm_fn_op2_ari_s, arm_fn_tst_s);

/// TSTS arr
/// Test bits in register (Logical And), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_tsts_arr, arm_fn_op2_arr_s, true, arm_fn_tst_s);

/// TSTS rri
/// Test bits in register (Logical And), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_tsts_rri, arm_fn_op2_rri_s, arm_fn_tst_s);

/// TSTS rrr
/// Test bits in register (Logical And), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_tsts_rrr, arm_fn_op2_rrr_s, true, arm_fn_tst_s);

/// LDRH ofrm
/// Load halfword
/// Negative register offset
gen_hldr!(arm_ldrh_ofrm, LDRH, PRE, DEC, HDT_REG, false);

/// LDRSB ofrm
/// Load signed byte
/// Negative register offset
gen_hldr!(arm_ldrsb_ofrm, LDRSB, PRE, DEC, HDT_REG, false);

/// LDRSH ofrm
/// Load signed halfword
/// Negative register offset
gen_hldr!(arm_ldrsh_ofrm, LDRSH, PRE, DEC, HDT_REG, false);

/// STRH prrm
/// Store halfword
/// Register offset, pre-decrement
gen_hstr!(arm_strh_prrm, STRH, PRE, DEC, HDT_REG, true);

/// TEQS lli
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_teqs_lli, arm_fn_op2_lli_s, arm_fn_teq_s);

/// TEQS llr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_teqs_llr, arm_fn_op2_llr_s, true, arm_fn_teq_s);

/// TEQS lri
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_teqs_lri, arm_fn_op2_lri_s, arm_fn_teq_s);

/// TEQS lrr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_teqs_lrr, arm_fn_op2_lrr_s, true, arm_fn_teq_s);

/// TEQS ari
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_teqs_ari, arm_fn_op2_ari_s, arm_fn_teq_s);

/// TEQS arr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_teqs_arr, arm_fn_op2_arr_s, true, arm_fn_teq_s);

/// TEQS rri
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_teqs_rri, arm_fn_op2_rri_s, arm_fn_teq_s);

/// TEQS rrr
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_teqs_rrr, arm_fn_op2_rrr_s, true, arm_fn_teq_s);

/// LDRH prrm
/// Load halfword
/// Register offset, pre-decrement
gen_hldr!(arm_ldrh_prrm, LDRH, PRE, DEC, HDT_REG, true);

/// LDRSB prrm
/// Load signed byte
/// Register offset, pre-decrement
gen_hldr!(arm_ldrsb_prrm, LDRSB, PRE, DEC, HDT_REG, true);

/// LDRSH prrm
/// Load signed halfword
/// Register offset, pre-decrement
gen_hldr!(arm_ldrsh_prrm, LDRSH, PRE, DEC, HDT_REG, true);

/// STRH ofim
/// Store halfword
/// Negative immediate offset
gen_hstr!(arm_strh_ofim, STRH, PRE, DEC, HDT_IMM, false);

/// CMPS lli
/// Compare register to value (Subtract), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_cmps_lli, arm_fn_op2_lli_s, arm_fn_cmp_s);

/// CMPS llr
/// Compare register to value (Subtract), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_cmps_llr, arm_fn_op2_llr_s, true, arm_fn_cmp_s);

/// CMPS lri
/// Compare register to value (Subtract), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_cmps_lri, arm_fn_op2_lri_s, arm_fn_cmp_s);

/// CMPS lrr
/// Compare register to value (Subtract), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_cmps_lrr, arm_fn_op2_lrr_s, true, arm_fn_cmp_s);

/// CMPS ari
/// Compare register to value (Subtract), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_cmps_ari, arm_fn_op2_ari_s, arm_fn_cmp_s);

/// CMPS arr
/// Compare register to value (Subtract), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_cmps_arr, arm_fn_op2_arr_s, true, arm_fn_cmp_s);

/// CMPS rri
/// Compare register to value (Subtract), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_cmps_rri, arm_fn_op2_rri_s, arm_fn_cmp_s);

/// CMPS rrr
/// Compare register to value (Subtract), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_cmps_rrr, arm_fn_op2_rrr_s, true, arm_fn_cmp_s);

/// LDRH ofim
/// Load halfword
/// Negative immediate offset
gen_hldr!(arm_ldrh_ofim, LDRH, PRE, DEC, HDT_IMM, false);

/// LDRSB ofim
/// Load signed byte
/// Negative immediate offset
gen_hldr!(arm_ldrsb_ofim, LDRSB, PRE, DEC, HDT_IMM, false);

/// LDRSH ofim
/// Load signed halfword
/// Negative immediate offset
gen_hldr!(arm_ldrsh_ofim, LDRSH, PRE, DEC, HDT_IMM, false);

/// STRH prim
/// Store halfword
/// Immediate offset, pre-decrement
gen_hstr!(arm_strh_prim, STRH, PRE, DEC, HDT_IMM, true);

/// CMNS lli
/// Compare register to negation of value (Add), setting flags
/// Logical shift-left by immediate
gen_dproc_nw!(arm_cmns_lli, arm_fn_op2_lli_s, arm_fn_cmn_s);

/// CMNS llr
/// Compare register to negation of value (Add), setting flags
/// Logical shift-left by register
gen_dproc_nw!(arm_cmns_llr, arm_fn_op2_llr_s, true, arm_fn_cmn_s);

/// CMNS lri
/// Compare register to negation of value (Add), setting flags
/// Logical shift-right by immediate
gen_dproc_nw!(arm_cmns_lri, arm_fn_op2_lri_s, arm_fn_cmn_s);

/// CMNS lrr
/// Compare register to negation of value (Add), setting flags
/// Logical shift-right by register
gen_dproc_nw!(arm_cmns_lrr, arm_fn_op2_lrr_s, true, arm_fn_cmn_s);

/// CMNS ari
/// Compare register to negation of value (Add), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_nw!(arm_cmns_ari, arm_fn_op2_ari_s, arm_fn_cmn_s);

/// CMNS arr
/// Compare register to negation of value (Add), setting flags
/// Arithmetic shift-right by register
gen_dproc_nw!(arm_cmns_arr, arm_fn_op2_arr_s, true, arm_fn_cmn_s);

/// CMNS rri
/// Compare register to negation of value (Add), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_nw!(arm_cmns_rri, arm_fn_op2_rri_s, arm_fn_cmn_s);

/// CMNS rrr
/// Compare register to negation of value (Add), setting flags
/// Rotate right by register
gen_dproc_nw!(arm_cmns_rrr, arm_fn_op2_rrr_s, true, arm_fn_cmn_s);

/// LDRH prim
/// Load halfword
/// Immediate offset, pre-decrement
gen_hldr!(arm_ldrh_prim, LDRH, PRE, DEC, HDT_IMM, true);

/// LDRSB prim
/// Load signed byte
/// Immediate offset, pre-decrement
gen_hldr!(arm_ldrsb_prim, LDRSB, PRE, DEC, HDT_IMM, true);

/// LDRSH prim
/// Load signed halfword
/// Immediate offset, pre-decrement
gen_hldr!(arm_ldrsh_prim, LDRSH, PRE, DEC, HDT_IMM, true);

/// ORR lli
/// Logical Or
/// Logical shift-left by immediate
gen_dproc!(arm_orr_lli, arm_fn_op2_lli, arm_fn_orr);

/// ORR llr
/// Logical Or
/// Logical shift-left by register
gen_dproc!(arm_orr_llr, arm_fn_op2_llr, true, arm_fn_orr);

/// ORR lri
/// Logical Or
/// Logical shift-right by immediate
gen_dproc!(arm_orr_lri, arm_fn_op2_lri, arm_fn_orr);

/// ORR lrr
/// Logical Or
/// Logical shift-right by register
gen_dproc!(arm_orr_lrr, arm_fn_op2_lrr, true, arm_fn_orr);

/// ORR ari
/// Logical Or
/// Arithmetic shift-right by immediate
gen_dproc!(arm_orr_ari, arm_fn_op2_ari, arm_fn_orr);

/// ORR arr
/// Logical Or
/// Arithmetic shift-right by register
gen_dproc!(arm_orr_arr, arm_fn_op2_arr, true, arm_fn_orr);

/// ORR rri
/// Logical Or
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_orr_rri, arm_fn_op2_rri, arm_fn_orr);

/// ORR rrr
/// Logical Or
/// Rotate right by register
gen_dproc!(arm_orr_rrr, arm_fn_op2_rrr, true, arm_fn_orr);

/// STRH ofrp
/// Store halfword
/// Positive register offset
gen_hstr!(arm_strh_ofrp, STRH, PRE, INC, HDT_REG, false);

/// ORRS lli
/// Logical Or, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_orrs_lli, arm_fn_op2_lli_s, arm_fn_orr_s);

/// ORRS llr
/// Logical Or, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_orrs_llr, arm_fn_op2_llr_s, true, arm_fn_orr_s);

/// ORRS lri
/// Logical Or, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_orrs_lri, arm_fn_op2_lri_s, arm_fn_orr_s);

/// ORRS lrr
/// Logical Or, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_orrs_lrr, arm_fn_op2_lrr_s, true, arm_fn_orr_s);

/// ORRS ari
/// Logical Or, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_orrs_ari, arm_fn_op2_ari_s, arm_fn_orr_s);

/// ORRS arr
/// Logical Or, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_orrs_arr, arm_fn_op2_arr_s, true, arm_fn_orr_s);

/// ORRS rri
/// Logical Or, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_orrs_rri, arm_fn_op2_rri_s, arm_fn_orr_s);

/// ORRS rrr
/// Logical Or, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_orrs_rrr, arm_fn_op2_rrr_s, true, arm_fn_orr_s);

/// LDRH ofrp
/// Load halfword
/// Positive register offset
gen_hldr!(arm_ldrh_ofrp, LDRH, PRE, INC, HDT_REG, false);

/// LDRSB ofrp
/// Load signed byte
/// Positive register offset
gen_hldr!(arm_ldrsb_ofrp, LDRSB, PRE, INC, HDT_REG, false);

/// LDRSH ofrp
/// Load signed halfword
/// Positive register offset
gen_hldr!(arm_ldrsh_ofrp, LDRSH, PRE, INC, HDT_REG, false);

/// MOV lli
/// Move value to a register
/// Logical shift-left by immediate
gen_dproc!(arm_mov_lli, arm_fn_op2_lli, arm_fn_mov);

/// MOV llr
/// Move value to a register
/// Logical shift-left by register
gen_dproc!(arm_mov_llr, arm_fn_op2_llr, true, arm_fn_mov);

/// MOV lri
/// Move value to a register
/// Logical shift-right by immediate
gen_dproc!(arm_mov_lri, arm_fn_op2_lri, arm_fn_mov);

/// MOV lrr
/// Move value to a register
/// Logical shift-right by register
gen_dproc!(arm_mov_lrr, arm_fn_op2_lrr, true, arm_fn_mov);

/// MOV ari
/// Move value to a register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_mov_ari, arm_fn_op2_ari, arm_fn_mov);

/// MOV arr
/// Move value to a register
/// Arithmetic shift-right by register
gen_dproc!(arm_mov_arr, arm_fn_op2_arr, true, arm_fn_mov);

/// MOV rri
/// Move value to a register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_mov_rri, arm_fn_op2_rri, arm_fn_mov);

/// MOV rrr
/// Move value to a register
/// Rotate right by register
gen_dproc!(arm_mov_rrr, arm_fn_op2_rrr, true, arm_fn_mov);

/// STRH prrp
/// Store halfword
/// Register offset, pre-increment
gen_hstr!(arm_strh_prrp, STRH, PRE, INC, HDT_REG, true);

/// MOVS lli
/// Move value to a register, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_movs_lli, arm_fn_op2_lli_s, arm_fn_mov_s);

/// MOVS llr
/// Move value to a register, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_movs_llr, arm_fn_op2_llr_s, true, arm_fn_mov_s);

/// MOVS lri
/// Move value to a register, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_movs_lri, arm_fn_op2_lri_s, arm_fn_mov_s);

/// MOVS lrr
/// Move value to a register, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_movs_lrr, arm_fn_op2_lrr_s, true, arm_fn_mov_s);

/// MOVS ari
/// Move value to a register, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_movs_ari, arm_fn_op2_ari_s, arm_fn_mov_s);

/// MOVS arr
/// Move value to a register, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_movs_arr, arm_fn_op2_arr_s, true, arm_fn_mov_s);

/// MOVS rri
/// Move value to a register, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_movs_rri, arm_fn_op2_rri_s, arm_fn_mov_s);

/// MOVS rrr
/// Move value to a register, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_movs_rrr, arm_fn_op2_rrr_s, true, arm_fn_mov_s);

/// LDRH prrp
/// Load halfword
/// Register offset, pre-increment
gen_hldr!(arm_ldrh_prrp, LDRH, PRE, INC, HDT_REG, true);

/// LDRSB prrp
/// Load signed byte
/// Register offset, pre-increment
gen_hldr!(arm_ldrsb_prrp, LDRSB, PRE, INC, HDT_REG, true);

/// LDRSH prrp
/// Load signed halfword
/// Register offset, pre-increment
gen_hldr!(arm_ldrsh_prrp, LDRSH, PRE, INC, HDT_REG, true);

/// BIC lli
/// Clear bits in register (NAND)
/// Logical shift-left by immediate
gen_dproc!(arm_bic_lli, arm_fn_op2_lli, arm_fn_bic);

/// BIC llr
/// Clear bits in register (NAND)
/// Logical shift-left by register
gen_dproc!(arm_bic_llr, arm_fn_op2_llr, true, arm_fn_bic);

/// BIC lri
/// Clear bits in register (NAND)
/// Logical shift-right by immediate
gen_dproc!(arm_bic_lri, arm_fn_op2_lri, arm_fn_bic);

/// BIC lrr
/// Clear bits in register (NAND)
/// Logical shift-right by register
gen_dproc!(arm_bic_lrr, arm_fn_op2_lrr, true, arm_fn_bic);

/// BIC ari
/// Clear bits in register (NAND)
/// Arithmetic shift-right by immediate
gen_dproc!(arm_bic_ari, arm_fn_op2_ari, arm_fn_bic);

/// BIC arr
/// Clear bits in register (NAND)
/// Arithmetic shift-right by register
gen_dproc!(arm_bic_arr, arm_fn_op2_arr, true, arm_fn_bic);

/// BIC rri
/// Clear bits in register (NAND)
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_bic_rri, arm_fn_op2_rri, arm_fn_bic);

/// BIC rrr
/// Clear bits in register (NAND)
/// Rotate right by register
gen_dproc!(arm_bic_rrr, arm_fn_op2_rrr, true, arm_fn_bic);

/// STRH ofip
/// Store halfword
/// Positive immediate offset
gen_hstr!(arm_strh_ofip, STRH, PRE, INC, HDT_IMM, false);

/// BICS lli
/// Clear bits in register (NAND), setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_bics_lli, arm_fn_op2_lli_s, arm_fn_bic_s);

/// BICS llr
/// Clear bits in register (NAND), setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_bics_llr, arm_fn_op2_llr_s, true, arm_fn_bic_s);

/// BICS lri
/// Clear bits in register (NAND), setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_bics_lri, arm_fn_op2_lri_s, arm_fn_bic_s);

/// BICS lrr
/// Clear bits in register (NAND), setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_bics_lrr, arm_fn_op2_lrr_s, true, arm_fn_bic_s);

/// BICS ari
/// Clear bits in register (NAND), setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_bics_ari, arm_fn_op2_ari_s, arm_fn_bic_s);

/// BICS arr
/// Clear bits in register (NAND), setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_bics_arr, arm_fn_op2_arr_s, true, arm_fn_bic_s);

/// BICS rri
/// Clear bits in register (NAND), setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_bics_rri, arm_fn_op2_rri_s, arm_fn_bic_s);

/// BICS rrr
/// Clear bits in register (NAND), setting flags
/// Rotate right by register
gen_dproc_sf!(arm_bics_rrr, arm_fn_op2_rrr_s, true, arm_fn_bic_s);

/// LDRH ofip
/// Load halfword
/// Positive immediate offset
gen_hldr!(arm_ldrh_ofip, LDRH, PRE, INC, HDT_IMM, false);

/// LDRSB ofip
/// Load signed byte
/// Positive immediate offset
gen_hldr!(arm_ldrsb_ofip, LDRSB, PRE, INC, HDT_IMM, false);

/// LDRSH ofip
/// Load signed halfword
/// Positive immediate offset
gen_hldr!(arm_ldrsh_ofip, LDRSH, PRE, INC, HDT_IMM, false);

/// MVN lli
/// Move negation of value to a register
/// Logical shift-left by immediate
gen_dproc!(arm_mvn_lli, arm_fn_op2_lli, arm_fn_mvn);

/// MVN llr
/// Move negation of value to a register
/// Logical shift-left by register
gen_dproc!(arm_mvn_llr, arm_fn_op2_llr, true, arm_fn_mvn);

/// MVN lri
/// Move negation of value to a register
/// Logical shift-right by immediate
gen_dproc!(arm_mvn_lri, arm_fn_op2_lri, arm_fn_mvn);

/// MVN lrr
/// Move negation of value to a register
/// Logical shift-right by register
gen_dproc!(arm_mvn_lrr, arm_fn_op2_lrr, true, arm_fn_mvn);

/// MVN ari
/// Move negation of value to a register
/// Arithmetic shift-right by immediate
gen_dproc!(arm_mvn_ari, arm_fn_op2_ari, arm_fn_mvn);

/// MVN arr
/// Move negation of value to a register
/// Arithmetic shift-right by register
gen_dproc!(arm_mvn_arr, arm_fn_op2_arr, true, arm_fn_mvn);

/// MVN rri
/// Move negation of value to a register
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc!(arm_mvn_rri, arm_fn_op2_rri, arm_fn_mvn);

/// MVN rrr
/// Move negation of value to a register
/// Rotate right by register
gen_dproc!(arm_mvn_rrr, arm_fn_op2_rrr, true, arm_fn_mvn);

/// STRH prip
/// Store halfword
/// Immediate offset, pre-increment
gen_hstr!(arm_strh_prip, STRH, PRE, INC, HDT_IMM, true);

/// MVNS lli
/// Move negation of value to a register, setting flags
/// Logical shift-left by immediate
gen_dproc_sf!(arm_mvns_lli, arm_fn_op2_lli_s, arm_fn_mvn_s);

/// MVNS llr
/// Move negation of value to a register, setting flags
/// Logical shift-left by register
gen_dproc_sf!(arm_mvns_llr, arm_fn_op2_llr_s, true, arm_fn_mvn_s);

/// MVNS lri
/// Move negation of value to a register, setting flags
/// Logical shift-right by immediate
gen_dproc_sf!(arm_mvns_lri, arm_fn_op2_lri_s, arm_fn_mvn_s);

/// MVNS lrr
/// Move negation of value to a register, setting flags
/// Logical shift-right by register
gen_dproc_sf!(arm_mvns_lrr, arm_fn_op2_lrr_s, true, arm_fn_mvn_s);

/// MVNS ari
/// Move negation of value to a register, setting flags
/// Arithmetic shift-right by immediate
gen_dproc_sf!(arm_mvns_ari, arm_fn_op2_ari_s, arm_fn_mvn_s);

/// MVNS arr
/// Move negation of value to a register, setting flags
/// Arithmetic shift-right by register
gen_dproc_sf!(arm_mvns_arr, arm_fn_op2_arr_s, true, arm_fn_mvn_s);

/// MVNS rri
/// Move negation of value to a register, setting flags
/// Rotate right by immediate, or rotate right with extend (RRX)
gen_dproc_sf!(arm_mvns_rri, arm_fn_op2_rri_s, arm_fn_mvn_s);

/// MVNS rrr
/// Move negation of value to a register, setting flags
/// Rotate right by register
gen_dproc_sf!(arm_mvns_rrr, arm_fn_op2_rrr_s, true, arm_fn_mvn_s);

/// LDRH prip
/// Load halfword
/// Immediate offset, pre-increment
gen_hldr!(arm_ldrh_prip, LDRH, PRE, INC, HDT_IMM, true);

/// LDRSB prip
/// Load signed byte
/// Immediate offset, pre-increment
gen_hldr!(arm_ldrsb_prip, LDRSB, PRE, INC, HDT_IMM, true);

/// LDRSH prip
/// Load signed halfword
/// Immediate offset, pre-increment
gen_hldr!(arm_ldrsh_prip, LDRSH, PRE, INC, HDT_IMM, true);

/// AND imm
/// Logical And
/// Immediate value
gen_dproc!(arm_and_imm, arm_fn_op2_imm, arm_fn_and);

/// ANDS imm
/// Logical And, setting flags
/// Immediate value
gen_dproc_sf!(arm_ands_imm, arm_fn_op2_imm_s, arm_fn_and_s);

/// EOR imm
/// Logical Exclusive-or
/// Immediate value
gen_dproc!(arm_eor_imm, arm_fn_op2_imm, arm_fn_eor);

/// EORS imm
/// Logical Exclusive-or, setting flags
/// Immediate value
gen_dproc_sf!(arm_eors_imm, arm_fn_op2_imm_s, arm_fn_eor_s);

/// SUB imm
/// Subtract from register
/// Immediate value
gen_dproc!(arm_sub_imm, arm_fn_op2_imm, arm_fn_sub);

/// SUBS imm
/// Subtract, setting flags
/// Immediate value
gen_dproc_sf!(arm_subs_imm, arm_fn_op2_imm_s, arm_fn_sub_s);

/// RSB imm
/// Subtract register from value
/// Immediate value
gen_dproc!(arm_rsb_imm, arm_fn_op2_imm, arm_fn_rsb);

/// RSBS imm
/// Reverse Subtract, setting flags
/// Immediate value
gen_dproc_sf!(arm_rsbs_imm, arm_fn_op2_imm_s, arm_fn_rsb_s);

/// ADD imm
/// Add to register
/// Immediate value
gen_dproc!(arm_add_imm, arm_fn_op2_imm, arm_fn_add);

/// ADDS imm
/// Add to register, setting flags
/// Immediate value
gen_dproc_sf!(arm_adds_imm, arm_fn_op2_imm_s, arm_fn_add_s);

/// ADC imm
/// Add to register with carry
/// Immediate value
gen_dproc!(arm_adc_imm, arm_fn_op2_imm, arm_fn_adc);

/// ADCS imm
/// Add to register with carry, setting flags
/// Immediate value
gen_dproc_sf!(arm_adcs_imm, arm_fn_op2_imm_s, arm_fn_adc_s);

/// SBC imm
/// Subtract from register with borrow
/// Immediate value
gen_dproc!(arm_sbc_imm, arm_fn_op2_imm, arm_fn_sbc);

/// SBCS imm
/// Subtract from register with borrow, setting flags
/// Immediate value
gen_dproc_sf!(arm_sbcs_imm, arm_fn_op2_imm_s, arm_fn_sbc_s);

/// RSC imm
/// Subtract register from value with borrow
/// Immediate value
gen_dproc!(arm_rsc_imm, arm_fn_op2_imm, arm_fn_rsc);

/// RSCS imm
/// Subtract register from value with borrow, setting flags
/// Immediate value
gen_dproc_sf!(arm_rscs_imm, arm_fn_op2_imm_s, arm_fn_rsc_s);

/// TSTS imm
/// Test bits in register (Logical And), setting flags
/// Immediate value
gen_dproc_nw!(arm_tsts_imm, arm_fn_op2_imm_s, arm_fn_tst_s);

/// TEQS imm
/// Test equivalence of bits in register (Logical Exclusive-or), setting flags
/// Immediate value
gen_dproc_nw!(arm_teqs_imm, arm_fn_op2_imm_s, arm_fn_teq_s);

/// CMPS imm
/// Compare register to value (Subtract), setting flags
/// Immediate value
gen_dproc_nw!(arm_cmps_imm, arm_fn_op2_imm_s, arm_fn_cmp_s);


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
gen_dproc_sf!(arm_orrs_imm, arm_fn_op2_imm_s, arm_fn_orr_s);

/// MOV imm
/// Move value to a register
/// Immediate value
gen_dproc!(arm_mov_imm, arm_fn_op2_imm, arm_fn_mov);

/// MOVS imm
/// Move value to a register, setting flags
/// Immediate value
gen_dproc_sf!(arm_movs_imm, arm_fn_op2_imm_s, arm_fn_mov_s);

/// BIC imm
/// Clear bits in register (NAND)
/// Immediate value
gen_dproc!(arm_bic_imm, arm_fn_op2_imm, arm_fn_bic);

/// BICS imm
/// Clear bits in register (NAND), setting flags
/// Immediate value
gen_dproc_sf!(arm_bics_imm, arm_fn_op2_imm_s, arm_fn_bic_s);

/// MVN imm
/// Move negation of value to a register
/// Immediate value
gen_dproc!(arm_mvn_imm, arm_fn_op2_imm, arm_fn_mvn);

/// MVNS imm
/// Move negation of value to a register, setting flags
/// Immediate value
gen_dproc_sf!(arm_mvns_imm, arm_fn_op2_imm_s, arm_fn_mvn_s);

/// STR ptim
gen_str!(arm_str_ptim, STR, POST, DEC, SDT_IMM, false, false);

/// LDR ptim
gen_ldr!(arm_ldr_ptim, LDR, POST, DEC, SDT_IMM, false, false);

/// STRT ptim
gen_str!(arm_strt_ptim, STR, POST, DEC, SDT_IMM, false, true);

/// LDRT ptim
gen_ldr!(arm_ldrt_ptim, LDR, POST, DEC, SDT_IMM, false, true);

/// STRB ptim
gen_str!(arm_strb_ptim, STRB, POST, DEC, SDT_IMM, false, false);

/// LDRB ptim
gen_ldr!(arm_ldrb_ptim, LDRB, POST, DEC, SDT_IMM, false, false);

/// STRBT ptim
gen_str!(arm_strbt_ptim, STRB, POST, DEC, SDT_IMM, false, true);

/// LDRBT ptim
gen_ldr!(arm_ldrbt_ptim, LDRB, POST, DEC, SDT_IMM, false, true);

/// STR ptip
gen_str!(arm_str_ptip, STR, POST, INC, SDT_IMM, false, false);

/// LDR ptip
gen_ldr!(arm_ldr_ptip, LDR, POST, INC, SDT_IMM, false, false);

/// STRT ptip
gen_str!(arm_strt_ptip, STR, POST, INC, SDT_IMM, false, true);

/// LDRT ptip
gen_ldr!(arm_ldrt_ptip, LDR, POST, INC, SDT_IMM, false, true);

/// STRB ptip
gen_str!(arm_strb_ptip, STRB, POST, INC, SDT_IMM, false, false);

/// LDRB ptip
gen_ldr!(arm_ldrb_ptip, LDRB, POST, INC, SDT_IMM, false, false);

/// STRBT ptip
gen_str!(arm_strbt_ptip, STRB, POST, INC, SDT_IMM, false, true);

/// LDRBT ptip
gen_ldr!(arm_ldrbt_ptip, LDRB, POST, INC, SDT_IMM, false, true);

/// STR ofim
gen_str!(arm_str_ofim, STR, PRE, DEC, SDT_NEG_IMM, false, false);

/// LDR ofim
gen_ldr!(arm_ldr_ofim, LDR, PRE, DEC, SDT_NEG_IMM, false, false);

/// STR prim
gen_str!(arm_str_prim, STR, PRE, DEC, SDT_IMM, true, false);

/// LDR prim
gen_ldr!(arm_ldr_prim, LDR, PRE, DEC, SDT_IMM, true, false);

/// STRB ofim
gen_str!(arm_strb_ofim, STRB, PRE, DEC, SDT_NEG_IMM, false, false);

/// LDRB ofim
gen_ldr!(arm_ldrb_ofim, LDRB, PRE, DEC, SDT_NEG_IMM, false, false);

/// STRB prim
gen_str!(arm_strb_prim, STRB, PRE, DEC, SDT_IMM, true, false);

/// LDRB prim
gen_ldr!(arm_ldrb_prim, LDRB, PRE, DEC, SDT_IMM, true, false);

/// STR ofip
gen_str!(arm_str_ofip, STR, PRE, INC, SDT_POS_IMM, false, false);

/// LDR ofip
gen_ldr!(arm_ldr_ofip, LDR, PRE, INC, SDT_POS_IMM, false, false);

/// STR prip
gen_str!(arm_str_prip, STR, PRE, INC, SDT_IMM, true, false);

/// LDR prip
gen_ldr!(arm_ldr_prip, LDR, PRE, INC, SDT_IMM, true, false);

/// STRB ofip
gen_str!(arm_strb_ofip, STRB, PRE, INC, SDT_POS_IMM, false, false);

/// LDRB ofip
gen_ldr!(arm_ldrb_ofip, LDRB, PRE, INC, SDT_POS_IMM, false, false);

/// STRB prip
gen_str!(arm_strb_prip, STRB, PRE, INC, SDT_IMM, true, false);

/// LDRB prip
gen_ldr!(arm_ldrb_prip, LDRB, PRE, INC, SDT_IMM, true, false);

/// STR ptrmll
gen_str!(arm_str_ptrmll, STR, POST, DEC, SDT_LSL, false, false);

/// STR ptrmlr
gen_str!(arm_str_ptrmlr, STR, POST, DEC, SDT_LSR, false, false);

/// STR ptrmar
gen_str!(arm_str_ptrmar, STR, POST, DEC, SDT_ASR, false, false);

/// STR ptrmrr
gen_str!(arm_str_ptrmrr, STR, POST, DEC, SDT_ROR, false, false);

/// LDR ptrmll
gen_ldr!(arm_ldr_ptrmll, LDR, POST, DEC, SDT_LSL, false, false);

/// LDR ptrmlr
gen_ldr!(arm_ldr_ptrmlr, LDR, POST, DEC, SDT_LSR, false, false);

/// LDR ptrmar
gen_ldr!(arm_ldr_ptrmar, LDR, POST, DEC, SDT_ASR, false, false);

/// LDR ptrmrr
gen_ldr!(arm_ldr_ptrmrr, LDR, POST, DEC, SDT_ROR, false, false);

/// STRT ptrmll
gen_str!(arm_strt_ptrmll, STR, POST, DEC, SDT_LSL, false, true);

/// STRT ptrmlr
gen_str!(arm_strt_ptrmlr, STR, POST, DEC, SDT_LSR, false, true);

/// STRT ptrmar
gen_str!(arm_strt_ptrmar, STR, POST, DEC, SDT_ASR, false, true);

/// STRT ptrmrr
gen_str!(arm_strt_ptrmrr, STR, POST, DEC, SDT_ROR, false, true);

/// LDRT ptrmll
gen_ldr!(arm_ldrt_ptrmll, LDR, POST, DEC, SDT_LSL, false, true);

/// LDRT ptrmlr
gen_ldr!(arm_ldrt_ptrmlr, LDR, POST, DEC, SDT_LSR, false, true);

/// LDRT ptrmar
gen_ldr!(arm_ldrt_ptrmar, LDR, POST, DEC, SDT_ASR, false, true);

/// LDRT ptrmrr
gen_ldr!(arm_ldrt_ptrmrr, LDR, POST, DEC, SDT_ROR, false, true);

/// STRB ptrmll
gen_str!(arm_strb_ptrmll, STRB, POST, DEC, SDT_LSL, false, false);

/// STRB ptrmlr
gen_str!(arm_strb_ptrmlr, STRB, POST, DEC, SDT_LSR, false, false);

/// STRB ptrmar
gen_str!(arm_strb_ptrmar, STRB, POST, DEC, SDT_ASR, false, false);

/// STRB ptrmrr
gen_str!(arm_strb_ptrmrr, STRB, POST, DEC, SDT_ROR, false, false);

/// LDRB ptrmll
gen_ldr!(arm_ldrb_ptrmll, LDRB, POST, DEC, SDT_LSL, false, false);

/// LDRB ptrmlr
gen_ldr!(arm_ldrb_ptrmlr, LDRB, POST, DEC, SDT_LSR, false, false);

/// LDRB ptrmar
gen_ldr!(arm_ldrb_ptrmar, LDRB, POST, DEC, SDT_ASR, false, false);

/// LDRB ptrmrr
gen_ldr!(arm_ldrb_ptrmrr, LDRB, POST, DEC, SDT_ROR, false, false);

/// STRBT ptrmll
gen_str!(arm_strbt_ptrmll, STRB, POST, DEC, SDT_LSL, false, true);

/// STRBT ptrmlr
gen_str!(arm_strbt_ptrmlr, STRB, POST, DEC, SDT_LSR, false, true);

/// STRBT ptrmar
gen_str!(arm_strbt_ptrmar, STRB, POST, DEC, SDT_ASR, false, true);

/// STRBT ptrmrr
gen_str!(arm_strbt_ptrmrr, STRB, POST, DEC, SDT_ROR, false, true);

/// LDRBT ptrmll
gen_ldr!(arm_ldrbt_ptrmll, LDRB, POST, DEC, SDT_LSL, false, true);

/// LDRBT ptrmlr
gen_ldr!(arm_ldrbt_ptrmlr, LDRB, POST, DEC, SDT_LSR, false, true);

/// LDRBT ptrmar
gen_ldr!(arm_ldrbt_ptrmar, LDRB, POST, DEC, SDT_ASR, false, true);

/// LDRBT ptrmrr
gen_ldr!(arm_ldrbt_ptrmrr, LDRB, POST, DEC, SDT_ROR, false, true);

/// STR ptrpll
gen_str!(arm_str_ptrpll, STR, POST, INC, SDT_LSL, false, false);

/// STR ptrplr
gen_str!(arm_str_ptrplr, STR, POST, INC, SDT_LSR, false, false);

/// STR ptrpar
gen_str!(arm_str_ptrpar, STR, POST, INC, SDT_ASR, false, false);

/// STR ptrprr
gen_str!(arm_str_ptrprr, STR, POST, INC, SDT_ROR, false, false);

/// LDR ptrpll
gen_ldr!(arm_ldr_ptrpll, LDR, POST, INC, SDT_LSL, false, false);

/// LDR ptrplr
gen_ldr!(arm_ldr_ptrplr, LDR, POST, INC, SDT_LSR, false, false);

/// LDR ptrpar
gen_ldr!(arm_ldr_ptrpar, LDR, POST, INC, SDT_ASR, false, false);

/// LDR ptrprr
gen_ldr!(arm_ldr_ptrprr, LDR, POST, INC, SDT_ROR, false, false);

/// STRT ptrpll
gen_str!(arm_strt_ptrpll, STR, POST, INC, SDT_LSL, false, true);

/// STRT ptrplr
gen_str!(arm_strt_ptrplr, STR, POST, INC, SDT_LSR, false, true);

/// STRT ptrpar
gen_str!(arm_strt_ptrpar, STR, POST, INC, SDT_ASR, false, true);

/// STRT ptrprr
gen_str!(arm_strt_ptrprr, STR, POST, INC, SDT_ROR, false, true);

/// LDRT ptrpll
gen_ldr!(arm_ldrt_ptrpll, LDR, POST, INC, SDT_LSL, false, true);

/// LDRT ptrplr
gen_ldr!(arm_ldrt_ptrplr, LDR, POST, INC, SDT_LSR, false, true);

/// LDRT ptrpar
gen_ldr!(arm_ldrt_ptrpar, LDR, POST, INC, SDT_ASR, false, true);

/// LDRT ptrprr
gen_ldr!(arm_ldrt_ptrprr, LDR, POST, INC, SDT_ROR, false, true);

/// STRB ptrpll
gen_str!(arm_strb_ptrpll, STRB, POST, INC, SDT_LSL, false, false);

/// STRB ptrplr
gen_str!(arm_strb_ptrplr, STRB, POST, INC, SDT_LSR, false, false);

/// STRB ptrpar
gen_str!(arm_strb_ptrpar, STRB, POST, INC, SDT_ASR, false, false);

/// STRB ptrprr
gen_str!(arm_strb_ptrprr, STRB, POST, INC, SDT_ROR, false, false);

/// LDRB ptrpll
gen_ldr!(arm_ldrb_ptrpll, LDRB, POST, INC, SDT_LSL, false, false);

/// LDRB ptrplr
gen_ldr!(arm_ldrb_ptrplr, LDRB, POST, INC, SDT_LSR, false, false);

/// LDRB ptrpar
gen_ldr!(arm_ldrb_ptrpar, LDRB, POST, INC, SDT_ASR, false, false);

/// LDRB ptrprr
gen_ldr!(arm_ldrb_ptrprr, LDRB, POST, INC, SDT_ROR, false, false);

/// STRBT ptrpll
gen_str!(arm_strbt_ptrpll, STRB, POST, INC, SDT_LSL, false, true);

/// STRBT ptrplr
gen_str!(arm_strbt_ptrplr, STRB, POST, INC, SDT_LSR, false, true);

/// STRBT ptrpar
gen_str!(arm_strbt_ptrpar, STRB, POST, INC, SDT_ASR, false, true);

/// STRBT ptrprr
gen_str!(arm_strbt_ptrprr, STRB, POST, INC, SDT_ROR, false, true);

/// LDRBT ptrpll
gen_ldr!(arm_ldrbt_ptrpll, LDRB, POST, INC, SDT_LSL, false, true);

/// LDRBT ptrplr
gen_ldr!(arm_ldrbt_ptrplr, LDRB, POST, INC, SDT_LSR, false, true);

/// LDRBT ptrpar
gen_ldr!(arm_ldrbt_ptrpar, LDRB, POST, INC, SDT_ASR, false, true);

/// LDRBT ptrprr
gen_ldr!(arm_ldrbt_ptrprr, LDRB, POST, INC, SDT_ROR, false, true);

/// STR ofrmll
gen_str!(arm_str_ofrmll, STR, PRE, DEC, SDT_NEG_LSL, false, false);

/// STR ofrmlr
gen_str!(arm_str_ofrmlr, STR, PRE, DEC, SDT_NEG_LSR, false, false);

/// STR ofrmar
gen_str!(arm_str_ofrmar, STR, PRE, DEC, SDT_NEG_ASR, false, false);

/// STR ofrmrr
gen_str!(arm_str_ofrmrr, STR, PRE, DEC, SDT_NEG_ROR, false, false);

/// LDR ofrmll
gen_ldr!(arm_ldr_ofrmll, LDR, PRE, DEC, SDT_NEG_LSL, false, false);

/// LDR ofrmlr
gen_ldr!(arm_ldr_ofrmlr, LDR, PRE, DEC, SDT_NEG_LSR, false, false);

/// LDR ofrmar
gen_ldr!(arm_ldr_ofrmar, LDR, PRE, DEC, SDT_NEG_ASR, false, false);

/// LDR ofrmrr
gen_ldr!(arm_ldr_ofrmrr, LDR, PRE, DEC, SDT_NEG_ROR, false, false);

/// STR prrmll
gen_str!(arm_str_prrmll, STR, PRE, DEC, SDT_LSL, true, false);

/// STR prrmlr
gen_str!(arm_str_prrmlr, STR, PRE, DEC, SDT_LSR, true, false);

/// STR prrmar
gen_str!(arm_str_prrmar, STR, PRE, DEC, SDT_ASR, true, false);

/// STR prrmrr
gen_str!(arm_str_prrmrr, STR, PRE, DEC, SDT_ROR, true, false);

/// LDR prrmll
gen_ldr!(arm_ldr_prrmll, LDR, PRE, DEC, SDT_LSL, true, false);

/// LDR prrmlr
gen_ldr!(arm_ldr_prrmlr, LDR, PRE, DEC, SDT_LSR, true, false);

/// LDR prrmar
gen_ldr!(arm_ldr_prrmar, LDR, PRE, DEC, SDT_ASR, true, false);

/// LDR prrmrr
gen_ldr!(arm_ldr_prrmrr, LDR, PRE, DEC, SDT_ROR, true, false);

/// STRB ofrmll
gen_str!(arm_strb_ofrmll, STRB, PRE, DEC, SDT_NEG_LSL, false, false);

/// STRB ofrmlr
gen_str!(arm_strb_ofrmlr, STRB, PRE, DEC, SDT_NEG_LSR, false, false);

/// STRB ofrmar
gen_str!(arm_strb_ofrmar, STRB, PRE, DEC, SDT_NEG_ASR, false, false);

/// STRB ofrmrr
gen_str!(arm_strb_ofrmrr, STRB, PRE, DEC, SDT_NEG_ROR, false, false);

/// LDRB ofrmll
gen_ldr!(arm_ldrb_ofrmll, LDRB, PRE, DEC, SDT_NEG_LSL, false, false);

/// LDRB ofrmlr
gen_ldr!(arm_ldrb_ofrmlr, LDRB, PRE, DEC, SDT_NEG_LSR, false, false);

/// LDRB ofrmar
gen_ldr!(arm_ldrb_ofrmar, LDRB, PRE, DEC, SDT_NEG_ASR, false, false);

/// LDRB ofrmrr
gen_ldr!(arm_ldrb_ofrmrr, LDRB, PRE, DEC, SDT_NEG_ROR, false, false);

/// STRB prrmll
gen_str!(arm_strb_prrmll, STRB, PRE, DEC, SDT_LSL, true, false);

/// STRB prrmlr
gen_str!(arm_strb_prrmlr, STRB, PRE, DEC, SDT_LSR, true, false);

/// STRB prrmar
gen_str!(arm_strb_prrmar, STRB, PRE, DEC, SDT_ASR, true, false);

/// STRB prrmrr
gen_str!(arm_strb_prrmrr, STRB, PRE, DEC, SDT_ROR, true, false);

/// LDRB prrmll
gen_ldr!(arm_ldrb_prrmll, LDRB, PRE, DEC, SDT_LSL, true, false);

/// LDRB prrmlr
gen_ldr!(arm_ldrb_prrmlr, LDRB, PRE, DEC, SDT_LSR, true, false);

/// LDRB prrmar
gen_ldr!(arm_ldrb_prrmar, LDRB, PRE, DEC, SDT_ASR, true, false);

/// LDRB prrmrr
gen_ldr!(arm_ldrb_prrmrr, LDRB, PRE, DEC, SDT_ROR, true, false);

/// STR ofrpll
gen_str!(arm_str_ofrpll, STR, PRE, INC, SDT_POS_LSL, false, false);

/// STR ofrplr
gen_str!(arm_str_ofrplr, STR, PRE, INC, SDT_POS_LSR, false, false);

/// STR ofrpar
gen_str!(arm_str_ofrpar, STR, PRE, INC, SDT_POS_ASR, false, false);

/// STR ofrprr
gen_str!(arm_str_ofrprr, STR, PRE, INC, SDT_POS_ROR, false, false);

/// LDR ofrpll
gen_ldr!(arm_ldr_ofrpll, LDR, PRE, INC, SDT_POS_LSL, false, false);

/// LDR ofrplr
gen_ldr!(arm_ldr_ofrplr, LDR, PRE, INC, SDT_POS_LSR, false, false);

/// LDR ofrpar
gen_ldr!(arm_ldr_ofrpar, LDR, PRE, INC, SDT_POS_ASR, false, false);

/// LDR ofrprr
gen_ldr!(arm_ldr_ofrprr, LDR, PRE, INC, SDT_POS_ROR, false, false);

/// STR prrpll
gen_str!(arm_str_prrpll, STR, PRE, INC, SDT_LSL, true, false);

/// STR prrplr
gen_str!(arm_str_prrplr, STR, PRE, INC, SDT_LSR, true, false);

/// STR prrpar
gen_str!(arm_str_prrpar, STR, PRE, INC, SDT_ASR, true, false);

/// STR prrprr
gen_str!(arm_str_prrprr, STR, PRE, INC, SDT_ROR, true, false);

/// LDR prrpll
gen_ldr!(arm_ldr_prrpll, LDR, PRE, INC, SDT_LSL, true, false);

/// LDR prrplr
gen_ldr!(arm_ldr_prrplr, LDR, PRE, INC, SDT_LSR, true, false);

/// LDR prrpar
gen_ldr!(arm_ldr_prrpar, LDR, PRE, INC, SDT_ASR, true, false);

/// LDR prrprr
gen_ldr!(arm_ldr_prrprr, LDR, PRE, INC, SDT_ROR, true, false);

/// STRB ofrpll
gen_str!(arm_strb_ofrpll, STRB, PRE, INC, SDT_POS_LSL, false, false);

/// STRB ofrplr
gen_str!(arm_strb_ofrplr, STRB, PRE, INC, SDT_POS_LSR, false, false);

/// STRB ofrpar
gen_str!(arm_strb_ofrpar, STRB, PRE, INC, SDT_POS_ASR, false, false);

/// STRB ofrprr
gen_str!(arm_strb_ofrprr, STRB, PRE, INC, SDT_POS_ROR, false, false);

/// LDRB ofrpll
gen_ldr!(arm_ldrb_ofrpll, LDRB, PRE, INC, SDT_POS_LSL, false, false);

/// LDRB ofrplr
gen_ldr!(arm_ldrb_ofrplr, LDRB, PRE, INC, SDT_POS_LSR, false, false);

/// LDRB ofrpar
gen_ldr!(arm_ldrb_ofrpar, LDRB, PRE, INC, SDT_POS_ASR, false, false);

/// LDRB ofrprr
gen_ldr!(arm_ldrb_ofrprr, LDRB, PRE, INC, SDT_POS_ROR, false, false);

/// STRB prrpll
gen_str!(arm_strb_prrpll, STRB, PRE, INC, SDT_LSL, true, false);

/// STRB prrplr
gen_str!(arm_strb_prrplr, STRB, PRE, INC, SDT_LSR, true, false);

/// STRB prrpar
gen_str!(arm_strb_prrpar, STRB, PRE, INC, SDT_ASR, true, false);

/// STRB prrprr
gen_str!(arm_strb_prrprr, STRB, PRE, INC, SDT_ROR, true, false);

/// LDRB prrpll
gen_ldr!(arm_ldrb_prrpll, LDRB, PRE, INC, SDT_LSL, true, false);

/// LDRB prrplr
gen_ldr!(arm_ldrb_prrplr, LDRB, PRE, INC, SDT_LSR, true, false);

/// LDRB prrpar
gen_ldr!(arm_ldrb_prrpar, LDRB, PRE, INC, SDT_ASR, true, false);

/// LDRB prrprr
gen_ldr!(arm_ldrb_prrprr, LDRB, PRE, INC, SDT_ROR, true, false);

/// STMDA 
/// Store multiple words, decrement after
gen_stm!(arm_stmda, POST, DEC, false, false);

/// LDMDA 
/// Load multiple words, decrement after
gen_ldm!(arm_ldmda, POST, DEC, false, false);

/// STMDA w
/// Store multiple words, decrement after
/// Write back
gen_stm!(arm_stmda_w, POST, DEC, true, false);

/// LDMDA w
/// Load multiple words, decrement after
/// Write back
gen_ldm!(arm_ldmda_w, POST, DEC, true, false);

/// STMDA u
/// Store multiple words, decrement after
/// Use user-mode registers
gen_stm!(arm_stmda_u, POST, DEC, false, true);

/// LDMDA u
/// Load multiple words, decrement after
/// Use user-mode registers
gen_ldm_u!(arm_ldmda_u, POST, DEC, false);

/// STMDA uw
/// Store multiple words, decrement after
/// Use user-mode registers, with write back
gen_stm!(arm_stmda_uw, POST, DEC, true, true);

/// LDMDA uw
/// Load multiple words, decrement after
/// Use user-mode registers, with write back
gen_ldm_u!(arm_ldmda_uw, POST, DEC, true);

/// STMIA 
/// Store multiple words, increment after
gen_stm!(arm_stmia, POST, INC, false, false);

/// LDMIA 
/// Load multiple words, increment after
gen_ldm!(arm_ldmia, POST, INC, false, false);

/// STMIA w
/// Store multiple words, increment after
/// Write back
gen_stm!(arm_stmia_w, POST, INC, true, false);

/// LDMIA w
/// Load multiple words, increment after
/// Write back
gen_ldm!(arm_ldmia_w, POST, INC, true, false);

/// STMIA u
/// Store multiple words, increment after
/// Use user-mode registers
gen_stm!(arm_stmia_u, POST, INC, false, true);

/// LDMIA u
/// Load multiple words, increment after
/// Use user-mode registers
gen_ldm_u!(arm_ldmia_u, POST, INC, false);

/// STMIA uw
/// Store multiple words, increment after
/// Use user-mode registers, with write back
gen_stm!(arm_stmia_uw, POST, INC, true, true);

/// LDMIA uw
/// Load multiple words, increment after
/// Use user-mode registers, with write back
gen_ldm_u!(arm_ldmia_uw, POST, INC, true);

/// STMDB 
/// Store multiple words, decrement before
gen_stm!(arm_stmdb, PRE, DEC, false, false);

/// LDMDB 
/// Load multiple words, decrement before
gen_ldm!(arm_ldmdb, PRE, DEC, false, false);

/// STMDB w
/// Store multiple words, decrement before
/// Write back
gen_stm!(arm_stmdb_w, PRE, DEC, true, false);

/// LDMDB w
/// Load multiple words, decrement before
/// Write back
gen_ldm!(arm_ldmdb_w, PRE, DEC, true, false);

/// STMDB u
/// Store multiple words, decrement before
/// Use user-mode registers
gen_stm!(arm_stmdb_u, PRE, DEC, false, true);

/// LDMDB u
/// Load multiple words, decrement before
/// Use user-mode registers
gen_ldm_u!(arm_ldmdb_u, PRE, DEC, false);

/// STMDB uw
/// Store multiple words, decrement before
/// Use user-mode registers, with write back
gen_stm!(arm_stmdb_uw, PRE, DEC, true, true);

/// LDMDB uw
/// Load multiple words, decrement before
/// Use user-mode registers, with write back
gen_ldm_u!(arm_ldmdb_uw, PRE, DEC, true);

/// STMIB 
/// Store multiple words, increment before
gen_stm!(arm_stmib, PRE, INC, false, false);

/// LDMIB 
/// Load multiple words, increment before
gen_ldm!(arm_ldmib, PRE, INC, false, false);

/// STMIB w
/// Store multiple words, increment before
/// Write back
gen_stm!(arm_stmib_w, PRE, INC, true, false);

/// LDMIB w
/// Load multiple words, increment before
/// Write back
gen_ldm!(arm_ldmib_w, PRE, INC, true, false);

/// STMIB u
/// Store multiple words, increment before
/// Use user-mode registers
gen_stm!(arm_stmib_u, PRE, INC, false, true);

/// LDMIB u
/// Load multiple words, increment before
/// Use user-mode registers
gen_ldm_u!(arm_ldmib_u, PRE, INC, false);

/// STMIB uw
/// Store multiple words, increment before
/// Use user-mode registers, with write back
gen_stm!(arm_stmib_uw, PRE, INC, true, true);

/// LDMIB uw
/// Load multiple words, increment before
/// Use user-mode registers, with write back
gen_ldm_u!(arm_ldmib_uw, PRE, INC, true);

/// STC ofm
/// Store coprocessor data to memory
/// Negative offset
pub fn arm_stc_ofm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_ofm");
}

/// LDC ofm
/// Load coprocessor data from memory
/// Negative offset
pub fn arm_ldc_ofm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_ofm");
}

/// STC prm
/// Store coprocessor data to memory
/// Pre-decrement
pub fn arm_stc_prm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_prm");
}

/// LDC prm
/// Load coprocessor data from memory
/// Pre-decrement
pub fn arm_ldc_prm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_prm");
}

/// STC ofp
/// Store coprocessor data to memory
/// Positive offset
pub fn arm_stc_ofp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_ofp");
}

/// LDC ofp
/// Load coprocessor data from memory
/// Positive offset
pub fn arm_ldc_ofp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_ofp");
}

/// STC prp
/// Store coprocessor data to memory
/// Pre-increment
pub fn arm_stc_prp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_prp");
}

/// LDC prp
/// Load coprocessor data from memory
/// Pre-increment
pub fn arm_ldc_prp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_prp");
}

/// STC unm
/// Store coprocessor data to memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_stc_unm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_unm");
}

/// LDC unm
/// Load coprocessor data from memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_ldc_unm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_unm");
}

/// STC ptm
/// Store coprocessor data to memory
/// Post-decrement
pub fn arm_stc_ptm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_ptm");
}

/// LDC ptm
/// Load coprocessor data from memory
/// Post-decrement
pub fn arm_ldc_ptm(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_ptm");
}

/// STC unp
/// Store coprocessor data to memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_stc_unp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_unp");
}

/// LDC unp
/// Load coprocessor data from memory
/// Unindexed, bits 7-0 available for copro use
pub fn arm_ldc_unp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_unp");
}

/// STC ptp
/// Store coprocessor data to memory
/// Post-increment
pub fn arm_stc_ptp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_stc_ptp");
}

/// LDC ptp
/// Load coprocessor data from memory
/// Post-increment
pub fn arm_ldc_ptp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_ldc_ptp");
}

/// CDP 
/// Perform coprocessor data operation
pub fn arm_cdp(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_cdp");
}

/// MCR 
/// Write coprocessor register from ARM register
pub fn arm_mcr(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_mcr");
}

/// MRC 
/// Read coprocessor register to ARM register
pub fn arm_mrc(cpu: &mut ArmCpu, _: u32) {
	cpu.bad_coprocessor_instr("arm_mrc");
}

