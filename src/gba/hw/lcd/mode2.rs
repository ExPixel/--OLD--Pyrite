use super::*;
use super::super::super::core::memory::*;
use super::tiles::*;
use super::obj::*;

pub fn render_mode_2(dispcnt: u16, memory: &mut GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
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

	if ((dispcnt >> 11) & 1) != 0 {
		let params = BGRotScaleParams {
			ref_x_reg: ioreg::BG3X,
			ref_y_reg: ioreg::BG3Y,
			dx_reg: ioreg::BG3PA,
			dy_reg: ioreg::BG3PC,
			dmx_reg: ioreg::BG3PB,
			dmy_reg: ioreg::BG3PD
		};
		draw_tiles_rs_mode(memory.get_reg(ioreg::BG3CNT), params, memory, line, &mut lines.bg3);
	}
	
	draw_objs(
		(0x06010000, 0x06017FFF), 
		((dispcnt >> 6) & 1) == 1, 
		((dispcnt >> 5) & 1) == 1, 
		memory, 
		line, 
		lines);
}