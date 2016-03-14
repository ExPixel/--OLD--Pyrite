use super::*;
use super::super::super::core::memory::*;
// use super::super::super::core::memory::ioreg::IORegister16;
// use super::super::super::core::memory::ioreg::IORegister32;



/*
 When Rotation/Scaling used (Attribute 0, bit 8 set):
    9     Double-Size Flag     (0=Normal, 1=Double)
  When Rotation/Scaling not used (Attribute 0, bit 8 cleared):
    9     OBJ Disable          (0=Normal, 1=Not displayed)
*/

#[derive(Default, Copy, Clone)]
struct ObjData {
	attr0: u16,
	attr1: u16,
	attr2: u16
}

#[derive(Default, Copy, Clone)]
struct ObjAffineData {
	pa: u16, // dx
	pb: u16, // dmx
	pc: u16, // dy
	pd: u16  // dmy
}

pub fn draw_objs(tiles_region: (u32, u32), one_dim: bool, hblank_free: bool, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let tile_region = memory.get_slice(tiles_region.0, tiles_region.1);
	let palette_region = memory.get_slice(0x05000200, 0x050003FF); // OBJ palettes are in a different location from tiles.
	let oam_region = memory.get_region(MEM_OAM);

	let mut obj_data: ObjData = Default::default();
	let mut affine_data: ObjAffineData = Default::default();

	let mut attr_addr = 0;

	for o in 0..128 {
		// #debug debug code for the obj demo.
		pyrite_debugging!({
			if o == 1 {
				attr_addr += 8;
				continue;
			}
		});

		obj_data.attr0 = oam_region.direct_read16(attr_addr);
		if ((obj_data.attr0 >> 8) & 1) == 0 {
			if ((obj_data.attr0 >> 9) & 1) == 0 {
				obj_data.attr1 = oam_region.direct_read16(attr_addr + 2);
				obj_data.attr2 = oam_region.direct_read16(attr_addr + 4);
				draw_simple_obj(one_dim, tile_region, palette_region, obj_data, line, lines);
			}
		} else {
			obj_data.attr1 = oam_region.direct_read16(attr_addr + 2);
			obj_data.attr2 = oam_region.direct_read16(attr_addr + 4);
			let rot_scale_params_off = (((obj_data.attr1 >> 9) & 0x1f) as usize) << 5;

			affine_data.pa = oam_region.direct_read16( 6 + rot_scale_params_off );
			affine_data.pb = oam_region.direct_read16( 14 + rot_scale_params_off );
			affine_data.pc = oam_region.direct_read16( 22 + rot_scale_params_off );
			affine_data.pd = oam_region.direct_read16( 30 + rot_scale_params_off );

			pyrite_debugging!({
				println!("----- OBJ [{}] -----", o);
				println!("attr0: 0x{:04x}", obj_data.attr0);
				println!("attr1: 0x{:04x}", obj_data.attr1);
				println!("attr2: 0x{:04x}", obj_data.attr2);
				println!("PA: 0x{:04x}", affine_data.pa);
				println!("PB: 0x{:04x}", affine_data.pb);
				println!("PC: 0x{:04x}", affine_data.pc);
				println!("PD: 0x{:04x}", affine_data.pd);
				println!("P-Select: 0x{:04x}", ((obj_data.attr1 >> 9) & 0x1f));
				println!("PA-Loc: 0x{:04x}", 6 + rot_scale_params_off);
				println!("PB-Loc: 0x{:04x}", 14 + rot_scale_params_off);
				println!("PC-Loc: 0x{:04x}", 22 + rot_scale_params_off);
				println!("PD-Loc: 0x{:04x}", 30 + rot_scale_params_off);
			});

			draw_rot_scale_obj(one_dim, tile_region, palette_region, obj_data, affine_data, line, lines);
		}
		attr_addr += 8; // there's an empty slot for rot/scale
	} 
}


// (width, height, shift-per-line)
const OBJ_SIZES: [(u16, u16, u16); 16] = [
	(8, 8, 0), (16, 16, 1), (32, 32, 2), (64, 64, 3), // square
	(16, 8, 1), (32, 8, 2), (32, 16, 2), (64, 32, 3), // horizontal
	(8, 16, 0), (8, 32, 0), (16, 32, 1), (32, 64, 2), // vertical 
	(8, 8, 0), (16, 16, 1), (32, 32, 2), (64, 64, 3)  // Prohibited (we mirror square, though) #TODO might remove this.
];

