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


	// #TODO could probably make thumb instructions better by giving them
	// a rset_lo functions so that they don't have to check PC because
	// it's impossible for them to change it.
	pub fn rset(&mut self, register: u32, value: u32) {
		if DEBUG_TRACK_REGISTERS { // #TODO REMOVE DEBUG CODE.
			let tbit = self.registers.getfi_t();
			debug_track_register_change(register, self.get_exec_address() | tbit, self.registers.get(register), value);
		}

		if !self.branched && register == 15 { self.branched = true }
		return self.registers.set(register, value);
	}

	pub fn get_pc(&self) -> u32 {
		self.registers.get_pc()
	}

	pub fn set_pc(&mut self, value: u32) {
		self.branched = true;
		self.registers.set_pc(value);
	}

	pub fn thumb_mode(&self) -> bool {
		self.registers.getf_t()
	}

	/// Reads an unsigned 8 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte)
	pub fn mread8_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it. (maybe)
		self.memory.read8(address) as u32
	}

	/// Reads a signed 8 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte)
	pub fn mread8_signed_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.(maybe)
		((self.memory.read8(address) as i8) as i32) as u32
	}

	/// Reads an unsigned 16 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte)
	pub fn mread16_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.(maybe)
		let data = self.memory.read16(address & 0xFFFFFFFE); // make sure what we retrieve is word aligned.
		let offset = (address & 0x1) * 8; // offset from the word boundary in bits.
		((data << (16 - offset)) | (data >> offset)) as u32 // rotate right by offset.
		// self.memory.read16(address) as u32
	}

	/// Reads a signed 16 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte)
	pub fn mread16_signed_al(&self, address: u32) -> u32 {
		// # TODO alignment shouldn't be necessary on these so I should remove it.(maybe)
		((self.memory.read16(address) as i16) as i32) as u32
	}

	/// Reads an unsigned 32 bit value from memory and makes sure that
	/// all of the correct data ends up on the correct data bus (basically byte)
	pub fn mread32_al(&self, address: u32) -> u32 {
		let data = self.memory.read32(address & 0xFFFFFFFC); // make sure what we retrieve is word aligned.
		let offset = (address & 0x3) * 8; // offset from the word boundary in bits.
		(data << (32 - offset)) | (data >> offset) // rotate right by offset.
	}

	// pub fn mread8(&self, address: u32) -> u8 {
	// 	self.memory.read8(address)
	// }

	// pub fn mread16(&self, address: u32) -> u16 {
	// 	self.memory.read16(address)
	// }

	// pub fn mread32(&self, address: u32) -> u32 {
	// 	self.memory.read32(address)
	// }

	pub fn mwrite8(&mut self, address: u32, value: u8) {
		self.memory.write8(address, value);
	}

	pub fn mwrite16(&mut self, address: u32, value: u16) {
		self.memory.write16(address, value);
	}

	pub fn mwrite32(&mut self, address: u32, value: u32) {
		self.memory.write32(address, value);
	}

	fn arm_tick(&mut self) {
		let exec_addr = self.get_exec_address(); // #TODO remove this debug code.

		if self.arm_pipeline.ready() {
			let decoded = self.arm_pipeline.decoded;
			let condition = (decoded >> 28) & 0xf;
			if self.check_condition(condition) {
				before_execution(exec_addr, self); // #TODO remove this debug code.
				execute_arm(self, decoded);
				after_execution(exec_addr, self); // #TODO remove this debug code.
			}
		}

		if self.branched {
			self.align_pc();
			self.arm_pipeline.flush();
			self.branched = false;
			self.fill_pipeline();

			// #TODO remove debug code.
			if DEBUG_TRACK_BRANCHES {
				let __self_pc = self.get_pc();
				let __self_thumb = self.thumb_mode();
				debug_push_branch(self, exec_addr, false, __self_pc, __self_thumb);
			}
		} else {
			let pc = self.get_pc();
			let next = self.memory.read32(pc);
			self.registers.set(REG_PC, pc + 4);
			self.arm_pipeline.next(next);
		}
	}

	fn thumb_tick(&mut self) {
		let exec_addr = self.get_exec_address(); // #TODO remove this debug code.

		if self.thumb_pipeline.ready() {
			let decoded = self.thumb_pipeline.decoded as u32;
			before_execution(exec_addr, self); // #TODO remove this debug code.
			execute_thumb(self, decoded);
			after_execution(exec_addr, self); // #TODO remove this debug code.
		}

		if self.branched {
			self.align_pc(); // half-word aligning the program counter for THUMB mode.
			self.thumb_pipeline.flush();
			self.branched = false;
			self.fill_pipeline();

			// #TODO remove debug code.
			if DEBUG_TRACK_BRANCHES {
				let __self_pc = self.get_pc();
				let __self_thumb = self.thumb_mode();
				debug_push_branch(self, exec_addr, true, __self_pc, __self_thumb);
			}
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

	/// Returns the number of the software interrupt for the GBA.
	pub fn get_gba_swi(&mut self, instr: u32) -> u32 {
		if self.thumb_mode() {
			let interrupt = instr & 0xff;
			interrupt
		} else {
			let interrupt = instr & 0xffffff;
			interrupt >> 16
		}
	}

	/// The software interrupt instruction is used to enter Supervisor mode in a controlled manner. 
	/// The instruction causes the software interrupt trap to be taken, which effects the mode change. 
	/// The PC is then forced to a fixed value (0x08) and the CPSR is saved in SPSR_svc. 
	/// If the SWI vector address is suitably protected (by external memory management hardware) 
	/// from modification by the user, a fully protected operating system may be constructed.

	/// The PC is saved in R14_svc upon entering the software interrupt trap, 
	/// with the PC adjusted to point to the word after the SWI instruction. 
	/// MOVS PC,R14_svc will return to the calling program and restore the CPSR.
	/// 
	/// Note that the link mechanism is not re-entrant, 
	/// so if the supervisor code wishes to use software interrupts within itself it 
	/// must first save a copy of the return address and SPSR.
	fn handle_arm_swi(&mut self) {
		self.clock.code_access32_nonseq(SWI_VECTOR);
		self.clock.code_access32_seq(SWI_VECTOR + 4);
		let cpsr = self.registers.get_cpsr(); // We don't want the new mode in there.
		self.registers.set_mode(MODE_SVC);
		self.registers.set_spsr(cpsr);
		let next_pc = self.get_pc() - 4;
		self.rset(REG_LR, next_pc);
		self.rset(REG_PC, SWI_VECTOR); // The tick function will handle flushing the pipeline.
		// self.align_pc();
		// self.arm_pipeline.flush();
		// self.branched = false;
	}

	/// Perform Software Interrupt:
	/// Move the address of the next instruction into LR, move CPSR to SPSR, 
	/// load the SWI vector address (0x8) into the PC. Switch to ARM state and enter SVC mode.
	fn handle_thumb_swi(&mut self) {
		self.clock.code_access32_nonseq(SWI_VECTOR);
		self.clock.code_access32_seq(SWI_VECTOR + 4);
		let cpsr = self.registers.get_cpsr(); // We don't want the new mode in there.
		self.registers.set_mode(MODE_SVC);
		self.registers.set_spsr(cpsr);
		let next_pc = self.get_pc() - 2;
		self.rset(REG_LR, next_pc);
		self.rset(REG_PC, SWI_VECTOR); // The tick function will handle flushing the pipeline.
		self.registers.clearf_t(); // Enters ARM mode.
		// self.align_pc();
		// self.thumb_pipeline.flush();
		// self.branched = false;
	}

	pub fn thumb_swi(&mut self) {
		self.clock_prefetch_thumb();
		self.handle_thumb_swi();
	}

	pub fn arm_swi(&mut self) {
		self.clock_prefetch_arm();
		self.handle_arm_swi();
	}

	pub fn allow_irq_interrupt(&mut self) -> bool {
		!self.registers.getf_i()
	}

	pub fn fill_pipeline(&mut self) {
		if self.thumb_mode() {
			self.fill_thumb_pipeline();
		} else {
			self.fill_arm_pipeline();
		}
	}

	pub fn fill_arm_pipeline(&mut self) {
		if self.arm_pipeline.count == 0 {
			let pc = self.get_pc();
			let mut next = self.memory.read32(pc);
			self.arm_pipeline.next(next);
			next = self.memory.read32(pc + 4);
			self.arm_pipeline.next(next);
			self.registers.set(REG_PC, pc + 8);
		} else if self.arm_pipeline.count == 1 {
			let pc = self.get_pc();
			let next = self.memory.read32(pc);
			self.registers.set(REG_PC, pc + 4);
			self.arm_pipeline.next(next);
		}
	}

	pub fn fill_thumb_pipeline(&mut self) {
		if self.thumb_pipeline.count == 0 {
			let pc = self.get_pc();
			let mut next = self.memory.read16(pc);
			self.thumb_pipeline.next(next);
			next = self.memory.read16(pc + 2);
			self.thumb_pipeline.next(next);
			self.registers.set(REG_PC, pc + 4);
		} else if self.thumb_pipeline.count == 1 {
			let pc = self.get_pc();
			let next = self.memory.read16(pc);
			self.thumb_pipeline.next(next);
			self.registers.set(REG_PC, pc + 2);
		}
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
		// println!("[0x{:08X}] IRQ INTERRUPT: 0x{:04X}", self.get_exec_address(), self.memory.get_reg(ioreg::IF));
		self.clock.code_access32_nonseq(SWI_VECTOR);
		self.clock.code_access32_seq(SWI_VECTOR + 4);
		let cpsr = self.registers.get_cpsr(); // We don't want the new mode in the spsr.
		self.registers.set_mode(MODE_IRQ);
		self.registers.set_spsr(cpsr);
		self.registers.setf_i(); // Disables future IRQ interrupts.

		// The call below is now uncessary now that I am using get_exec_address()
		// but I'm keeping the code for now.
		// self.fill_pipeline(); // ensure the pipeline is filled for this.

		// the value of LR must be set so that
		// subs PC, LR, #4 can return to the next instruction.
		// this usually happens at 0x13c in the BIOS.
		// the exec_addr is the address of the next instruction to be executed.
		// ^ unless we're in the middle of executing the instruction already. The it's the current instruction.
		let next_pc = self.get_exec_address() + 4;

		self.rset(REG_LR, next_pc);
		self.rset(REG_PC, HWI_VECTOR);
		self.registers.clearf_t(); // Enters ARM mode.
		self.thumb_pipeline.flush();
		self.arm_pipeline.flush();
		self.align_pc();
		self.branched = false;
	}

	/// The CPU has hit an undefined instruction.
	pub fn on_undefined(&mut self) {
		self.reg_dump_pretty();
		debug_unwind_branches();
		panic!("picnic -- undefined instruction");
	}

	/// Returns the address of the instruction currently
	/// being executed.
	pub fn get_exec_address(&self) -> u32 {
		if self.thumb_mode() {
			let c = self.thumb_pipeline.count as u32;
			(self.registers.get(15) - (c * 2))
		} else {
			let c = self.arm_pipeline.count as u32;
			(self.registers.get(15) - (c * 4))
		}
	}


	/// Disasssembly of the instruction currently being executed.
	pub fn disasm_exec(&self) -> String {
		if self.thumb_mode() {
			super::super::super::debug::armdis::disasm_thumb(self.get_exec_address(), &self.memory, 0b11111111)
		} else {
			super::super::super::debug::armdis::disasm_arm(self.get_exec_address(), &self.memory, 0b11111111)
		}
	}

	/// Called when the CPU tries to execute a coprocessor instruction.
	pub fn bad_coprocessor_instr(&mut self, instr_name: &'static str) {
		self.reg_dump_pretty();
		panic!("Attempted to call a bad coprocessor data instruction: `{}`", instr_name);
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

// BREAKPOINT OPTIONS:
const DEBUG_STOP: bool = false; // This is actually a breakpoint.
const DEBUG_THUMB: Option<bool> = Some(false);
const DEBUG_ITERATIONS: u32 = 1;
const DEBUG_ADDR: u32 = 0x13c; // 0x00000AB2, 0x000012e8, 0x08005BCA, 0x08009794, 0x08005BC8 0x0000131e

// BRANCH TRACKING OPTIONS:
const DEBUG_TRACK_BRANCHES: bool = false;
const DEBUG_TRACKED_BRANCHES_COUNT: usize = 128;
const DEBUG_BRANCH_TRACK_REGISTERS: bool = false;

// REGISTER TRACKING OPTIONS:
const DEBUG_TRACK_REGISTERS: bool = false;

// MEMORY TABLE OPTIONS:
const DEBUG_PRINT_MEMORY_TABLE: bool = false;
const MEMORY_TABLE_START: u32 = 0x06000510;// 0x03007e40
const MEMORY_TABLE_LENGTH: u32 = 128;

// Holds current debug information:
static mut debug_current_iterations: u32 = 0;
static mut debug_branch_count: usize = 0;
static mut debug_branch_next_idx: usize = 0;
static mut debug_tracked_registers: [(u32, u32, u32); 16] = [(0, 0, 0); 16];
static mut track_branch_registers: [[(u32, u32, u32); 16]; DEBUG_TRACKED_BRANCHES_COUNT] = [[(0, 0, 0); 16]; DEBUG_TRACKED_BRANCHES_COUNT];
static mut debug_tracked_branches: [(u32, bool, u32, bool, u32); DEBUG_TRACKED_BRANCHES_COUNT] = [(0, false, 0, false, 0); DEBUG_TRACKED_BRANCHES_COUNT];
// static mut branch_test: bool = false;

// Debug functions:
fn debug_track_register_change(register: u32, location: u32, current_value: u32, new_value: u32) {
	unsafe {
		debug_tracked_registers[register as usize] = (location, new_value, current_value);
	}
}

fn debug_print_register_changes() {
	println!("========== REGISTER CHANGES ==========");
	unsafe {
		for r in 0..16 {
			let rchange = debug_tracked_registers[r];
			let mut rchange_addr = rchange.0;
			let rchange_addr_mode = if (rchange_addr & 1) == 1 { "T" } else { "A" };
			rchange_addr &= !1;
			println!("[0x{:08X} ({})] r{} = 0x{:08X} (~ previous 0x{:08x})", rchange_addr, rchange_addr_mode, r, rchange.1, rchange.2);
		}
	}
	println!("======== REGISTER CHANGES END ========");
}

fn debug_push_branch(_: &mut ArmCpu, branch_from: u32, branch_from_thumb: bool, branch_to: u32, branch_to_thumb: bool) {
	unsafe {
		if debug_branch_next_idx >= DEBUG_TRACKED_BRANCHES_COUNT {
			debug_branch_next_idx = 0;
		}
		debug_tracked_branches[debug_branch_next_idx] = (branch_from, branch_from_thumb, branch_to, branch_to_thumb, 1);
		if DEBUG_BRANCH_TRACK_REGISTERS {
			track_branch_registers[debug_branch_next_idx] = debug_tracked_registers.clone();
		}
		debug_branch_next_idx += 1;
		if debug_branch_count < DEBUG_TRACKED_BRANCHES_COUNT {
			debug_branch_count += 1;
		}
	}
}

fn debug_get_mode_char(thumb_mode: bool) -> &'static str {
	if thumb_mode { "T" }
	else { "A" }
}

fn debug_unwind_branches() {
	unsafe {
		let mut bcount = debug_branch_count;
		let mut bidx = if debug_branch_next_idx == 0 {
			 DEBUG_TRACKED_BRANCHES_COUNT - 1
		} else {
			debug_branch_next_idx - 1
		};

		println!("========== BRANCHES ==========");

		while bcount > 0 {
			let branch = debug_tracked_branches[bidx];

			println!("0x{:08X} ({}) -> 0x{:08X} ({}) ; {} times", branch.0, debug_get_mode_char(branch.1), branch.2, debug_get_mode_char(branch.3), branch.4);
			if DEBUG_BRANCH_TRACK_REGISTERS {
				for r in 0..16 {
					let rchange = track_branch_registers[bidx][r];
					let mut rchange_addr = rchange.0;
					let rchange_addr_mode = if (rchange_addr & 1) == 1 { "T" } else { "A" };
					rchange_addr &= !1;
					println!("\t[0x{:08X} ({})] r{} = 0x{:08X} (~ previous 0x{:08x})", rchange_addr, rchange_addr_mode, r, rchange.1, rchange.2);
				}
			}

			bcount -= 1;
			if bidx == 0 { bidx = DEBUG_TRACKED_BRANCHES_COUNT - 1; }
			else { bidx -= 1; }
		}
		println!("======== BRANCHES END ========");
	}
}

#[allow(warnings)]
fn before_execution(address: u32, cpu: &mut ArmCpu) {
	if DEBUG_STOP && (DEBUG_THUMB == None || DEBUG_THUMB == Some(cpu.registers.getf_t())) && address == DEBUG_ADDR {
		unsafe { debug_current_iterations += 1; if debug_current_iterations < DEBUG_ITERATIONS { return; }}
		println!("============BEFORE============");
		cpu.reg_dump_pretty();
		if DEBUG_PRINT_MEMORY_TABLE {
			print_memory_table!(cpu.memory, MEMORY_TABLE_START, MEMORY_TABLE_START + MEMORY_TABLE_LENGTH - 1);
		}
		if DEBUG_TRACK_REGISTERS {
			debug_print_register_changes();
		}
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
		if DEBUG_PRINT_MEMORY_TABLE {
			print_memory_table!(cpu.memory, MEMORY_TABLE_START, MEMORY_TABLE_START + MEMORY_TABLE_LENGTH - 1);
		}
		if DEBUG_TRACK_BRANCHES {
			debug_unwind_branches();
		}
		if DEBUG_TRACK_REGISTERS {
			debug_print_register_changes();
		}
		panic!("picnic");
	}
}


