use super::*;
use super::super::super::core::memory::*;


// const SCREEN_SIZES: [(u32, u32); 8] = [
// 	// Text Mode
// 	(256, 256),
// 	(512, 256),
// 	(256, 512),
// 	(512, 512),

// 	// Rotation / Scaling Mode
// 	(128, 128),
// 	(256, 256),
// 	(512, 512),
// 	(1024, 1024)
// ];

/// Value  Text Mode      Rotation/Scaling Mode
///   0      256x256 (2K)   128x128   (256 bytes)
///   1      512x256 (4K)   256x256   (1K)
///   2      256x512 (4K)   512x512   (4K)
///   3      512x512 (8K)   1024x1024 (16K)

/// Screen sizes for shifts instead of multiplication and
/// division.
const SCREEN_SIZES: [(u32, u32); 8] = [
	// Text Mode
	(8, 8),
	(9, 8),
	(8, 9),
	(9, 9),

	// Rotation / Scaling Mode
	(7, 7),
	(8, 8),
	(9, 9),
	(10, 10)
];

/// BG Mode 0,1,2 (Tile/Map based Modes)  
///   06000000-0600FFFF  64 KBytes shared for BG Map and Tiles  
///   06010000-06017FFF  32 KBytes OBJ Tiles  
/// The shared 64K area can be split into BG Map area(s), and BG Tiles area(s),
/// the respective addresses for Map and Tile areas are set up by BG0CNT-BG3CNT registers. 
/// The Map address may be specified in units of 2K (steps of 800h), the Tile address in units of 16K (steps of 4000h).  
/// 
/// BG Mode 0,1 (Tile/Map based Text mode)  
/// The tiles may have 4bit or 8bit color depth, minimum map size is 32x32 tiles, maximum is 64x64 tiles, up to 1024 tiles can be used per map.  
///   Item        Depth     Required Memory  
///   One Tile    4bit      20h bytes  
///   One Tile    8bit      40h bytes  
///   1024 Tiles  4bit      8000h (32K)  
///   1024 Tiles  8bit      10000h (64K) - excluding some bytes for BG map  
///   BG Map      32x32     800h (2K)  
///   BG Map      64x64     2000h (8K)
///
/// In 'Text Modes', the screen size is organized as follows: The screen consists of one or more 256x256 pixel (32x32 tiles) areas.
/// When Size=0: only 1 area (SC0), when Size=1 or Size=2: two areas (SC0,SC1 either horizontally or vertically arranged next to each other), 
/// when Size=3: four areas (SC0,SC1 in upper row, SC2,SC3 in lower row). 
/// Whereas SC0 is defined by the normal BG Map base address (Bit 8-12 of BG#CNT), 
/// SC1 uses same address +2K, SC2 address +4K, SC3 address +6K. When the screen is scrolled it'll always wraparound.
pub fn draw_tiles_text_mode(bgcnt: u16, xoffset: u16, yoffset: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let character_base_block = ((bgcnt >> 2) & 0x3) * 16 * 1024; // (0-3, in units of 16 KBytes) (=BG Tile Data)
	let mosaic = ((bgcnt >> 6) & 0x1) == 1;
	let palette = ((bgcnt >> 7) & 0x1) == 1; // 0=16/16, 1=256/1
	let screen_base_block = ((bgcnt >> 8) & 0x1f) * 2 * 1024; // (0-31, in units of 2 KBytes) (=BG Map Data)
	let display_area_overflow = ((bgcnt >> 13) & 0x1) == 1; // (0=Transparent, 1=Wraparound; BG2CNT/BG3CNT only)
	let screen_size = SCREEN_SIZES[((bgcnt >> 14) & 0x3) as usize];
}
