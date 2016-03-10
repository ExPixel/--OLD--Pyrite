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

pub fn draw_objs(tiles_region: (u32, u32), one_dim: bool, hblank_free: bool, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let tile_region = memory.get_slice(tiles_region.0, tiles_region.1);
	let palette_region = memory.get_slice(0x05000200, 0x050003FF); // OBJ palettes are in a different location from tiles.
	let oam_region = memory.get_region(MEM_OAM);
	let mut attr_addr = 0;

	for _ in 0..128 {
		let attr0 = oam_region.direct_read16(attr_addr);

		if ((attr0 >> 8) & 1) == 0 {
			if ((attr0 >> 9) & 1) == 0 {
				let attr1 = oam_region.direct_read16(attr_addr + 2);
				let attr2 = oam_region.direct_read16(attr_addr + 4);
				draw_simple_obj(one_dim, tile_region, palette_region, attr0, attr1, attr2, line, lines);
			}
		} else {
			let attr1 = oam_region.direct_read16(attr_addr + 2);
			let attr2 = oam_region.direct_read16(attr_addr + 4);
			let attr_affine = oam_region.direct_read16(attr_addr + 6);
			draw_rot_scale_obj(one_dim, tile_region, palette_region, attr0, attr1, attr2, attr_affine, line, lines);
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

/*
8bit depth (256 colors, 1 palette)
Each tile occupies 64 bytes of memory, the first 8 bytes for the topmost row of the tile, and so on. 
Each byte selects the palette entry for each dot.
*/

pub fn get_simple_obj_dot_4bpp_1d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, size: (u16, u16, u16)) -> Pixel {
	let tile = attr2 & 0x1ff;
	// dividing by 8 to get width and height in 8x8 tiles.
	let fragment = ((oy >> 3) << size.2) + (ox >> 3);
	let tx = ox & 7;
	let ty = oy & 7;

	// (((tile as usize) + (fragment as usize)) << 5)
	//     fragment is a tile 
	//     32 byte sper tile
	// ((ty as usize) << 2) + ((tx as usize) >> 1)
	//     4 bytes per tile line
	//     1/2 byte per tile column
	let offset = (((tile as usize) + (fragment as usize)) << 5) + ((ty as usize) << 2) + ((tx as usize) >> 1);
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
	let tile = attr2 & 0x1ff;
	let tx = ox & 7;
	let ty = oy & 7;

	// turning oy into tile y
	// 32 bytes per tile
	// 32 tiles per line (put together with the one above it)
	let yoffset = ((oy as usize) >> 3) << 10;

	// turning ox into tile x
	// 32 bytes per tile
	let xoffset = ((ox as usize) >> 3) << 5;

	// ((ty as usize) << 2) + ((tx as usize) >> 1)
	// 4 bytes per tile line
	// 1/2 byte per tile column
	let offset = ((tile as usize) << 5) + yoffset + xoffset + ((ty as usize) << 2) + ((tx as usize) >> 1);
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

pub fn get_simple_obj_dot_8bpp_1d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, size: (u16, u16, u16)) -> Pixel {
	pyrite_debugging!({
		debug_println!(
			"get_simple_obj_dot_8bpp_1d",
			attr2, ox, oy
		);
	});
	(255, 0, 255, 255)
}

pub fn get_simple_obj_dot_8bpp_2d(tiles: &[u8], palette: &[u8], attr2: u16, ox: u16, oy: u16, size: (u16, u16, u16)) -> Pixel {
	pyrite_debugging!({
		debug_println!(
			"get_simple_obj_dot_8bpp_2d",
			attr2, ox, oy
		);
	});
	(255, 0, 255, 255)
}

/// Draw an object with no rotation/scaling.
fn draw_simple_obj(one_dimensional: bool, tile_region: &[u8], palette_region: &[u8], attr0: u16, attr1: u16, attr2: u16, line: u16, lines: &mut GbaDisplayLines) {
	let ycoord = attr0 & 0xff;
	// not worrying about the obj mode for now.
	// let mode = (attr0 >> 10) & 0x3 // (0=Normal, 1=Semi-Transparent, 2=OBJ Window, 3=Prohibited)
	let xcoord = attr1 & 0x1ff;
	// #TODO implement mosaics
	// let mosaic = ((attr0 >> 12) & 1) == 1;
	let horizontal_flip = ((attr1 >> 12) & 1) == 1;
	let vertical_flip = ((attr1 >> 13) & 1) == 1;

	let get_dot: fn(&[u8], &[u8], u16, u16, u16, (u16, u16, u16)) -> Pixel = if one_dimensional {
		if ((attr0 >> 13) & 1) == 1 { get_simple_obj_dot_8bpp_1d }
		else { get_simple_obj_dot_4bpp_1d }
	} else {
		if ((attr0 >> 13) & 1) == 1 { get_simple_obj_dot_8bpp_2d }
		else { get_simple_obj_dot_4bpp_2d }
	};

	let size = {
		let shape = (attr0 >> 14) & 0x3; // (0=Square,1=Horizontal,2=Vertical,3=Prohibited)
		let size = (attr1 >> 14) & 0x3;
		OBJ_SIZES[((shape << 1) + size) as usize]
	};

	// let (in_y_bounds, oy) = check_obj_y_bounds(line, ycoord, size.1);
	let bottom = (ycoord + size.1) & 0xff;
	let in_y_bounds;
	let mut oy;
	if ycoord >= 160 {
		in_y_bounds = bottom >= line && bottom < 160; // bottom < 160 because we want to catch the parts that wrapped and nothing else.
		oy = (size.1 - 1) - (bottom - line)
	} else {
		in_y_bounds = line >= ycoord && line < ((ycoord + size.1) & 0xff);
		oy = line - ycoord
	}

	if vertical_flip { oy = (size.1 - 1) - oy; }

	if in_y_bounds {
		for sx in xcoord..(xcoord + size.0) {
			if (sx & 0x1ff) < 240 {
				let mut ox = sx - xcoord;
				if horizontal_flip { ox = (size.0 - 1) - ox; }
				let dot = get_dot(tile_region, palette_region, attr2, ox, oy, size);
				if dot.3 != 0 {
					lines.obj[(sx & 0x1ff) as usize] = dot;
				}
			}
		}
	}
}

fn draw_rot_scale_obj(one_dimensional: bool, tile_region: &[u8], palette_region: &[u8], attr0: u16, attr1: u16, attr2: u16, attr_affine: u16, line: u16, lines: &mut GbaDisplayLines) {
	pyrite_debugging!({
		println!("drawing rot/scale object ({}): [0x{:04x}] [0x{:04x}] [0x{:04x}]", line, attr0, attr1, attr2);
	});
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