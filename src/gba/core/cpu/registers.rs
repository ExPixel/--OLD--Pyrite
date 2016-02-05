/// This is the default mode.
pub const MODE_USR: u32 = 0b10000;

/// This is intended to be a priveleged user mode for the operating system.
pub const MODE_SYS: u32 = 0b11111;

// This mode is entered when a Fast Interrupt Request is triggered.
// Since all of the hardware interrupts on the GBA generate IRQs, this mode goes unused by default,
// though it would be possible to switch to this mode manually using the "msr" instruction.
// Banked registers: r8_fiq, r9_fiq, r10_fiq, r11_fiq, r12_fiq, r13_fiq, r14_fiq, and SPSR_fiq.
pub const MODE_FIQ: u32 = 0b10001;


// This mode is entered when an Interrupt Request is triggered.
// Any interrupt handler on the GBA will be called in IRQ mode.
// Banked registers: The ARM7tdmi has several sets of banked registers that get swapped in place of normal
// user mode registers when a priveleged mode is entered, to be swapped back out again once the
// mode is exited. In IRQ mode, r13_irq and r14_irq will be swapped in to replace r13 and r14.
// The current CPSR contents gets saved in the SPSR_irq register.
pub const MODE_IRQ: u32 = 0b10010;

// Supervisor mode. Entered when a SWI (software interrupt) call is executed.
// The GBA enters this state when calling the BIOS via SWI instructions.
// Banked registers: r13_svc, r14_svc, SPSR_svc.
pub const MODE_SVC: u32 = 0b10011;

// Abort mode. Entered after data or instruction prefetch abort.
// Banked registers: r13_abt, r14_abt, SPSR_abt.
pub const MODE_ABT: u32 = 0b10111;

// Undefined mode. Entered when an undefined instruction is executed.
// Banked registers: r13_und, r14_und, SPSR_und.
pub const MODE_UND: u32 = 0b11011;

/// Negative Flag
pub const REG_FLAG_N: u32 = 0x10000000;

/// Zero Flag
pub const REG_FLAG_Z: u32 = 0x20000000;

/// Carry Flag
pub const REG_FLAG_C: u32 = 0x40000000;

/// Overflow Flag
pub const REG_FLAG_V: u32 = 0x80000000;

/// IRQ Disable Flag
pub const REG_FLAG_I: u32 = 0x00000040;

/// FIQ Disable Flag
pub const REG_FLAG_F: u32 = 0x00000020;

/// Thumb Mode Flag
pub const REG_FLAG_T: u32 = 0x00000010;

/// Stack Pointer
pub const REG_SP: u32 = 13;

/// Link Register
pub const REG_LR: u32 = 14;

/// Program Counter
pub const REG_PC: u32 = 15;

pub struct ArmRegisters {
	internal_registers: [u32; 31],

	/// Current Program Status Register (CPSR)
	///   Bit   Expl.
	///   31    N - Sign Flag       (0=Not Signed, 1=Signed)               ;
	///   30    Z - Zero Flag       (0=Not Zero, 1=Zero)                   ; Condition
	///   29    C - Carry Flag      (0=Borrow/No Carry, 1=Carry/No Borrow) ; Code Flags
	///   28    V - Overflow Flag   (0=No Overflow, 1=Overflow)            ;
	///   27    Q - Sticky Overflow (1=Sticky Overflow, ARMv5TE and up only)
	///   26-8  Reserved            (For future use) - Do not change manually!
	///   7     I - IRQ disable     (0=Enable, 1=Disable)                     ;
	///   6     F - FIQ disable     (0=Enable, 1=Disable)                     ; Control
	///   5     T - State Bit       (0=ARM, 1=THUMB) - Do not change manually!; Bits
	///   4-0   M4-M0 - Mode Bits   (See below)                               ;
	cpsr: u32,

	spsr: [u32; 5]
}

fn is_priveleged_mode(mode: u32) -> bool {
	mode != MODE_USR
}