pub fn get_simple_obj_dot_4bpp_1d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, size: (u16, u16, u16)) -> Pixel {
	let tile = attr2 & 0x3ff;
	// dividing by 8 to get width and height in 8x8 tiles.
	let fragment = ((oy >> 3) * (size.1 >> 3)) + (ox >> 3);
	let tx = ox & 7;
	let ty = oy & 7;

	// (((tile as usize) + (fragment as usize)) << 5)
	//     fragment is a tile 
	//     32 bytes per tile
	// ((ty as usize) << 2) + ((tx as usize) >> 1)
	//     4 bytes per tile line
	//     1/2 byte per tile column
	let offset = (((tile as usize) + (fragment as usize)) << 5) + ((ty as usize) << 2) + ((tx as usize) >> 1);
	if offset >= tiles.len() { return (0, 0, 0, 0) }
	let dot = ((tiles[offset] >> ((tx & 1) << 2)) & 0xf) as usize;
	return if dot == 0 { 
		(0, 0, 0, 0)
	} else { 
		// 32 bytes per palette
		// 2 bytes per color entry
		let palette_number = ((attr2 >> 12) & 0x3) as usize;
		convert_rgb5_to_rgba8(palette.direct_read16( (palette_number << 5) + (dot << 1) ))
	}
}

pub fn get_simple_obj_dot_4bpp_2d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, _: (u16, u16, u16)) -> Pixel {
	let tile = attr2 & 0x3ff;
	let tx = ox & 7;
	let ty = oy & 7;

	// turning oy into tile y
	// 32 bytes per tile
	// 32 tiles per line (put together with the one above it)
	let yoffset = ((oy as usize) >> 3) << 10;

	// turning ox into tile x
	// 32 bytes per tile
	let xoffset = ((ox as usize) >> 3) << 5;

	// ((tile as usize) << 5)
	//     32 bytes per tile
	// ((ty as usize) << 2) + ((tx as usize) >> 1)
	//     4 bytes per tile line
	//     1/2 byte per tile column
	let offset = ((tile as usize) << 5) + yoffset + xoffset + ((ty as usize) << 2) + ((tx as usize) >> 1);
	if offset >= tiles.len() { return (0, 0, 0, 0) }
	let dot = ((tiles[offset] >> ((tx & 1) << 2)) & 0xf) as usize;
	return if dot == 0 { 
		(0, 0, 0, 0)
	} else { 
		// 32 bytes per palette
		// 2 bytes per color entry
		let palette_number = ((attr2 >> 12) & 0x3) as usize;
		convert_rgb5_to_rgba8(palette.direct_read16( (palette_number << 5) + (dot << 1) ))
	}
}

/*
8bit depth (256 colors, 1 palette)
Each tile occupies 64 bytes of memory, the first 8 bytes for the topmost row of the tile, and so on. 
Each byte selects the palette entry for each dot.
*/

pub fn get_simple_obj_dot_8bpp_1d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, size: (u16, u16, u16)) -> Pixel {
	let tile = (attr2 & 0x3ff) & !1; // ignores bit 1

	// dividing by 8 to get width and height in 8x8 tiles.
	let fragment = ((oy >> 3) << size.2) + (ox >> 3);
	let tx = ox & 7;
	let ty = oy & 7;

	// tile index only references 32 bytes at a time.
	let offset = ((tile as usize) << 5) + ((fragment as usize) << 6) + ((ty as usize) << 3) + (tx as usize);
	if offset >= tiles.len() { return (0, 0, 0, 0) }
	let dot = tiles[offset] as usize;
	return if dot == 0 { 
		(0, 0, 0, 0)
	} else {
		// 2 bytes per color entry
		convert_rgb5_to_rgba8(palette.direct_read16(dot << 1))
	}
}

pub fn get_simple_obj_dot_8bpp_2d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, size: (u16, u16, u16)) -> Pixel {
	let tile = (attr2 & 0x3ff) & !1;
	let tx = ox & 7;
	let ty = oy & 7;

	// turning oy into tile y
	// 64 bytes per tile
	// 32 tiles per line (put together with the one above it)
	let yoffset = (((oy as usize) >> 3) << 6) << 5;

	// turning ox into tile x
	// 64 bytes per tile
	let xoffset = ((ox as usize) >> 3) << 6;

	let offset = ((tile as usize) << 5) + yoffset + xoffset + ((ty as usize) << 3) + (tx as usize);
	if offset >= tiles.len() { return (0, 0, 0, 0) }
	let dot = tiles[offset] as usize;
	return if dot == 0 { 
		(0, 0, 0, 0)
	} else {
		// 2 bytes per color entry
		convert_rgb5_to_rgba8(palette.direct_read16(dot << 1))
	}
}

