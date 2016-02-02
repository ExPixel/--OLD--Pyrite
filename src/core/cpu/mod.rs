pub mod arm;
pub mod thumb;
pub mod alu;
pub mod clock;
pub mod registers;

use super::memory::*;
use self::registers::*;
use self::arm::execute_arm;
use self::thumb::execute_thumb;
use self::clock::*;

const SWI_VECTOR: u32 = 0x08;
const HWI_VECTOR: u32 = 0x18;

struct Pipeline<T : Copy> {
	fetched: T,
	decoded: T,
	count: u8
}

impl<T : Copy> Pipeline<T> {
	pub fn new(init_to: T) -> Pipeline<T> {
		Pipeline {
			fetched: init_to,
			decoded: init_to,
			count: 0u8
		}
	}

	pub fn flush(&mut self) {
		self.count = 0;
	}

	pub fn next(&mut self, instr: T) {
		self.decoded = self.fetched;
		self.fetched = instr;
		if !self.ready() {
			self.count += 1;
		}
	}

	/// Returns true if this pipeline is ready to execute.
	pub fn ready(&self) -> bool { self.count > 1 }
}


/// GameBoy ARM7TDMI Cpu.
pub struct ArmCpu {
	thumb_pipeline: Pipeline<u16>,
	arm_pipeline: Pipeline<u32>,
	pub registers: ArmRegisters,
	pub memory: GbaMemory,
	pub clock: ArmCpuClock,
	pub branched: bool,
}

impl ArmCpu {
	pub fn new() -> ArmCpu {
		ArmCpu {
			thumb_pipeline: Pipeline::new(0u16),
			arm_pipeline: Pipeline::new(0u32),
			registers: ArmRegisters::new(),
			memory: GbaMemory::new(),
			clock: ArmCpuClock::new(),
			branched: false
		}
	}

	/// Advances the ARM pipeline.
	/// executes, decodes, and then fetches the next instruction.
	pub fn tick(&mut self) {
		if self.thumb_mode() { self.thumb_tick(); }
		else { self.arm_tick(); }
	}

	pub fn rget(&self, register: u32) -> u32 {
		return self.registers.get(register)
	}

	pub fn get_pc(&self) -> u32 {
		self.registers.get_pc()
	}

	pub fn set_pc(&mut self, value: u32) {
		self.branched = true;
		self.registers.set_pc(value);
	}

	// #TODO could probably make thumb instructions better by giving them
	// a rset_lo functions so that they don't have to check PC because
	// it's impossible for them to change it.
	pub fn rset(&mut self, register: u32, value: u32) {
		if !self.branched && register == 15 { self.branched = true }
		return self.registers.set(register, value);
	}

	pub fn thumb_mode(&self) -> bool {
		self.registers.getf_t()
	}

