pub const MODE_USR: u32 = 0b10000;
pub const MODE_SYS: u32 = 0b11111;
pub const MODE_FIQ: u32 = 0b10001;
pub const MODE_IRQ: u32 = 0b10010;
pub const MODE_SVC: u32 = 0b10011;
pub const MODE_ABT: u32 = 0b10111;
pub const MODE_UND: u32 = 0b11011;

/// Negative Flag
pub const REG_FLAG_N: u32 = 0x10000000;

/// Zero Flag
pub const REG_FLAG_Z: u32 = 0x20000000;

/// Carry Flag
pub const REG_FLAG_C: u32 = 0x40000000;

/// Overflow Flag
pub const REG_FLAG_V: u32 = 0x80000000;

pub const REG_FLAG_I: u32 = 0x00000040;
pub const REG_FLAG_F: u32 = 0x80000020;
pub const REG_FLAG_T: u32 = 0x80000010;

/// Stack Pointer
pub const REG_SP: u32 = 13;

/// Link Register
pub const REG_LR: u32 = 14;

/// Program Counter
pub const REG_PC: u32 = 15;

pub struct ArmRegisters {
	internal_registers: [u32; 31],
	mode: u32,
	cpsr: u32,
	spsr: [u32; 5]
}

impl ArmRegisters {
	pub fn get(&self, register: u32) -> u32 {
		let reg_index = self.get_register_index(register);
		self.internal_registers[reg_index]
	}

	pub fn set(&mut self, register: u32, value: u32) {
		let reg_index = self.get_register_index(register);
		self.internal_registers[reg_index] = value;
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
		match self.mode {
			MODE_USR | MODE_SYS => register,
			MODE_FIQ if register >= 8 && register <= 14  => register + 8,
			MODE_SVC if register == 13 || register == 14 => register + 10,
			MODE_ABT if register == 13 || register == 14 => register + 12,
			MODE_IRQ if register == 13 || register == 14 => register + 14,
			MODE_UND if register == 13 || register == 14 => register + 16,
			_ => register
		}
	}

	/// Writes a value to the cpsr
	/// to only the flag bits
	pub fn write_cpsr_flags(&mut self, value: u32) {
		self.cpsr &= 0x0fffffff;
		self.cpsr |= value & 0xf0000000;
	}

	pub fn read_cpsr(&self) -> u32 {
		self.cpsr
	}

	pub fn write_cpsr(&mut self, value: u32) {
		self.cpsr = value;
	}

	pub fn get_spsr_index(&self) -> usize {
		match self.mode {
			MODE_FIQ => 0,
			MODE_SVC => 1,
			MODE_ABT => 2,
			MODE_IRQ => 3,
			MODE_UND => 4,
			_ => panic!("BAD SPSR INDEX! CURRENT MODE: {}", self.mode)
		}
	}

	pub fn write_spsr_flags(&mut self, value: u32) {
		let spsr_index = self.get_spsr_index();
		self.spsr[spsr_index] &= 0x0fffffff;
		self.spsr[spsr_index] |= value & 0xf0000000;
	}

	pub fn read_spsr(&self) -> u32 {
		let spsr_index = self.get_spsr_index();
		self.spsr[spsr_index]
	}

	pub fn write_spsr(&mut self, value: u32) {
		let spsr_index = self.get_spsr_index();
		self.spsr[spsr_index] = value;
	}

	/// Saves the CPSR into the current mode's SPSR.
	pub fn save_cpsr(&mut self) {
		let cpsr = self.cpsr;
		self.write_spsr(cpsr);
	}

	/// Loads the current mode's SPSR into the CPSR.
	pub fn load_cpsr(&mut self) {
		let spsr = self.read_spsr();
		self.write_cpsr(spsr);
	}

	pub fn get_flag(&self, flag: u32) -> bool {
		(self.cpsr & flag) != 0
	}

	pub fn set_flag(&mut self, flag: u32) {
		self.cpsr |= flag;
	}

	pub fn clear_flag(&mut self, flag: u32) {
		self.cpsr &= !flag;
	}

	pub fn put_flag(&mut self, flag: u32, value: bool) {
		if value { self.set_flag(flag); }
		else { self.clear_flag(flag); }
	}
}