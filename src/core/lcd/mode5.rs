use super::*;
use super::super::memory::*;

const FRAME_0_BUFFER: (u32, u32) = (0x06000000, 0x06009FFF);
const FRAME_1_BUFFER: (u32, u32) = (0x0600A000, 0x06013FFF);

pub fn render_mode_5(dispcnt: u16, memory: &mut GbaMemory, line: u16, line_buffer: &mut [GbaPixel]) {
	let transparent = memory.read16(0x05000000);
	let transparent_color = convert_rgb5_to_rgb8(transparent);
	if line < 128 {
		let frame_buffer = if ((dispcnt >> 4) & 1) != 0 {
			memory.get_slice(FRAME_1_BUFFER.0, FRAME_1_BUFFER.1)
		} else {
			memory.get_slice(FRAME_0_BUFFER.0, FRAME_0_BUFFER.1)
		};
		let frame_line_offset = 160 * (line as usize) * 2;
		for col in 0..160 {
			let col_offset = frame_line_offset + col * 2;
			let pixel = convert_rgb5_to_rgb8(frame_buffer.direct_read16(col_offset));
			line_buffer[col] = pixel;
		}

		for col in 160..240 {
			line_buffer[col] = transparent_color;
		}
	} else {
		for col in 0..line_buffer.len() {
			line_buffer[col] = transparent_color;
		}
	}
}