impl ArmRegisters {
	pub fn new() -> ArmRegisters {
		ArmRegisters {
			internal_registers: [0u32; 31],
			cpsr: 0,
			spsr: [0u32; 5]
		}
	}

	pub fn get_pc(&self) -> u32 {
		self.internal_registers[15]
	}

	pub fn set_pc(&mut self, value: u32) {
		self.internal_registers[15] = value;
	}

	/// Returns the value of the specified register for the current mode.
	pub fn get(&self, register: u32) -> u32 {
		let reg_index = self.get_register_index(register);
		self.internal_registers[reg_index]
	}

	/// Sets the value of the specified register for the current mode.
	pub fn set(&mut self, register: u32, value: u32) {
		let reg_index = self.get_register_index(register);
		self.internal_registers[reg_index] = value;
	}

	pub fn get_with_mode(&self, mode: u32, register: u32) -> u32 {
		let reg_index = Self::get_register_index_with_mode(mode, register);
		self.internal_registers[reg_index]
	}

	pub fn set_with_mode(&mut self, mode: u32, register: u32, value: u32) {
		let reg_index = Self::get_register_index_with_mode(mode, register);
		self.internal_registers[reg_index] = value;
	}

	fn get_register_index_with_mode(mode: u32, register32: u32) -> usize {
		let register = register32 as usize;
		match mode {
			MODE_USR | MODE_SYS => register,
			MODE_FIQ if register >= 8 && register <= 14  => register + 8,
			MODE_SVC if register == 13 || register == 14 => register + 10,
			MODE_ABT if register == 13 || register == 14 => register + 12,
			MODE_IRQ if register == 13 || register == 14 => register + 14,
			MODE_UND if register == 13 || register == 14 => register + 16,
			_ => register
		}
	}

	/// Changes register numbers to their location
	/// in the internal_registers array.
	/// 
	/// USR: (Same)  
	/// SYS: (Same)  
	/// FIQ: starting at 16 for r8-r14  
	/// SVC: starting at 23 for r13-r14  
	/// ABT: starting at 25 for r13-r14  
	/// IRQ: starting at 27 for r13-r14  
	/// UND: starting at 29 for r13-r14  
	fn get_register_index(&self, register32: u32) -> usize {
		let register = register32 as usize;
		match self.get_mode() {
			MODE_USR | MODE_SYS => register,
			MODE_FIQ if register >= 8 && register <= 14  => register + 8,
			MODE_SVC if register == 13 || register == 14 => register + 10,
			MODE_ABT if register == 13 || register == 14 => register + 12,
			MODE_IRQ if register == 13 || register == 14 => register + 14,
			MODE_UND if register == 13 || register == 14 => register + 16,
			_ => register
		}
	}

	pub fn get_mode(&self) -> u32 {
		self.cpsr & 0x1f
	}

	pub fn set_mode(&mut self, mode: u32) {
		self.cpsr &= !0x1f;
		self.cpsr |= mode;
	}

	/// Writes a value to the cpsr
	/// to only the flag bits
	pub fn set_cpsr_flags(&mut self, value: u32) {
		self.cpsr &= 0x0fffffff;
		self.cpsr |= value & 0xf0000000;
	}

	/// Returns get value of the cpsr.
	pub fn get_cpsr(&self) -> u32 {
		self.cpsr
	}

	/// Writes a value to the cpsr
	pub fn set_cpsr(&mut self, value: u32) {
		self.cpsr = value;
	}

	/// Returns the index of the spsr for the current mode.
	pub fn get_spsr_index(&self) -> usize {
		match self.get_mode() {
			MODE_FIQ => 0,
			MODE_SVC => 1,
			MODE_ABT => 2,
			MODE_IRQ => 3,
			MODE_UND => 4,
			_ => panic!("BAD SPSR INDEX! CURRENT MODE = 0b{:05b}; PC = 0x{:08x}", self.get_mode(), self.get_pc())
		}
	}

