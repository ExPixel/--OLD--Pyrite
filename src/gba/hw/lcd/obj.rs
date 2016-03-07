use super::*;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::IORegister16;
use super::super::super::core::memory::ioreg::IORegister32;

/*
 When Rotation/Scaling used (Attribute 0, bit 8 set):
    9     Double-Size Flag     (0=Normal, 1=Double)
  When Rotation/Scaling not used (Attribute 0, bit 8 cleared):
    9     OBJ Disable          (0=Normal, 1=Not displayed)
*/

pub fn draw_objs(tiles_region: (u32, u32), one_dim: bool, hblank_free: bool, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let tile_region = memory.get_slice(tiles_region.0, tiles_region.1);
	let oam_region = memory.get_region(MEM_OAM);
	let mut attr_addr = 0;
	for obj in 0..128 {
		let attr0 = oam_region.direct_read16(attr_addr);

		if ((attr0 >> 8) & 1) == 0 {
			if ((attr0 >> 9) & 1) == 0 {
				let attr1 = oam_region.direct_read16(attr_addr + 2);
				let attr2 = oam_region.direct_read16(attr_addr + 4);
				draw_simple_obj(attr0, attr1, attr2, memory, line, lines);
			}
		} else {
			let attr1 = oam_region.direct_read16(attr_addr + 2);
			let attr2 = oam_region.direct_read16(attr_addr + 4);
			let attr_affine = oam_region.direct_read16(attr_addr + 6);
			draw_rot_scale_obj(attr0, attr1, attr2, attr_affine, memory, line, lines);
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

// temporary function while I try to reason about this whole thing.
// will inline it all soon.
fn check_obj_y_bounds(line: u16, ycoord: u16, height: u16) -> (bool, u16) {
	let bottom = (ycoord + height) & 0xff;
	if ycoord >= 160 {
		return (bottom >= line && bottom < 160, (height - 1) - (bottom - line)); // bottom < 160 because we want to catch the parts that wrapped and nothing else.
	} else {
		return (line >= ycoord && line < ((ycoord + height) & 0xff), line - ycoord)
	}
}

/// Draw an object with no rotation/scaling.
fn draw_simple_obj(attr0: u16, attr1: u16, attr2: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
	let ycoord = attr0 & 0xff;
	// not worrying about the obj mode for now.
	// let mode = (attr0 >> 10) & 0x3 // (0=Normal, 1=Semi-Transparent, 2=OBJ Window, 3=Prohibited)
	let xcoord = attr1 & 0x1ff;
	let mosaic = ((attr0 >> 12) & 1) == 1;
	let palette_type = ((attr0 >> 13) & 1) == 1; // (0=16/16, 1=256/1)
	let horizontal_flip = ((attr1 >> 12) & 1) == 1;
	let vertical_flip = ((attr1 >> 13) & 1) == 1;

	let size = {
		let shape = (attr0 >> 14) & 0x3; // (0=Square,1=Horizontal,2=Vertical,3=Prohibited)
		let size = (attr1 >> 14) & 0x3;
		OBJ_SIZES[((shape << 1) + size) as usize]
	};

	let (in_y_bounds, oy) = check_obj_y_bounds(line, ycoord, size.1);

	if in_y_bounds {
		for sx in xcoord..(xcoord + size.0) {
			if (sx & 0x1ff) < 240 {
				let ox = sx - xcoord;
				let fragment = ((oy >> 3) << size.2) + (ox >> 3); // like...I don't even know anymore.
				lines.obj[(sx & 0x1ff) as usize] = rand_color!(fragment);
			}
		}
	}
}

fn draw_rot_scale_obj(attr0: u16, attr1: u16, attr2: u16, attr_affine: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
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