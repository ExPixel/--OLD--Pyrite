use super::*;
use super::super::super::core::memory::*;
use super::obj::*;

const FRAME_0_BUFFER: (u32, u32) = (0x06000000, 0x06009FFF);
const FRAME_1_BUFFER: (u32, u32) = (0x0600A000, 0x06013FFF);

pub fn render_mode_5(dispcnt: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	lines.bg0_enable = false;
	lines.bg1_enable = false;
	lines.bg2_enable = true;
	lines.bg3_enable = false;
	
	if line < 128 {
		let frame_buffer = if ((dispcnt >> 4) & 1) != 0 {
			memory.get_slice(FRAME_1_BUFFER.0, FRAME_1_BUFFER.1)
		} else {
			memory.get_slice(FRAME_0_BUFFER.0, FRAME_0_BUFFER.1)
		};
		let frame_line_offset = 160 * (line as usize) * 2;
		for col in 0..160 {
			let col_offset = frame_line_offset + col * 2;
			let pixel = convert_rgb5_to_rgba8(frame_buffer.direct_read16(col_offset));
			lines.bg2[col] = pixel;
		}

		for col in 160..240 {
			lines.bg2[col] = 0;
		}
	} else {
		for col in 0..240 {
			lines.bg2[col] = 0;
		}
	}

	draw_objs(
		(0x06010000, 0x06017FFF),
		((dispcnt >> 6) & 1) == 1,
		((dispcnt >> 5) & 1) == 1,
		memory,
		line,
		lines);
}