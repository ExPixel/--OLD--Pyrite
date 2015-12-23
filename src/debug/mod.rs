pub mod armdis;
pub use ::core::Gba;

use self::armdis::*;

pub fn print_gba_rom_disasm(gba: &mut Gba, thumb_mode: bool) {
	if thumb_mode {
		print_gba_rom_disasm_thumb(gba);
	} else {
		print_gba_rom_disasm_arm(gba);
	}
}

fn print_gba_rom_disasm_arm(gba: &mut Gba) {
	let mut buffer = String::new();
	let mut offset = 0x8000000u32;
	let len = gba.cpu.memory.rom.len() as u32;
	let max = offset + len;

	while offset < max {
		buffer.clear();
		disasm_arm_into(&mut buffer, offset, &mut gba.cpu.memory, DIS_WRITE_ALL);
		println!("{}", buffer);
		offset += 4;
	}
}

fn print_gba_rom_disasm_thumb(gba: &mut Gba) {
	let mut buffer = String::new();
	let mut offset = 0x8000000u32;
	let len = gba.cpu.memory.rom.len() as u32;
	let max = offset + len;

	while offset < max {
		buffer.clear();
		disasm_thumb_into(&mut buffer, offset, &mut gba.cpu.memory, DIS_WRITE_ALL);
		println!("{}", buffer);
		offset += 2;
	}
}