	/// Returns the index of the spsr for the current mode.
	pub fn get_spsr_index_safe(&self) -> Option<usize> {
		match self.get_mode() {
			MODE_FIQ => Some(0),
			MODE_SVC => Some(1),
			MODE_ABT => Some(2),
			MODE_IRQ => Some(3),
			MODE_UND => Some(4),
			_ => None
		}
	}

	pub fn set_spsr_flags(&mut self, value: u32) {
		let spsr_index = self.get_spsr_index();
		self.spsr[spsr_index] &= 0x0fffffff;
		self.spsr[spsr_index] |= value & 0xf0000000;
	}

	pub fn get_spsr(&self) -> u32 {
		let spsr_index = self.get_spsr_index();
		self.spsr[spsr_index]
	}

	pub fn get_spsr_safe(&self) -> u32 {
		let spsr_index = self.get_spsr_index_safe();
		match spsr_index {
			Some(idx) => self.spsr[idx],
			None => 0
		}
	}

	pub fn set_spsr(&mut self, value: u32) {
		let spsr_index = self.get_spsr_index();
		self.spsr[spsr_index] = value;
	}

	/// Only writes to flag bits in unpriveldged modes.
	pub fn set_spsr_safe(&mut self, value: u32) {
		if is_priveleged_mode(self.get_mode()) {
			self.set_spsr_flags(value);
		} else {
			self.set_spsr(value);
		}
	}

	/// Only writes to flag bits in unpriveldged modes.
	pub fn set_cpsr_safe(&mut self, value: u32) {
		if is_priveleged_mode(self.get_mode()) {
			self.set_cpsr(value);
		} else {
			self.set_cpsr_flags(value);
		}
	}

	/// Saves the CPSR into the current mode's SPSR.
	pub fn cpsr_to_spsr(&mut self) {
		let cpsr = self.cpsr;
		self.set_spsr(cpsr);
	}

	/// Loads the current mode's SPSR into the CPSR.
	pub fn spsr_to_cpsr(&mut self) {
		let spsr = self.get_spsr();
		self.set_cpsr(spsr);
	}
	// clearly generated flags code:
	/// Sets the n flag
	pub fn setf_n(&mut self) { self.cpsr |= 1 << 31; }

	/// Clears the n flag
	pub fn clearf_n(&mut self) { self.cpsr &= !(1 << 31); }

	/// Sets the n flag to a given boolean value.
	pub fn putf_n(&mut self, v: bool) {
		if v { self.setf_n() }
		else { self.clearf_n() }
	}

	/// Sets the n flag to a given integer value.
	pub fn putfi_n(&mut self, v: u32) {
		self.putf_n(v != 0);
	}

	/// Returns the n flag as an integer value.
	pub fn getfi_n(&self) -> u32 { (self.cpsr >> 31) & 1 }

	/// Returns the n flag as a boolean value.
	pub fn getf_n(&self) -> bool { self.getfi_n() != 0 }

	/// Sets the z flag
	pub fn setf_z(&mut self) { self.cpsr |= 1 << 30; }

	/// Clears the z flag
	pub fn clearf_z(&mut self) { self.cpsr &= !(1 << 30); }

	/// Sets the z flag to a given boolean value.
	pub fn putf_z(&mut self, v: bool) {
		if v { self.setf_z() }
		else { self.clearf_z() }
	}

	/// Sets the z flag to a given integer value.
	pub fn putfi_z(&mut self, v: u32) {
		self.putf_z(v != 0);
	}

	/// Returns the z flag as an integer value.
	pub fn getfi_z(&self) -> u32 { (self.cpsr >> 30) & 1 }

	/// Returns the z flag as a boolean value.
	pub fn getf_z(&self) -> bool { self.getfi_z() != 0 }

	/// Sets the c flag
	pub fn setf_c(&mut self) { self.cpsr |= 1 << 29; }