/// Draw an object with no rotation/scaling.
fn draw_simple_obj(one_dimensional: bool, tile_region: &[u8], palette_region: &[u8], obj: ObjData, line: u16, lines: &mut GbaDisplayLines) {
	let ycoord = obj.attr0 & 0xff;
	// not worrying about the obj mode for now.
	// let mode = (attr0 >> 10) & 0x3 // (0=Normal, 1=Semi-Transparent, 2=OBJ Window, 3=Prohibited)
	let xcoord = obj.attr1 & 0x1ff;
	// #TODO implement mosaics
	// let mosaic = ((attr0 >> 12) & 1) == 1;
	let horizontal_flip = ((obj.attr1 >> 12) & 1) == 1;
	let vertical_flip = ((obj.attr1 >> 13) & 1) == 1;

	let get_dot: fn(&[u8], &[u8], u16, u16, u16, (u16, u16, u16)) -> Pixel = if one_dimensional {
		if ((obj.attr0 >> 13) & 1) == 1 { get_simple_obj_dot_8bpp_1d }
		else { get_simple_obj_dot_4bpp_1d }
	} else {
		if ((obj.attr0 >> 13) & 1) == 1 { get_simple_obj_dot_8bpp_2d }
		else { get_simple_obj_dot_4bpp_2d }
	};

	let (width, height, line_shift) = {
		let shape = (obj.attr0 >> 14) & 0x3; // (0=Square,1=Horizontal,2=Vertical,3=Prohibited)
		let size = (obj.attr1 >> 14) & 0x3;
		OBJ_SIZES[((shape << 1) + size) as usize]
	};

	let mut px = obj.attr1 & 0x1ff;
	let mut py = obj.attr0 & 0xff;

	if py + height > 256 {
		py -= 256;
	}

	if (line - py) < height { // negatives will wrap (making them larger)
		let ty = line - py;// texture y
		let f_ty = if vertical_flip { height - ty } else { ty }; // possibly flipped ty

		let tx_offset = if (px + width) > 512 { 512 - px } else { 0 };
		if (px < 240) || tx_offset != 0 {
			for tx in 0..width {
				if px < 240 {
					let f_tx = if horizontal_flip { width - tx } else { tx }; // possibly flipped tx.
					lines.obj[px as usize] = get_dot(tile_region, palette_region, obj.attr2, f_tx, f_ty, (width, height, line_shift));
				}
				px = (px + 1) & 0x1ff;
			}
		}
	}
}

fn draw_rot_scale_obj(one_dimensional: bool, tile_region: &[u8], palette_region: &[u8], obj: ObjData, affine: ObjAffineData, line: u16, lines: &mut GbaDisplayLines) {
}

/*
OBJ Attribute 0 (R/W)
  Bit   Expl.
  0-7   Y-Coordinate           (0-255)
  8     Rotation/Scaling Flag  (0=Off, 1=On)
  When Rotation/Scaling used (Attribute 0, bit 8 set):
    9     Double-Size Flag     (0=Normal, 1=Double)
  When Rotation/Scaling not used (Attribute 0, bit 8 cleared):
    9     OBJ Disable          (0=Normal, 1=Not displayed)
  10-11 OBJ Mode  (0=Normal, 1=Semi-Transparent, 2=OBJ Window, 3=Prohibited)
  12    OBJ Mosaic             (0=Off, 1=On)
  13    Colors/Palettes        (0=16/16, 1=256/1)
  14-15 OBJ Shape              (0=Square,1=Horizontal,2=Vertical,3=Prohibited)
Caution: A very large OBJ (of 128 pixels vertically, ie. a 64 pixels OBJ in a Double Size area) located at Y>128 will be treated as at Y>-128, the OBJ is then displayed parts offscreen at the TOP of the display, it is then NOT displayed at the bottom.

OBJ Attribute 1 (R/W)
  Bit   Expl.
  0-8   X-Coordinate           (0-511)
  When Rotation/Scaling used (Attribute 0, bit 8 set):
    9-13  Rotation/Scaling Parameter Selection (0-31)
          (Selects one of the 32 Rotation/Scaling Parameters that
          can be defined in OAM, for details read next chapter.)
  When Rotation/Scaling not used (Attribute 0, bit 8 cleared):
    9-11  Not used
    12    Horizontal Flip      (0=Normal, 1=Mirrored)
    13    Vertical Flip        (0=Normal, 1=Mirrored)
  14-15 OBJ Size               (0..3, depends on OBJ Shape, see Attr 0)
          Size  Square   Horizontal  Vertical
          0     8x8      16x8        8x16
          1     16x16    32x8        8x32
          2     32x32    32x16       16x32
          3     64x64    64x32       32x64

OBJ Attribute 2 (R/W)
  Bit   Expl.
  0-9   Character Name          (0-1023=Tile Number)
  10-11 Priority relative to BG (0-3; 0=Highest)
  12-15 Palette Number   (0-15) (Not used in 256 color/1 palette mode)
*/