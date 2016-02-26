use super::super::core::memory::*;

pub mod tiles;

// #TODO remove these allows
pub mod mode0;

#[allow(unused_variables)]
pub mod mode1;

#[allow(unused_variables)]
pub mod mode2;

pub mod mode3;
pub mod mode4;
pub mod mode5;



pub type Pixel = (u8, u8, u8, u8);
pub type GbaPixel = (u8, u8, u8);
pub type GbaLcdLine = Vec<GbaPixel>;
pub type GbaBGLine = [Pixel; 240];

// No point in having a secondary screen buffer
// since the GBA renders in scan lines anyway.
pub type GbaLcdScreenBuffer = Vec<GbaLcdLine>;


pub struct GbaDisplayLines {
	pub bg0: GbaBGLine,
	pub bg1: GbaBGLine,
	pub bg2: GbaBGLine,
	pub bg3: GbaBGLine,
	pub obj: GbaBGLine
}

pub struct GbaLcd {
	pub screen_buffer: GbaLcdScreenBuffer,
	pub lines: GbaDisplayLines
}

impl GbaLcd {
	pub fn new() -> GbaLcd {
		GbaLcd {
			screen_buffer: vec![vec![(0u8, 0u8, 0u8); 240]; 160],
			lines: GbaDisplayLines {
				bg0: [(0, 0, 0, 0); 240],
				bg1: [(0, 0, 0, 0); 240],
				bg2: [(0, 0, 0, 0); 240],
				bg3: [(0, 0, 0, 0); 240],
				obj: [(0, 0, 0, 0); 240]
			}
		}
	}

	/// Causes the LCD to render a single line.
	#[allow(unused_variables)] // #TODO remove this
	pub fn render_line(&mut self, memory: &GbaMemory, line: u16) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);

		match dispcnt & 0x7 {
			0 => mode0::render_mode_0(dispcnt, memory, line, &mut self.lines),
			1 => mode1::render_mode_1(dispcnt, memory, line, &mut self.lines),
			2 => mode2::render_mode_2(dispcnt, memory, line, &mut self.lines),
			3 => mode3::render_mode_3(dispcnt, memory, line, &mut self.lines),
			4 => mode4::render_mode_4(dispcnt, memory, line, &mut self.lines),
			5 => mode5::render_mode_5(dispcnt, memory, line, &mut self.lines),
			_ => unreachable!()
		}

		self.blend(line, memory);
	}

	fn blend(&mut self, line: u16, memory: &GbaMemory) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);

		let transparent = memory.read16(0x05000000);
		let transparent_color = convert_rgb5_to_rgb8(transparent);
		let output = &mut self.screen_buffer[line as usize][0..240];

		for idx in 0..240 {
			output[idx] = transparent_color; // Make the line transparent to start with.
		}

		let bg0_enabled = ((dispcnt >> 8) & 1) != 0;
		let bg1_enabled = ((dispcnt >> 9) & 1) != 0;
		let bg2_enabled = ((dispcnt >> 10) & 1) != 0;
		let bg3_enabled = ((dispcnt >> 11) & 1) != 0;

		let bg0_priority = memory.get_reg(ioreg::BG0CNT) & 0x3;
		let bg1_priority = memory.get_reg(ioreg::BG1CNT) & 0x3;
		let bg2_priority = memory.get_reg(ioreg::BG2CNT) & 0x3;
		let bg3_priority = memory.get_reg(ioreg::BG3CNT) & 0x3;

		// I can turn this into one operation, by organizing the lines into one array
		// of arrays and then using that instead...
		// blend_pixels( lines[0][idx] )
		// blend_pixels( lines[1][idx] )
		// ...
		for priority in (0..4).rev() {
			if bg3_enabled && bg3_priority == priority { Self::blend_lines(&self.lines.bg3, output);}
			if bg2_enabled && bg2_priority == priority { Self::blend_lines(&self.lines.bg2, output);}
			if bg1_enabled && bg1_priority == priority { Self::blend_lines(&self.lines.bg1, output);}
			if bg0_enabled && bg0_priority == priority { Self::blend_lines(&self.lines.bg0, output);}
		}
		pyrite_debugging!({println!("...")});
	}

	fn blend_lines(src: &[Pixel], dest: &mut [GbaPixel]) {
		// #TODO SIMD pls.
		for idx in 0..240 {
			let dest_pixel = dest[idx];
			let src_pixel = src[idx];
			let out_pixel = Self::blend_pixels(src_pixel, dest_pixel);
			dest[idx] = out_pixel;
		}
	}

	#[inline(always)]
	fn blend_pixels(a: Pixel, b: GbaPixel) -> GbaPixel {
		let aa = (a.3 as u32) + 1; // alpha component of a
		let aa_inv = 256 - (a.3 as u32);
		let _blend = |ca, cb| -> u8 {
			let ca = ca as u32; // color component of a
			let cb = cb as u32; // color component of b
			((aa * ca + aa_inv * cb) >> 8) as u8
		};
		(_blend(a.0, b.0), _blend(a.1, b.1), _blend(a.2, b.2))
	}
}

/*
rOut = (rA * aA / 255) + (rB * aB * (255 - aA) / (255*255))
gOut = (gA * aA / 255) + (gB * aB * (255 - aA) / (255*255))
bOut = (bA * aA / 255) + (bB * aB * (255 - aA) / (255*255))
aOut = aA + (aB * (255 - aA) / 255)
*/

/// Bit   Expl.
/// 0-4   Red Intensity   (0-31)
/// 5-9   Green Intensity (0-31)
/// 10-14 Blue Intensity  (0-31)
/// 15    Not used in GBA Mode (in NDS Mode: Alpha=0=Transparent, Alpha=1=Normal)
#[inline(always)]
pub fn convert_rgb5_to_rgb8(rgb5: u16) -> GbaPixel {
	(
		(((rgb5 & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 5) & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 10) & 0x1f) * 527 + 23 ) >> 6) as u8
	)
}

/// Bit   Expl.
/// 0-4   Red Intensity   (0-31)
/// 5-9   Green Intensity (0-31)
/// 10-14 Blue Intensity  (0-31)
/// 15    Not used in GBA Mode (in NDS Mode: Alpha=0=Transparent, Alpha=1=Normal)
#[inline(always)]
pub fn convert_rgb5_to_rgba8(rgb5: u16) -> Pixel {
	(
		(((rgb5 & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 5) & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 10) & 0x1f) * 527 + 23 ) >> 6) as u8,
		255
	)
}