	/// Clears the c flag
	pub fn clearf_c(&mut self) { self.cpsr &= !(1 << 29); }

	/// Sets the c flag to a given boolean value.
	pub fn putf_c(&mut self, v: bool) {
		if v { self.setf_c() }
		else { self.clearf_c() }
	}

	/// Sets the c flag to a given integer value.
	pub fn putfi_c(&mut self, v: u32) {
		self.putf_c(v != 0);
	}

	/// Returns the c flag as an integer value.
	pub fn getfi_c(&self) -> u32 { (self.cpsr >> 29) & 1 }

	/// Returns the c flag as a boolean value.
	pub fn getf_c(&self) -> bool { self.getfi_c() != 0 }

	/// Sets the v flag
	pub fn setf_v(&mut self) { self.cpsr |= 1 << 28; }

	/// Clears the v flag
	pub fn clearf_v(&mut self) { self.cpsr &= !(1 << 28); }

	/// Sets the v flag to a given boolean value.
	pub fn putf_v(&mut self, v: bool) {
		if v { self.setf_v() }
		else { self.clearf_v() }
	}

	/// Sets the v flag to a given integer value.
	pub fn putfi_v(&mut self, v: u32) {
		self.putf_v(v != 0);
	}

	/// Returns the v flag as an integer value.
	pub fn getfi_v(&self) -> u32 { (self.cpsr >> 28) & 1 }

	/// Returns the v flag as a boolean value.
	pub fn getf_v(&self) -> bool { self.getfi_v() != 0 }

	/// Sets the i flag
	pub fn setf_i(&mut self) { self.cpsr |= 1 << 7; }

	/// Clears the i flag
	pub fn clearf_i(&mut self) { self.cpsr &= !(1 << 7); }

	/// Sets the i flag to a given boolean value.
	pub fn putf_i(&mut self, v: bool) {
		if v { self.setf_i() }
		else { self.clearf_i() }
	}

	/// Sets the i flag to a given integer value.
	pub fn putfi_i(&mut self, v: u32) {
		self.putf_i(v != 0);
	}

	/// Returns the i flag as an integer value.
	pub fn getfi_i(&self) -> u32 { (self.cpsr >> 7) & 1 }

	/// Returns the i flag as a boolean value.
	pub fn getf_i(&self) -> bool { self.getfi_i() != 0 }

	/// Sets the f flag
	pub fn setf_f(&mut self) { self.cpsr |= 1 << 6; }

	/// Clears the f flag
	pub fn clearf_f(&mut self) { self.cpsr &= !(1 << 6); }

	/// Sets the f flag to a given boolean value.
	pub fn putf_f(&mut self, v: bool) {
		if v { self.setf_f() }
		else { self.clearf_f() }
	}

	/// Sets the f flag to a given integer value.
	pub fn putfi_f(&mut self, v: u32) {
		self.putf_f(v != 0);
	}

	/// Returns the f flag as an integer value.
	pub fn getfi_f(&self) -> u32 { (self.cpsr >> 6) & 1 }

	/// Returns the f flag as a boolean value.
	pub fn getf_f(&self) -> bool { self.getfi_f() != 0 }

	/// Sets the t flag
	pub fn setf_t(&mut self) { self.cpsr |= 1 << 5; }

	/// Clears the t flag
	pub fn clearf_t(&mut self) { self.cpsr &= !(1 << 5); }

	/// Sets the t flag to a given boolean value.
	pub fn putf_t(&mut self, v: bool) {
		if v { self.setf_t() }
		else { self.clearf_t() }
	}

	/// Sets the t flag to a given integer value.
	pub fn putfi_t(&mut self, v: u32) {
		self.putf_t(v != 0);
	}

	/// Returns the t flag as an integer value.
	pub fn getfi_t(&self) -> u32 { (self.cpsr >> 5) & 1 }

	/// Returns the t flag as a boolean value.
	pub fn getf_t(&self) -> bool { self.getfi_t() != 0 }
}