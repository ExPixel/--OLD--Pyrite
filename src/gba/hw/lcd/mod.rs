use super::super::core::memory::*;

pub mod tiles;
pub mod obj;

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

// Add a way for modes themselves to turn these off
// mode 3 for instance only uses bg2, no need for the others.
pub struct GbaDisplayLines {
	pub bg0: GbaBGLine,
	pub bg1: GbaBGLine,
	pub bg2: GbaBGLine,
	pub bg3: GbaBGLine,
	pub obj: GbaBGLine,
	pub obj_priorities: [u8; 240]
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
				obj: [(0, 0, 0, 0); 240],
				obj_priorities: [0u8; 240]
			}
		}
	}

	/// Causes the LCD to render a single line.
	#[allow(unused_variables)] // #TODO remove this
	pub fn render_line(&mut self, memory: &mut GbaMemory, line: u16) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);
		self.clear_obj_line();

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

	fn clear_obj_line(&mut self) {
		for i in 0..240 {
			self.lines.obj[i] = (0, 0, 0, 0);
			self.lines.obj_priorities[i] = 0;
		}
	}

	fn blend(&mut self, line: u16, memory: &GbaMemory) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);
		
		let transparent_color = convert_rgb5_to_rgb8(memory.read16(0x05000000));
		let output = &mut self.screen_buffer[line as usize][0..240];

		let bg0_enabled = ((dispcnt >> 8) & 1) != 0;
		let bg1_enabled = ((dispcnt >> 9) & 1) != 0;
		let bg2_enabled = ((dispcnt >> 10) & 1) != 0;
		let bg3_enabled = ((dispcnt >> 11) & 1) != 0;

		let bg0_priority = memory.get_reg(ioreg::BG0CNT) & 0x3;
		let bg1_priority = memory.get_reg(ioreg::BG1CNT) & 0x3;
		let bg2_priority = memory.get_reg(ioreg::BG2CNT) & 0x3;
		let bg3_priority = memory.get_reg(ioreg::BG3CNT) & 0x3;

		// (priority, line)
		let mut rendering_order: [(u8, Option<&GbaBGLine>); 4] = [(0, None), (0, None), (0, None), (0, None)];
		let mut rendering_order_idx = 0;
		for priority in 0..4 {
			let mut temp_priority = rendering_order_idx;

			if bg0_enabled && bg0_priority == priority {
				rendering_order[rendering_order_idx] = (0, Some(&self.lines.bg0));
				rendering_order_idx += 1;
			}

			if bg1_enabled && bg1_priority == priority {
				rendering_order[rendering_order_idx] = (0, Some(&self.lines.bg1));
				rendering_order_idx += 1;
			}

			if bg2_enabled && bg2_priority == priority {
				rendering_order[rendering_order_idx] = (0, Some(&self.lines.bg2));
				rendering_order_idx += 1;
			}

			if bg3_enabled && bg3_priority == priority {
				rendering_order[rendering_order_idx] = (0, Some(&self.lines.bg3));
				rendering_order_idx += 1;
			}
		}

		// I'll have borrowing issues if I don't define this here like so:
		let obj_priorities = self.lines.obj_priorities;
		let obj_line = self.lines.obj;

		let process_pixel = |priority: u8, pixel_idx: usize, dest: &mut [GbaPixel], maybe_line: Option<&GbaBGLine>| {
			let mut dest_pixel = dest[pixel_idx];

			// First we draw the BG's pixel (if there is one)
			if let Some(line) = maybe_line {
				let src_pixel = line[pixel_idx];
				dest_pixel = Self::blend_pixels(src_pixel, dest_pixel);
			}

			// Then we draw the OBJ's pixel on top if there is one at this priority.
			let obj_priority = obj_priorities[pixel_idx];
			if obj_priority > 0 && (obj_priority - 1) == priority {
				let obj_pixel = obj_line[pixel_idx];
				dest_pixel = Self::blend_pixels(obj_pixel, dest_pixel);	
			}

			dest[pixel_idx] = dest_pixel;
		};

		for pix in 0..240 {
			// #TODO I'm drawing the OBJ's pixel multiple times. Is there any way to not do this?
			output[pix] = transparent_color;
			process_pixel(rendering_order[0].0, pix, output, rendering_order[0].1);
			process_pixel(rendering_order[1].0, pix, output, rendering_order[1].1);
			process_pixel(rendering_order[2].0, pix, output, rendering_order[2].1);
			process_pixel(rendering_order[3].0, pix, output, rendering_order[3].1);
		}
	}

	fn blend_lines(src: &[Pixel], dest: &mut [GbaPixel]) {
		for idx in 0..240 {
			let dest_pixel = dest[idx];
			let src_pixel = src[idx];
			let out_pixel = Self::blend_pixels(src_pixel, dest_pixel);
			dest[idx] = out_pixel;
		}
	}

	#[inline(always)]
	fn blend_pixels(a: Pixel, b: GbaPixel) -> GbaPixel {
		if a.3 == 0 { return b } // It's not going to show up.

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
