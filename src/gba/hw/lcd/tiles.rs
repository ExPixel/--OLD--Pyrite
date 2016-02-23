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

const RANDOM_FUCKING_COLORS: [(u8, u8, u8); 272] = [
	(0xEF, 0x53, 0x50), (0xCF, 0xD8, 0xDC), (0x75, 0x75, 0x75), (0x30, 0x4F, 0xFE), (0x00, 0x91, 0xEA),
	(0xFF, 0xB3, 0x00), (0x64, 0xB5, 0xF6), (0x00, 0x69, 0x5C), (0x38, 0x8E, 0x3C), (0x15, 0x65, 0xC0),
	(0x79, 0x86, 0xCB), (0xD8, 0x43, 0x15), (0x1A, 0x23, 0x7E), (0xEA, 0x80, 0xFC), (0xF0, 0xF4, 0xC3),
	(0xB2, 0xEB, 0xF2), (0xE5, 0x73, 0x73), (0xFF, 0x70, 0x43), (0x9E, 0x9E, 0x9E), (0xFF, 0xD7, 0x40),
	(0xD7, 0xCC, 0xC8), (0xAA, 0x00, 0xFF), (0xFF, 0xD6, 0x00), (0x95, 0x75, 0xCD), (0x29, 0xB6, 0xF6),
	(0xAE, 0xD5, 0x81), (0x00, 0xE6, 0x76), (0xF5, 0xF5, 0xF5), (0xFB, 0xE9, 0xE7), (0xB3, 0x9D, 0xDB),
	(0xA5, 0xD6, 0xA7), (0xC0, 0xCA, 0x33), (0x00, 0x83, 0x8F), (0xAD, 0x14, 0x57), (0xE3, 0xF2, 0xFD),
	(0xF9, 0xFB, 0xE7), (0xFF, 0xCC, 0x80), (0xDC, 0xE7, 0x75), (0xEC, 0xEF, 0xF1), (0xF4, 0x8F, 0xB1),
	(0xF0, 0x62, 0x92), (0xFF, 0xEB, 0x3B), (0xE0, 0x40, 0xFB), (0xAF, 0xB4, 0x2B), (0x29, 0x62, 0xFF),
	(0x00, 0xB0, 0xFF), (0xFD, 0xD8, 0x35), (0x45, 0x27, 0xA0), (0xFF, 0x6D, 0x00), (0x8C, 0x9E, 0xFF),
	(0xFF, 0x6E, 0x40), (0xCD, 0xDC, 0x39), (0xEF, 0x6C, 0x00), (0x45, 0x5A, 0x64), (0xFB, 0xC0, 0x2D),
	(0x9C, 0xCC, 0x65), (0x33, 0x69, 0x1E), (0x76, 0xFF, 0x03), (0xD4, 0xE1, 0x57), (0xC2, 0x18, 0x5B),
	(0xFF, 0x80, 0xAB), (0x81, 0xC7, 0x84), (0xFF, 0x57, 0x22), (0x55, 0x8B, 0x2F), (0x81, 0xD4, 0xFA),
	(0xE0, 0xF2, 0xF1), (0x00, 0x96, 0x88), (0xA7, 0xFF, 0xEB), (0x00, 0xC8, 0x53), (0x3F, 0x51, 0xB5),
	(0xEF, 0xEB, 0xE9), (0xFF, 0x52, 0x52), (0xF1, 0xF8, 0xE9), (0xE0, 0xE0, 0xE0), (0xBF, 0x36, 0x0C),
	(0xC5, 0xE1, 0xA5), (0x64, 0xDD, 0x17), (0xB2, 0xFF, 0x59), (0xA1, 0x88, 0x7F), (0xE0, 0xF7, 0xFA),
	(0x19, 0x76, 0xD2), (0xD3, 0x2F, 0x2F), (0xFF, 0xEB, 0x3B), (0x4C, 0xAF, 0x50), (0xE6, 0x51, 0x00),
	(0xEC, 0x40, 0x7A), (0x9F, 0xA8, 0xDA), (0xFF, 0xEC, 0xB3), (0x00, 0xBC, 0xD4), (0xFF, 0xC1, 0x07),
	(0xF3, 0xE5, 0xF5), (0x37, 0x47, 0x4F), (0x00, 0xE5, 0xFF), (0x28, 0x35, 0x93), (0x3F, 0x51, 0xB5),
	(0x00, 0x89, 0x7B), (0xFF, 0xEA, 0x00), (0xE1, 0xF5, 0xFE), (0xE9, 0x1E, 0x63), (0xD5, 0x00, 0x00),
	(0x39, 0x49, 0xAB), (0x80, 0xCB, 0xC4), (0x5C, 0x6B, 0xC0), (0xCE, 0x93, 0xD8), (0x1D, 0xE9, 0xB6),
	(0xC5, 0x11, 0x62), (0x69, 0xF0, 0xAE), (0x00, 0x4D, 0x40), (0x88, 0x0E, 0x4F), (0x79, 0x55, 0x48),
	(0xFF, 0xAB, 0x91), (0x82, 0xB1, 0xFF), (0x18, 0xFF, 0xFF), (0x4A, 0x14, 0x8C), (0xFB, 0x8C, 0x00),
	(0x03, 0xA9, 0xF4), (0xFF, 0xEB, 0xEE), (0xFF, 0x6F, 0x00), (0x4C, 0xAF, 0x50), (0x4E, 0x34, 0x2E),
	(0xE1, 0xBE, 0xE7), (0xBC, 0xAA, 0xA4), (0xB3, 0x88, 0xFF), (0x7C, 0x4D, 0xFF), (0x7C, 0xB3, 0x42),
	(0x90, 0xCA, 0xF9), (0xB3, 0xE5, 0xFC), (0x1E, 0x88, 0xE5), (0xFF, 0x91, 0x00), (0x43, 0xA0, 0x47),
	(0xFF, 0xE0, 0xB2), (0x61, 0x61, 0x61), (0xE5, 0x39, 0x35), (0x54, 0x6E, 0x7A), (0x84, 0xFF, 0xFF),
	(0xD5, 0x00, 0xF9), (0xFF, 0x17, 0x44), (0x51, 0x2D, 0xA8), (0x00, 0x97, 0xA7), (0x9C, 0x27, 0xB0),
	(0x8B, 0xC3, 0x4A), (0xFF, 0xA0, 0x00), (0xD1, 0xC4, 0xE9), (0x21, 0x21, 0x21), (0x6D, 0x4C, 0x41),
	(0x67, 0x3A, 0xB7), (0xFF, 0xCA, 0x28), (0xFF, 0xC1, 0x07), (0xFF, 0xF3, 0xE0), (0x03, 0x9B, 0xE5),
	(0x42, 0x42, 0x42), (0x26, 0xC6, 0xDA), (0xC5, 0xCA, 0xE9), (0xC6, 0xFF, 0x00), (0xE8, 0xEA, 0xF6),
	(0x4F, 0xC3, 0xF7), (0x00, 0xAC, 0xC1), (0xFF, 0xA7, 0x26), (0x66, 0xBB, 0x6A), (0x02, 0x77, 0xBD),
	(0x5D, 0x40, 0x37), (0xFF, 0x8A, 0x80), (0xF4, 0x51, 0x1E), (0xDC, 0xED, 0xC8), (0xFF, 0xEE, 0x58),
	(0xEE, 0xFF, 0x41), (0xFC, 0xE4, 0xEC), (0x03, 0xA9, 0xF4), (0x80, 0xDE, 0xEA), (0x00, 0xB8, 0xD4),
	(0x4D, 0xB6, 0xAC), (0xFF, 0x57, 0x22), (0x62, 0x00, 0xEA), (0xF5, 0x7F, 0x17), (0xFF, 0x3D, 0x00),
	(0xF4, 0xFF, 0x81), (0x02, 0x88, 0xD1), (0x21, 0x96, 0xF3), (0xEF, 0x9A, 0x9A), (0xB0, 0xBE, 0xC5),
	(0xED, 0xE7, 0xF6), (0x00, 0xBC, 0xD4), (0xDD, 0x2C, 0x00), (0x2E, 0x7D, 0x32), (0x29, 0x79, 0xFF),
	(0xFF, 0xC4, 0x00), (0xE6, 0xEE, 0x9C), (0x4D, 0xD0, 0xE1), (0xFF, 0xE5, 0x7F), (0x00, 0xBF, 0xA5),
	(0x26, 0xA6, 0x9A), (0xFF, 0xF5, 0x9D), (0x60, 0x7D, 0x8B), (0xFF, 0x98, 0x00), (0xFF, 0xD1, 0x80),
	(0x6A, 0x1B, 0x9A), (0x21, 0x96, 0xF3), (0xF5, 0x7C, 0x00), (0x82, 0x77, 0x17), (0xAB, 0x47, 0xBC),
	(0x68, 0x9F, 0x38), (0xFF, 0xAB, 0x00), (0xBB, 0xDE, 0xFB), (0x8D, 0x6E, 0x63), (0xC6, 0x28, 0x28),
	(0xF4, 0x43, 0x36), (0xFF, 0xAB, 0x40), (0x00, 0x79, 0x6B), (0xCC, 0xFF, 0x90), (0xFF, 0xFF, 0x00),
	(0xB2, 0xDF, 0xDB), (0x60, 0x7D, 0x8B), (0x01, 0x57, 0x9B), (0xF5, 0x00, 0x57), (0xFA, 0xFA, 0xFA),
	(0x0D, 0x47, 0xA1), (0x8B, 0xC3, 0x4A), (0x00, 0x60, 0x64), (0xC8, 0xE6, 0xC9), (0x1B, 0x5E, 0x20),
	(0xB9, 0xF6, 0xCA), (0xFF, 0x9E, 0x80), (0x67, 0x3A, 0xB7), (0xFF, 0xCC, 0xBC), (0x9E, 0x9D, 0x24),
	(0x80, 0xD8, 0xFF), (0xFF, 0xFF, 0x8D), (0x30, 0x3F, 0x9F), (0x8E, 0x24, 0xAA), (0xB7, 0x1C, 0x1C),
	(0x00, 0x96, 0x88), (0xFF, 0xB7, 0x4D), (0xFF, 0xFD, 0xE7), (0x5E, 0x35, 0xB1), (0x53, 0x6D, 0xFE),
	(0x44, 0x8A, 0xFF), (0x31, 0x1B, 0x92), (0x65, 0x1F, 0xFF), (0xFF, 0xF1, 0x76), (0x64, 0xFF, 0xDA),
	(0x7B, 0x1F, 0xA2), (0xE6, 0x4A, 0x19), (0x40, 0xC4, 0xFF), (0x9E, 0x9E, 0x9E), (0x78, 0x90, 0x9C),
	(0xFF, 0x40, 0x81), (0x9C, 0x27, 0xB0), (0xFF, 0xF9, 0xC4), (0xFF, 0xD5, 0x4F), (0xBD, 0xBD, 0xBD),
	(0xCD, 0xDC, 0x39), (0xFF, 0x8A, 0x65), (0x7E, 0x57, 0xC2), (0x90, 0xA4, 0xAE), (0xFF, 0x8F, 0x00),
	(0xE9, 0x1E, 0x63), (0xFF, 0xCD, 0xD2), (0xAE, 0xEA, 0x00), (0x3E, 0x27, 0x23), (0xD8, 0x1B, 0x60),
	(0x3D, 0x5A, 0xFE), (0x42, 0xA5, 0xF5), (0xF8, 0xBB, 0xD0), (0xF4, 0x43, 0x36), (0x79, 0x55, 0x48),
	(0xFF, 0x98, 0x00), (0xFF, 0xF8, 0xE1), (0xFF, 0xE0, 0x82), (0xEE, 0xEE, 0xEE), (0xE8, 0xF5, 0xE9),
	(0xBA, 0x68, 0xC8), (0xF9, 0xA8, 0x25)
];

