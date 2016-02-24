use super::*;
use super::super::super::core::memory::*;

// This is here temporarily so that I don't lose my mind.
macro_rules! kbytes {
	($n: expr) => ($n * 1024)
}

// BG Mode 0,1,2 (Tile/Map based Modes)  
//   06000000-0600FFFF  64 KBytes shared for BG Map and Tiles  
//   06010000-06017FFF  32 KBytes OBJ Tiles  
// The shared 64K area can be split into BG Map area(s), and BG Tiles area(s),
// the respective addresses for Map and Tile areas are set up by BG0CNT-BG3CNT registers. 
// The Map address may be specified in units of 2K (steps of 800h), the Tile address in units of 16K (steps of 4000h).  
// 
// BG Mode 0,1 (Tile/Map based Text mode)  
// The tiles may have 4bit or 8bit color depth, minimum map size is 32x32 tiles, maximum is 64x64 tiles, up to 1024 tiles can be used per map.  
//   Item        Depth     Required Memory  
//   One Tile    4bit      20h bytes  
//   One Tile    8bit      40h bytes  
//   1024 Tiles  4bit      8000h (32K)  
//   1024 Tiles  8bit      10000h (64K) - excluding some bytes for BG map  
//   BG Map      32x32     800h (2K)  
//   BG Map      64x64     2000h (8K)
//
// In 'Text Modes', the screen size is organized as follows: The screen consists of one or more 256x256 pixel (32x32 tiles) areas.
// When Size=0: only 1 area (SC0), when Size=1 or Size=2: two areas (SC0,SC1 either horizontally or vertically arranged next to each other), 
// when Size=3: four areas (SC0,SC1 in upper row, SC2,SC3 in lower row). 
// Whereas SC0 is defined by the normal BG Map base address (Bit 8-12 of BG#CNT), 
// SC1 uses same address +2K, SC2 address +4K, SC3 address +6K. When the screen is scrolled it'll always wraparound.

// screen sizes in shifts.
const SCREEN_SIZES: [(u32, u32); 4] = [
	(8, 8),
	(9, 8),
	(8, 9),
	(9, 9)
];

pub fn draw_tiles_text_mode(bgcnt: u16, xoffset: u16, yoffset: u16, memory: &GbaMemory, line: u16, bg_line: &mut GbaBGLine) {
	let vram_tile_data = memory.get_slice(0x06000000, 0x0600FFFF);

	let character_base_block = (((bgcnt >> 2) & 0x3) as u32) * kbytes!(16); // (0-3, in units of 16 KBytes) (=BG Tile Data)
	let mosaic = ((bgcnt >> 6) & 0x1) == 1;

	let palette_type = ((bgcnt >> 7) & 0x1) == 1; // 0=16/16 (4bit), 1=256/1 (8bit)
	let tile_copy: fn(&[u8], &[u8], &mut [Pixel], u16, u32, u32) = if palette_type {
		copy_tile_line8bpp
	} else {
		copy_tile_line4bpp
	};
	let palette = memory.get_slice(0x05000000, 0x050001FF);

	let screen_base_block = (((bgcnt >> 8) & 0x1f) as u32) * kbytes!(2); // (0-31, in units of 2 KBytes) (=BG Map Data)

	// #FIXME: I think text mode always wraps so this might be unecessary.
	let display_area_overflow = ((bgcnt >> 13) & 0x1) == 1; // (0=Transparent, 1=Wraparound; BG2CNT/BG3CNT only)

	let screen_size = (bgcnt >> 14) & 0x3;

	let (screen_width, screen_height) = SCREEN_SIZES[screen_size as usize];

	// let character_data_size = if palette_type {} else {};
	let character_data = &vram_tile_data[(character_base_block as usize)..0xFFFF];


	let __sw_mod = (1 << screen_width) - 1; // used for mod (screen_width - 1)
	let __sh_mod = (1 << screen_height) - 1; // used for mod (screen_height - 1)

	// screen_width & screen_height are going to be powers of 2.
	let pixel_y = ((line as u32) + (yoffset as u32)) & __sh_mod;
	let mut column = 0;

	{
		let pixel_x = (xoffset as u32) & __sw_mod;
		let sc = ((pixel_x >> 8) & 1) + (((pixel_y >> 8) & 1) << 1);
		let tile_x = (pixel_x & 255) >> 3;
		let tile_y = (pixel_y & 255) >> 3;
		// 2kbytes per SC
		// 2 bytes per tile (32 tiles per line)
		// 		64 bytes per row
		// 		2 bytes per column
		let map_tile_data_addr = screen_base_block + (sc * kbytes!(2)) + (tile_y << 6) + (tile_x << 1);
		let map_tile_info = vram_tile_data.direct_read16(map_tile_data_addr as usize);
		tile_copy(palette, character_data, &mut bg_line[(column as usize)..((column as usize) + ((8 - (pixel_x & 7)) as usize))],
			map_tile_info, pixel_x & 7, pixel_y & 7);
		column += 8 - (pixel_x & 7);
	}

	// 232 because we don't want to draw the last tile unless it's being shown completely.
	while column < 232 {
		// #TODO handle mosaic
		let pixel_x = (column + (xoffset as u32)) & __sw_mod;
		let sc = ((pixel_x >> 8) & 1) + (((pixel_y >> 8) & 1) << 1);
		let tile_x = (pixel_x & 255) >> 3;
		let tile_y = (pixel_y & 255) >> 3;
		let map_tile_data_addr = screen_base_block + (sc * kbytes!(2)) + (tile_y << 6) + (tile_x << 1);
		let map_tile_info = vram_tile_data.direct_read16(map_tile_data_addr as usize);
		tile_copy(palette, character_data, &mut bg_line[(column as usize)..((column as usize) + 8)],
			map_tile_info, 0, pixel_y & 7);
		column += 8;
	}

	{
		let pixel_x = (column + (xoffset as u32)) & __sw_mod;
		let sc = ((pixel_x >> 8) & 1) + (((pixel_y >> 8) & 1) << 1);
		let tile_x = (pixel_x & 255) >> 3;
		let tile_y = (pixel_y & 255) >> 3;
		let map_tile_data_addr = screen_base_block + (sc * kbytes!(2)) + (tile_y << 6) + (tile_x << 1);
		let map_tile_info = vram_tile_data.direct_read16(map_tile_data_addr as usize);
		tile_copy(palette, character_data, &mut bg_line[(column as usize)..((column as usize) + ((240 - column) as usize))],
			map_tile_info, 0, pixel_y & 7);
	}
}

