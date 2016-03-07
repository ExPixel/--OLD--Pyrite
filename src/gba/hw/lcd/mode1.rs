use super::*;
use super::super::super::core::memory::*;
use super::tiles::*;
use super::obj::*;

pub fn render_mode_1(dispcnt: u16, memory: &mut GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	// I use this in both render_mode0 and render_mode1, maybe I could just make a common function for them or something?
	// I would only need to pass in dispcnt and line.
	let mut try_render_text_bg = |memory: &GbaMemory, enable: u16, bgcnt: ioreg::IORegister16, bghofs: ioreg::IORegister16, bgvofs: ioreg::IORegister16, 
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

	try_render_text_bg(memory, 0x8, ioreg::BG0CNT, ioreg::BG0HOFS, ioreg::BG0VOFS, &mut lines.bg0);
	try_render_text_bg(memory, 0x9, ioreg::BG1CNT, ioreg::BG1HOFS, ioreg::BG1VOFS, &mut lines.bg1);

	if ((dispcnt >> 10) & 1) != 0 {
		let params = BGRotScaleParams {
			ref_x_reg: ioreg::BG2X,
			ref_y_reg: ioreg::BG2Y,
			dx_reg: ioreg::BG2PA,
			dy_reg: ioreg::BG2PC,
			dmx_reg: ioreg::BG2PB,
			dmy_reg: ioreg::BG2PD
		};
		draw_tiles_rs_mode(memory.get_reg(ioreg::BG2CNT), params, memory, line, &mut lines.bg2);
	}
	
	draw_objs(
		(0x06010000, 0x06017FFF), 
		((dispcnt >> 6) & 1) == 1, 
		((dispcnt >> 5) & 1) == 1, 
		memory, 
		line, 
		lines);
}


/*
pub fn draw_tiles_rs_mode(bgcnt: u16, params: BGRotScaleParams, memory: &GbaMemory, line: u16, bg_line: &mut GbaBGLine) {
}
*/