pub fn rand_color(n: u32) -> (u8, u8, u8, u8) {
	let c = RANDOM_FUCKING_COLORS[(n as usize) % 272];
	return (c.0, c.1, c.2, 255);
}

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
	let character_data = &vram_tile_data[(character_base_block as usize)..(0xFFFF - 1)];


	let __sw_mod = ((1 << screen_width) - 1);
	let __sh_mod = ((1 << screen_height) - 1);

	// screen_width & screen_height are going to be powers of 2.
	let pixel_y = ((line as u32) + yoffset as u32) & __sh_mod;
	let mut column = 0;

	{
		let pixel_x = (xoffset as u32) & __sw_mod;
		let sc = ((pixel_x >> 8) & 1) + (((pixel_y >> 8) & 1) << 1);
		let tile_x = (pixel_x & 255) >> 3;
		let tile_y = (pixel_y & 255) >> 3;
		let map_tile_data_addr = screen_base_block + (sc * kbytes!(2)) + (tile_y << 6) + (tile_x << 1);
		let map_tile_info = vram_tile_data.direct_read16(map_tile_data_addr as usize);
		tile_copy(palette, character_data, &mut bg_line[(column as usize)..((column as usize) + ((8 - (pixel_x & 7)) as usize))],
			map_tile_info, pixel_x & 7, pixel_y & 7);


		if pyrite_debugging!() && line == 0 {
			debug_println!(
				line,
				xoffset,
				yoffset,
				pixel_y,
				pixel_x,
				pixel_y & 7,
				pixel_x & 7,
				tile_x,
				tile_y,
				sc,
				(column as usize) + ((8 - (pixel_x & 7)) as usize)
			);
		}

		column += 8 - (pixel_x & 7); // Just going to do this instead though, for now.
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
			map_tile_info, 8, pixel_y & 7);
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
			map_tile_info, 8 - (240 - column), pixel_y & 7);
	}
}

