use super::*;
use super::super::super::core::memory::*;
use super::tiles::*;

const TILE_ADDR: (u32, u32) = (0x06000000, 0x0600FFFF);

pub fn render_mode_0(dispcnt: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let mut try_render_bg = |enable: u16, bgcnt: ioreg::IORegister16, bghofs: ioreg::IORegister16, bgvofs: ioreg::IORegister16| {
		if ((dispcnt >> enable) & 1) != 0 {
			draw_tiles_text_mode(
				memory.get_reg(bgcnt), 
				memory.get_reg(bghofs) & 0x1ff,
				memory.get_reg(bgvofs) & 0x1ff,
				memory,
				line,
				&mut lines.bg0);
		}
	};

	try_render_bg(0x8, ioreg::BG0CNT, ioreg::BG0HOFS, ioreg::BG0VOFS);
	try_render_bg(0x9, ioreg::BG1CNT, ioreg::BG1HOFS, ioreg::BG1VOFS);
	try_render_bg(0xA, ioreg::BG2CNT, ioreg::BG2HOFS, ioreg::BG2VOFS);
	try_render_bg(0xB, ioreg::BG3CNT, ioreg::BG3HOFS, ioreg::BG3VOFS);
}