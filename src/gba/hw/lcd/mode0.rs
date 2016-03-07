use super::*;
use super::super::super::core::memory::*;
use super::tiles::*;
use super::obj::*;

const TILE_ADDR: (u32, u32) = (0x06000000, 0x0600FFFF);

pub fn render_mode_0(dispcnt: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	// I use this in both render_mode0 and render_mode1, maybe I could just make a common function for them or something?
	// I would only need to pass in dispcnt and line.
	let mut try_render_text_bg = |enable: u16, bgcnt: ioreg::IORegister16, bghofs: ioreg::IORegister16, bgvofs: ioreg::IORegister16, 
								bg_line: &mut GbaBGLine| {
		if ((dispcnt >> enable) & 1) != 0 {
			draw_tiles_text_mode(
				memory.get_reg(bgcnt), 
				memory.get_reg(bghofs) & 0x1ff,
				memory.get_reg(bgvofs) & 0x1ff,
				memory,
				line,
				bg_line);
		}
	};

	try_render_text_bg(0x8, ioreg::BG0CNT, ioreg::BG0HOFS, ioreg::BG0VOFS, &mut lines.bg0);
	try_render_text_bg(0x9, ioreg::BG1CNT, ioreg::BG1HOFS, ioreg::BG1VOFS, &mut lines.bg1);
	try_render_text_bg(0xA, ioreg::BG2CNT, ioreg::BG2HOFS, ioreg::BG2VOFS, &mut lines.bg2);
	try_render_text_bg(0xB, ioreg::BG3CNT, ioreg::BG3HOFS, ioreg::BG3VOFS, &mut lines.bg3);

	draw_objs(
		(0x06010000, 0x06017FFF), 
		((dispcnt >> 6) & 1) == 1, 
		((dispcnt >> 5) & 1) == 1, 
		memory, 
		line, 
		lines);
}