pub fn copy_tile_line4bpp(palette: &[u8], char_data: &[u8], output: &mut [Pixel], tile_info: u16, tx: u32, ty: u32) {
	let mut tx = tx;
	let mut ty = ty;

	let tile_number = tile_info & 0x3ff;

	// #TODO implement these
	let horizontal_flip = ((tile_info >> 10) & 1) == 1;
	let vertical_flip = ((tile_info >> 11) & 1) == 1;
	let palette_number = (tile_info >> 12) & 0xf;

	let left_dot_shift;
	let right_dot_shift;
	let offset_inc;

	if horizontal_flip {
		left_dot_shift = 4;
		right_dot_shift = 0;
		offset_inc = -1isize as usize;
		tx = 7 - tx;
	} else {
		left_dot_shift = 0;
		right_dot_shift = 4;
		offset_inc = 1usize;
	}

	if vertical_flip {
		ty = 7 - ty;
	}

	// 32 bytes per tile
	// 4 bytes per row
	// each byte is 2 columns
	let mut offset = (((tile_number as u32) << 5) + (ty << 2) + (tx >> 1)) as usize;

	let mut pindex = 0;
	
	while pindex < output.len() {
		let two_dots = char_data[offset];

		// #TODO optimize by turning the dot rendering into a function
		// and moving this if condition out of the loop and just drawing the first
		// dot solo if it's not aligned.
		if pindex != 0 || ((tx & 1) == 0) {
			// left pixel
			let left_dot = (two_dots >> left_dot_shift) & 0xf;
			if (left_dot & 15) == 0 {
				// If the color number is a multiple of 16 or 0, 
				// that means that it is color 0 of its palette, making it transparent.
				output[pindex] = (0, 0, 0, 0);
			} else {
				// 32 bytes per palette
				// 2 bytes per color entry
				output[pindex] = convert_rgb5_to_rgba8(palette.direct_read16(((palette_number << 5) + ((left_dot << 1) as u16)) as usize));
			}
			pindex += 1;
		}

		if pindex >= output.len() { break; }

		// right pixel
		let right_dot = (two_dots >> right_dot_shift) & 0xf;
		if (right_dot & 15) == 0 {
			// If the color number is a multiple of 16 or 0, 
			// that means that it is color 0 of its palette, making it transparent.
			output[pindex] = (0, 0, 0, 0);
		} else {
			// 32 bytes per palette
			// 2 bytes per color entry
			output[pindex] = convert_rgb5_to_rgba8(palette.direct_read16(((palette_number << 5) + ((right_dot << 1) as u16)) as usize));
		}
		pindex += 1;

		offset += offset_inc;
	}
}

pub fn copy_tile_line8bpp(palette: &[u8], char_data: &[u8], output: &mut [Pixel], tile_info: u16, tx: u32, ty: u32) {
	let mut tx = tx;
	let mut ty = ty;

	let tile_number = tile_info & 0x3ff;

	// #TODO implement these
	let horizontal_flip = ((tile_info >> 10) & 1) == 1;
	let vertical_flip = ((tile_info >> 11) & 1) == 1;

	let offset_inc;

	if horizontal_flip {
		offset_inc = -1isize as usize;
		tx = 7 - tx;
	} else {
		offset_inc = 1usize;
	}

	if vertical_flip {
		ty = 7 - ty;
	}

	// 64 bytes per tile
	// 8 bytes per row
	// 1 byte per column
	let mut offset = (((tile_number as u32) << 6) + (ty << 3) + tx) as usize;

	let mut max;
	// we don't want an out of bounds so we do this calculation here:
	if offset >= char_data.len() {
		return // no point, then.
	} else {
		max = char_data.len() - offset;
	};
	max = if max < output.len() { max } else { output.len() };

	for pindex in 0..max {
		let dot = char_data[offset];

		// 0 is transparent.
		if dot == 0 {
			output[pindex] = (0, 0, 0, 0);
		} else {
			output[pindex] = convert_rgb5_to_rgba8(palette.direct_read16((dot as usize) << 1));
		}

		offset += offset_inc;
	}
}