	/// Reads an unsigned 8 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte)
	pub fn mread8_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.
		self.memory.read8(address) as u32
	}

	/// Reads a signed 8 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte
	pub fn mread8_signed_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.
		((self.memory.read8(address) as i8) as i32) as u32
	}

	/// Reads an unsigned 16 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte
	pub fn mread16_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.
		self.memory.read16(address) as u32
	}

	/// Reads a signed 16 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte
	pub fn mread16_signed_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.
		((self.memory.read16(address) as i16) as i32) as u32
	}

	/// Reads an unsigned 32 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte
	pub fn mread32_al(&self, address: u32) -> u32 {
		let data = self.memory.read32(address & 0xFFFFFFFC); // make sure what we retrieve is word aligned.
		let offset = (address & 0x3) * 8; // offset from the word boundary in bits.
		(data << (32 - offset)) | (data >> offset) // rotate right by offset.
	}

	pub fn mread8(&self, address: u32) -> u8 {
		self.memory.read8(address)
	}

	pub fn mread16(&self, address: u32) -> u16 {
		self.memory.read16(address)
	}

	pub fn mread32(&self, address: u32) -> u32 {
		self.memory.read32(address)
	}

	pub fn mwrite8(&mut self, address: u32, value: u8) {
		self.memory.write8(address, value);
	}

	pub fn mwrite16(&mut self, address: u32, value: u16) {
		self.memory.write16(address, value);
	}

	pub fn mwrite32(&mut self, address: u32, value: u32) {
		if address & 0b11 != 0 {

		}
		self.memory.write32(address, value);
	}

	fn arm_tick(&mut self) {
		if self.arm_pipeline.ready() {
			let decoded = self.arm_pipeline.decoded;
			let condition = (decoded >> 28) & 0xf;
			if self.check_condition(condition) {
				let exec_addr = self.get_exec_address(); // #TODO remove this debug code.
				before_execution(exec_addr, self); // #TODO remove this debug code.
				execute_arm(self, decoded);
				after_execution(exec_addr, self); // #TODO remove this debug code.
			}
		}

		if self.branched {
			self.align_pc();
			self.arm_pipeline.flush();
			self.branched = false;
		} else {
			let pc = self.get_pc();
			let next = self.memory.read32(pc);
			self.registers.set(REG_PC, pc + 4);
			self.arm_pipeline.next(next);
		}
	}

	fn thumb_tick(&mut self) {
		if self.thumb_pipeline.ready() {
			let decoded = self.thumb_pipeline.decoded as u32;
			let exec_addr = self.get_exec_address(); // #TODO remove this debug code.
			before_execution(exec_addr, self); // #TODO remove this debug code.
			execute_thumb(self, decoded);
			after_execution(exec_addr, self); // #TODO remove this debug code.
		}

		if self.branched {
			self.align_pc(); // half-word aligning the program counter for THUMB mode.
			self.thumb_pipeline.flush();
			self.branched = false;
		} else {
			let pc = self.get_pc();
			let next = self.memory.read16(pc);
			self.registers.set(REG_PC, pc + 2);
			self.thumb_pipeline.next(next);
		}
	}

	pub fn align_pc(&mut self) {
		let pc = self.get_pc();
		if self.thumb_mode() {
			self.set_pc(pc & 0xFFFFFFFE);
		} else {
			self.set_pc(pc & 0xFFFFFFFC);
		}
	}

	pub fn clock_prefetch_arm(&mut self) {
		let pc = self.get_pc();
		self.clock.code_access32_seq(pc);
	}

	pub fn clock_prefetch_thumb(&mut self) {
		let pc = self.get_pc();
		self.clock.code_access16_seq(pc);
	}

	pub fn clock_branched_arm(&mut self) {
		let pc = self.get_pc();
		self.clock.code_access32_nonseq(pc);
		self.clock.code_access32_seq(pc + 4);
	}


	pub fn clock_branched_thumb(&mut self) {
		let pc = self.get_pc();
		self.clock.code_access16_nonseq(pc);
		self.clock.code_access16_seq(pc + 2);
	}

	/// ARM Condition Field {cond}
	/// The opcode {cond} suffixes can be used for conditionally executed code based on the C,N,Z,V flags in CPSR register. For example: BEQ = Branch if Equal, MOVMI = Move if Signed.
	/// In ARM mode, {cond} can be used with all opcodes (except for a few newer ARMv5 instructions: BKPT, PLD, CDP2, LDC2, MCR2, MRC2, STC2, and BLX_imm are nonconditional; however BLX_reg can be conditional).
	/// In THUMB mode, {cond} can be used only for branch opcodes.
	///   Code Suffix Flags         Meaning
	///   0:   EQ     Z=1           equal (zero) (same)
	///   1:   NE     Z=0           not equal (nonzero) (not same)
	///   2:   CS/HS  C=1           unsigned higher or same (carry set)
	///   3:   CC/LO  C=0           unsigned lower (carry cleared)
	///   4:   MI     N=1           negative (minus)
	///   5:   PL     N=0           positive or zero (plus)
	///   6:   VS     V=1           overflow (V set)
	///   7:   VC     V=0           no overflow (V cleared)
	///   8:   HI     C=1 and Z=0   unsigned higher
	///   9:   LS     C=0 or Z=1    unsigned lower or same
	///   A:   GE     N=V           greater or equal
	///   B:   LT     N<>V          less than
	///   C:   GT     Z=0 and N=V   greater than
	///   D:   LE     Z=1 or N<>V   less or equal
	///   E:   AL     -             always (the "AL" suffix can be omitted)
	///   F:   NV     -             never (ARMv1,v2 only) (Reserved ARMv3 and up)
	/// Execution Time: If condition=false: 1S cycle. Otherwise: as specified for the respective opcode.
	fn check_condition(&self, condition: u32) -> bool {
		match condition {
			0x0 => self.registers.getf_z(),		// EQ
			0x1 => !self.registers.getf_z(),	// NE
			0x2 => self.registers.getf_c(),		// CS / HS
			0x3 => !self.registers.getf_c(),	// CC / LO
			0x4 => self.registers.getf_n(),		// MI
			0x5 => !self.registers.getf_n(),	// PL
			0x6 => self.registers.getf_v(),		// VS
			0x7 => !self.registers.getf_v(),	// VC
			0x8 => self.registers.getf_c() & !self.registers.getf_z(),	// HI
			0x9 => !self.registers.getf_c() || self.registers.getf_z(),	// LS
			0xA => self.registers.getf_n() == self.registers.getf_v(),	// GE
			0xB => self.registers.getf_n() != self.registers.getf_v(),	// LT
			0xC => !self.registers.getf_z() && (self.registers.getf_n() == self.registers.getf_v()), // GT
			0xD => self.registers.getf_z() || (self.registers.getf_n() != self.registers.getf_v()), // LE
			0xE => true, // AL
			0xF | _ => false // NV
		}
	}

	/// Returns true if the program counter is at an executable
	/// location.
	pub fn executable(&self) -> bool {
		let ready = if self.thumb_mode() {
			self.thumb_pipeline.ready()
		} else {
			self.arm_pipeline.ready()
		};

		if ready {
			let pc = self.get_exec_address();
			let area = (pc >> 24) & 0xff; // Area of memory we are at.
			match area {
				// BIOS, External Work Ram, Internal Work Ram, ROM0-2
				0x00 | 
				0x02 | 
				0x03 | 
				0x08 | 0x09 | 
				0x0A | 0x0B |
				0x0C | 0x0D  => true,
				_ => false
			}
		} else {
			true
		}
	}

	// #TODO: look at this later
	// this is where I think the BIOS
	// finally jumps to the location of the
	// swi's function.
	// 0x0000016c

	// Perform Software Interrupt:
	// Move the address of the next instruction into LR, move CPSR to SPSR, 
	// load the SWI vector address (0x8) into the PC. Switch to ARM state and enter SVC mode.
	pub fn thumb_swi(&mut self, _: u32) {
		self.clock_prefetch_thumb();
		self.clock.code_access32_nonseq(SWI_VECTOR);
		self.clock.code_access32_seq(SWI_VECTOR + 4);
		self.registers.set_mode(MODE_SVC);
		self.registers.cpsr_to_spsr();
		let next_pc = self.get_pc() - 2;
		self.rset(REG_LR, next_pc);
		self.rset(REG_PC, SWI_VECTOR); // The tick function will handle flushing the pipeline.
		self.registers.clearf_t(); // Enters ARM mode.
	}

	// The software interrupt instruction is used to enter Supervisor mode in a controlled manner. 
	// The instruction causes the software interrupt trap to be taken, which effects the mode change. 
	// The PC is then forced to a fixed value (0x08) and the CPSR is saved in SPSR_svc. 
	// If the SWI vector address is suitably protected (by external memory management hardware) 
	// from modification by the user, a fully protected operating system may be constructed.

	// The PC is saved in R14_svc upon entering the software interrupt trap, 
	// with the PC adjusted to point to the word after the SWI instruction. 
	// MOVS PC,R14_svc will return to the calling program and restore the CPSR.
	// 
	// Note that the link mechanism is not re-entrant, 
	// so if the supervisor code wishes to use software interrupts within itself it 
	// must first save a copy of the return address and SPSR.
	pub fn arm_swi(&mut self, _: u32) {
		self.clock_prefetch_arm();
		self.clock.code_access32_nonseq(SWI_VECTOR);
		self.clock.code_access32_seq(SWI_VECTOR + 4);
		self.registers.set_mode(MODE_SVC);
		self.registers.cpsr_to_spsr();
		let next_pc = self.get_pc() - 4;
		self.rset(REG_LR, next_pc);
		self.rset(REG_PC, SWI_VECTOR); // The tick function will handle flushing the pipeline.
	}
	
	pub fn allow_irq_interrupt(&mut self) -> bool {
		self.registers.getf_i()
	}

	/// The branch part of the hardware interrupt with the state
	/// and status changes.
	///
	/// - When an interrupt occurs, the CPU does the following:
	/// 
	/// 1. Switches state to IRQ mode, bank-swaps the current stack register and 
	///    link register (thus preserving their old values), saves the CPSR in SPSR_irq, 
	///    and sets bit 7 (interrupt disable) in the CPSR. 
	/// 2. Saves the address of the next instruction in LR_irq compensating for Thumb/ARM 
	///    depending on the mode you are in. 
	/// 3. Switches to ARM state, executes code in BIOS at a hardware interrupt vector 
	///    (which you, the programmer, never see)
	pub fn irq_interrupt(&mut self) {
		self.registers.set_mode(MODE_IRQ);
		self.registers.cpsr_to_spsr();
		let next_pc = if self.thumb_mode() {
			self.get_pc() - 2
		} else {
			self.get_pc() - 4
		};
		self.rset(REG_LR, next_pc);
		self.rset(REG_PC, HWI_VECTOR);

		self.reg_dump_pretty();
		panic!("picnic");
	}

	/// The CPU has hit an undefined instruction.
	pub fn on_undefined(&mut self) {
	}

	/// Returns the address of the instruction currently
	/// being executed.
	pub fn get_exec_address(&self) -> u32 {
		if self.thumb_mode() {
			let c = self.thumb_pipeline.count as u32;
			self.registers.get(15) - (c * 2)
		} else {
			let c = self.arm_pipeline.count as u32;
			self.registers.get(15) - (c * 4)
		}
	}


	/// Disasssembly of the instruction currently being executed.
	pub fn disasm_exec(&self) -> String {
		if self.thumb_mode() {
			super::super::debug::armdis::disasm_thumb(self.get_exec_address(), &self.memory, 0b11111111)
		} else {
			super::super::debug::armdis::disasm_arm(self.get_exec_address(), &self.memory, 0b11111111)
		}
	}

	pub fn reg_dump(&self) {
		for r in 0..13 {
			print!("r{} = 0x{:08x}; ", r, self.rget(r));
		}
		print!("sp = 0x{:08x}; ", self.rget(13));
		print!("lr = 0x{:08x}; ", self.rget(14));
		println!("pc = 0x{:08x}", self.get_pc());
	}

	pub fn reg_dump_pretty(&self) {
		println!("executing: {}", self.disasm_exec());
		for r in 0..13 {
			println!("r{} = 0x{:08x}; ", r, self.rget(r));
		}
		println!("sp = 0x{:08x}; ", self.rget(13));
		println!("lr = 0x{:08x}; ", self.rget(14));
		println!("pc = 0x{:08x}", self.get_pc());

		print!("cpsr = 0x{:08x} [ ", self.registers.get_cpsr());
		if self.registers.getf_n() { print!("n "); }
		if self.registers.getf_z() { print!("z "); }
		if self.registers.getf_c() { print!("c "); }
		if self.registers.getf_v() { print!("v "); }
		if self.registers.getf_i() { print!("i "); }
		if self.registers.getf_f() { print!("f "); }
		if self.thumb_mode() { print!("t "); }
		print!("] ");

		match self.registers.get_mode() {
			MODE_USR => println!("[USR]"),
			MODE_SYS => println!("[SYS]"),
			MODE_FIQ => println!("[FIQ]"),
			MODE_IRQ => println!("[IRQ]"),
			MODE_SVC => println!("[SVC]"),
			MODE_ABT => println!("[ABT]"),
			MODE_UND => println!("[UND]"),
			_ => println!("[???]")
		}


		println!("spsr = 0x{:08x}", self.registers.get_spsr_safe());
	}
}