pub fn copy_tile_line4bpp(palette: &[u8], char_data: &[u8], output: &mut [Pixel], tile_info: u16, tx: u32, ty: u32) {
	let tile_number = tile_info & 0x3ff;

	// #TODO implement these
	// let horizontal_flip = (tile_info >> 10) & 1;
	// let vertical_flip = (tile_info >> 11) & 1;
	let palette_number = (tile_info >> 12) & 0xf;

	// 32 bytes per tile
	// 4 bytes per row
	// each byte is 2 columns
	let mut offset = (((tile_number as u32) << 5) + (ty << 2) + (tx >> 1)) as usize;
	let mut pindex = 0;
	
	while pindex < output.len() {
		let two_dots = char_data[offset];

		// left pixel
		let left_dot = two_dots & 0xf;
		if left_dot & 15 == 0 {
			// If the color number is a multiple of 16 or 0, 
			// that means that it is color 0 of its palette, making it transparent.
			output[pindex] = (0, 0, 0, 0);
		} else {
			output[pindex] = convert_rgb5_to_rgba8(palette.direct_read16(((palette_number << 5) + ((left_dot << 1) as u16)) as usize));
		}
		pindex += 1;

		if pindex >= output.len() { break; }

		// right pixel
		let right_dot = (two_dots >> 4) & 0xf;
		if right_dot & 15 == 0 {
			// If the color number is a multiple of 16 or 0, 
			// that means that it is color 0 of its palette, making it transparent.
			output[pindex] = (0, 0, 0, 0);
		} else {
			output[pindex] = convert_rgb5_to_rgba8(palette.direct_read16(((palette_number << 5) + ((right_dot << 1) as u16)) as usize));
		}
		pindex += 1;

		offset += 1;
	}
}

pub fn copy_tile_line8bpp(palette: &[u8], char_data: &[u8], output: &mut [Pixel], tile_info: u16, tx: u32, ty: u32) {
	println!("8bpp");
}

