use super::*;
use super::super::memory::*;

/// BG Mode 3 (Bitmap based Mode for still images)
///  06000000-06013FFF  80 KBytes Frame 0 buffer (only 75K actually used)
///  06014000-06017FFF  16 KBytes OBJ Tiles
/// 
/// BG Mode 3 - 240x160 pixels, 32768 colors
/// Two bytes are associated to each pixel, directly defining one of the 32768 colors 
/// (without using palette data, and thus not supporting a 'transparent' BG color).
///   Bit   Expl.
///   0-4   Red Intensity   (0-31)
///   5-9   Green Intensity (0-31)
///   10-14 Blue Intensity  (0-31)
///   15    Not used in GBA Mode (in NDS Mode: Alpha=0=Transparent, Alpha=1=Normal)
/// The first 480 bytes define the topmost line, the next 480 the next line, and so on. 
/// The background occupies 75 KBytes (06000000-06012BFF), most of the 80 Kbytes BG area, 
/// not allowing to redraw an invisible second frame in background, 
/// so this mode is mostly recommended for still images only.
pub fn render_mode_3(dispcnt: u16, memory: &mut GbaMemory, line: u16, line_buffer: &mut [GbaPixel]) {
	let vram = memory.get_region(MEM_VRAM);
	let frame_line_offset = 480 * line as usize;
	for col in 0..240 {
		let col_offset = frame_line_offset + col * 2;
		let pixel = convert_rgb5_to_rgb8(vram.direct_read16(col_offset));
		line_buffer[col] = pixel;
	}
}