use super::*;
use super::super::super::core::memory::*;

const FRAME_0_BUFFER: (u32, u32) = (0x06000000, 0x06009FFF);
const FRAME_1_BUFFER: (u32, u32) = (0x0600A000, 0x06013FFF);

/// BG Mode 4,5 (Bitmap based Modes)
///   06000000-06009FFF  40 KBytes Frame 0 buffer (only 37.5K used in Mode 4)
///   0600A000-06013FFF  40 KBytes Frame 1 buffer (only 37.5K used in Mode 4)
///   06014000-06017FFF  16 KBytes OBJ Tiles
///
/// One byte is associated to each pixel, selecting one of the 256 palette entries. 
/// Color 0 (backdrop) is transparent, and OBJs may be displayed behind the bitmap.
/// The first 240 bytes define the topmost line, the next 240 the next line, and so on. 
/// The background occupies 37.5 KBytes, allowing two frames to be used 
/// (06000000-060095FF for Frame 0, and 0600A000-060135FF for Frame 1).
pub fn render_mode_4(dispcnt: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let palette = memory.get_slice(0x05000000, 0x050001FF);
	let frame_buffer = if ((dispcnt >> 4) & 1) != 0 {
		memory.get_slice(FRAME_1_BUFFER.0, FRAME_1_BUFFER.1)
	} else {
		memory.get_slice(FRAME_0_BUFFER.0, FRAME_0_BUFFER.1)
	};
	let frame_line_offset = 240 * (line as usize);
	for col in 0..240 {
		let col_offset = frame_line_offset + col;
		// #TODO handle transparency
		let pal_ref = frame_buffer.direct_read8(col_offset);
		let pixel = convert_rgb5_to_rgba8(palette.direct_read16((pal_ref as usize) * 2));
		lines.bg2[col] = pixel;
	}
}