pub mod armdis;
pub mod debugger;

// not ready.
// pub mod armasm;

pub use ::gba::core::memory::GbaMemory;

use self::armdis::*;

pub fn print_gba_rom_disasm(memory: &mut GbaMemory, thumb_mode: bool) {
	if thumb_mode {
		print_gba_rom_disasm_thumb(memory);
	} else {
		print_gba_rom_disasm_arm(memory);
	}
}

fn print_gba_rom_disasm_arm(memory: &mut GbaMemory) {
	let mut buffer = String::new();
	let mut offset = 0x8000000u32;
	let len = memory.rom.len() as u32;
	let max = offset + len;

	while offset < max {
		buffer.clear();
		disasm_arm_into(&mut buffer, offset, memory, DIS_WRITE_ALL);
		println!("{}", buffer);
		offset += 4;
	}
}

fn print_gba_rom_disasm_thumb(memory: &mut GbaMemory) {
	let mut buffer = String::new();
	let mut offset = 0x8000000u32;
	let len = memory.rom.len() as u32;
	let max = offset + len;

	while offset < max {
		buffer.clear();
		disasm_thumb_into(&mut buffer, offset, memory, DIS_WRITE_ALL);
		println!("{}", buffer);
		offset += 2;
	}
}