const DEBUG_STOP: bool = false;
const DEBUG_THUMB: Option<bool> = Some(false);
const DEBUG_ITERATIONS: u32 = 0;
const DEBUG_ADDR: u32 = 0x0;
static mut debug_current_iterations: u32 = 0;

#[allow(warnings)]
fn before_execution(address: u32, cpu: &mut ArmCpu) {
	// if address < 0x40000 {
	// 	println!("% {}", cpu.disasm_exec());
	// }
	if DEBUG_STOP && (DEBUG_THUMB == None || DEBUG_THUMB == Some(cpu.registers.getf_t())) && address == DEBUG_ADDR {
		unsafe { debug_current_iterations += 1; if debug_current_iterations < DEBUG_ITERATIONS { return; }}
		println!("============BEFORE============");
		cpu.reg_dump_pretty();
		println!("==============================");
	}
}


#[allow(warnings)]
fn after_execution(address: u32, cpu: &mut ArmCpu) {
	if DEBUG_STOP && (DEBUG_THUMB == None || DEBUG_THUMB == Some(cpu.registers.getf_t())) && address == DEBUG_ADDR {
		unsafe { if debug_current_iterations < DEBUG_ITERATIONS { return; } }
		// println!("==============================");
		println!("=============AFTER============");
		cpu.reg_dump_pretty();
		panic!("picnic");
	}
}


