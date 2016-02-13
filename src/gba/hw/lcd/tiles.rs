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

const SCREEN_SIZES: [(u32, u32); 4] = [
	(256, 256),
	(512, 256),
	(256, 512),
	(512, 512)
];

pub fn draw_tiles_text_mode(bgcnt: u16, xoffset: u16, yoffset: u16, memory: &GbaMemory, line: u16, bg_line: &mut GbaBGLine) {
	let character_base_block = (((bgcnt >> 2) & 0x3) as u32) * kbytes!(16); // (0-3, in units of 16 KBytes) (=BG Tile Data)
	let mosaic = ((bgcnt >> 6) & 0x1) == 1;
	let palette = ((bgcnt >> 7) & 0x1) == 1; // 0=16/16 (4bit), 1=256/1 (8bit)
	let screen_base_block = (((bgcnt >> 8) & 0x1f) as u32) * kbytes!(2); // (0-31, in units of 2 KBytes) (=BG Map Data)

	// #FIXME: I think text mode always wraps so this might be unecessary.
	let display_area_overflow = ((bgcnt >> 13) & 0x1) == 1; // (0=Transparent, 1=Wraparound; BG2CNT/BG3CNT only)

	let screen_size = (bgcnt >> 14) & 0x3;

	let (screen_width, screen_height) = SCREEN_SIZES[screen_size as usize];

	// let character_data_size = if palette {} else {};
	// #FIXME for now we just slice the whole fucking thing. This might be bad but I don't see how it could fail.
	let character_data = memory.get_slice(0x06000000 + character_base_block, 0x0600FFFF);

	let get_sc = |sc: u32| {
		let off = 0x06000000 + screen_base_block + sc * kbytes!(2);
		memory.get_slice(off, off + kbytes!(2))
	};

	let __sw_mod = (screen_width - 1);
	let __sh_mod = (screen_height - 1);

	// screen_width & screen_height are going to be powers of 2.
	let screen_x = (xoffset as u32) & __sw_mod;
	let screen_y = (yoffset as u32) & __sh_mod;

	// splits the screen into the two regions of the screen that can
	// be seen through the 240x160 window.
	let mut x_regions = [
		(0, 0, 0), // (screen-x, real-screen-x, pixels)
		(0, 0, 0)
	];
	
	{
		let mut first_region_pixels = screen_width - screen_x;
		if first_region_pixels > 240 { first_region_pixels = 240; }
		x_regions[0] = (screen_x, 0, first_region_pixels);
		x_regions[1] = (0, first_region_pixels, 240 - first_region_pixels);
	}

	// draws a 256x256 tile area.
	let mut draw_tile_area = |sc: u32, x: u32, y: u32| {
		
	};

	match screen_size {
		0 => { // 256 x 256
			draw_tile_area(0, 0, 0);
		},
		1 => { // 512 x 256
			draw_tile_area(0, 0, 0);
			draw_tile_area(1, 256, 0);
		},
		2 => { // 256x512
			draw_tile_area(0, 0, 0);
			draw_tile_area(1, 0, 256);
		},
		3 => { // 512 x 512
			draw_tile_area(0, 0, 0);
			draw_tile_area(1, 256, 0); 
			draw_tile_area(2, 0, 256);
			draw_tile_area(3, 256, 256);
		},
		_ => unreachable!()
	}
}
