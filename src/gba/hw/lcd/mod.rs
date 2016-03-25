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

pub struct ObjLineInfo {
	/// I stuff all of the object's metadata in here.
	/// bits 0-2 priority
	/// bit  3   window
	/// bit  4   semi-transparent
	pub data: [u8; 240]
}

impl ObjLineInfo {
	#[inline(always)]
	pub fn get_priority(&self, idx: usize) -> u8 {
		self.data[idx] & 0x7
	}

	#[inline(always)]
	pub fn set_priority(&mut self, idx: usize, priority: u8) {
		self.data[idx] &= !0x7; // priorities can be overwritten...and stuff.
		self.data[idx] |= priority & 0x7;
	}

	#[inline(always)]
	pub fn is_window(&self, idx: usize) -> bool {
		((self.data[idx] >> 3) & 1) == 1
	}

	#[inline(always)]
	pub fn set_window(&mut self, idx: usize) {
		self.data[idx] |= 0x8;
	}

	#[inline(always)]
	pub fn is_transparent(&self, idx: usize) -> bool {
		((self.data[idx] >> 4) & 1) == 1
	}

	#[inline(always)]
	pub fn set_transparent(&mut self, idx: usize) {
		self.data[idx] |= 0x10;
	}
}

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

	pub bg0_enable: bool,
	pub bg1_enable: bool,
	pub bg2_enable: bool,
	pub bg3_enable: bool,

	pub obj_info: ObjLineInfo
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
				bg0_enable: false,
				bg1_enable: false,
				bg2_enable: false,
				bg3_enable: false,
				obj_info: ObjLineInfo { data: [0u8; 240] }
			}
		}
	}

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
			self.lines.obj_info.data[i] = 0;
		}
	}

	fn blend(&mut self, line: u16, memory: &GbaMemory) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);

		let backdrop = convert_rgb5_to_rgb8(memory.read16(0x05000000));
		let output = &mut self.screen_buffer[line as usize][0..240];

		let bg0_enabled = self.lines.bg0_enable && (((dispcnt >> 8) & 1) != 0);
		let bg1_enabled = self.lines.bg1_enable && (((dispcnt >> 9) & 1) != 0);
		let bg2_enabled = self.lines.bg2_enable && (((dispcnt >> 10) & 1) != 0);
		let bg3_enabled = self.lines.bg3_enable && (((dispcnt >> 11) & 1) != 0);

		let bg0_priority = memory.get_reg(ioreg::BG0CNT) & 0x3;
		let bg1_priority = memory.get_reg(ioreg::BG1CNT) & 0x3;
		let bg2_priority = memory.get_reg(ioreg::BG2CNT) & 0x3;
		let bg3_priority = memory.get_reg(ioreg::BG3CNT) & 0x3;

		let bldcnt = memory.get_reg(ioreg::BLDCNT);
		let blend_mode = (bldcnt >> 6) & 0x3;

		// (priority, bg, line)
		let mut rendering_order: [(u8, u16, Option<&GbaBGLine>); 4] = [(0, 0, None), (0, 0, None), (0, 0, None), (0, 0, None)];
		let mut rendering_order_idx = 0;
		for priority in (0..4).rev() {
			let mut temp_priority = rendering_order_idx;

			if bg3_enabled && bg3_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 3, Some(&self.lines.bg3));
				rendering_order_idx += 1;
			}

			if bg2_enabled && bg2_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 2, Some(&self.lines.bg2));
				rendering_order_idx += 1;
			}

			if bg1_enabled && bg1_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 1, Some(&self.lines.bg1));
				rendering_order_idx += 1;
			}

			if bg0_enabled && bg0_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 0, Some(&self.lines.bg0));
				rendering_order_idx += 1;
			}
		}

		// I'll have borrowing issues if I don't define this here like so:
		let obj_info = &self.lines.obj_info;
		let obj_line = &self.lines.obj;

		// #TODO split this into different functions that just take a pointer
		// to the rendering order because that's all they need, really.
		if blend_mode == 0 { // Alpha blending is disabled.
			let process_pixel = |priority: u8, pixel_idx: usize, dest: &mut [GbaPixel], maybe_line: Option<&GbaBGLine>| {
				let mut dest_pixel = dest[pixel_idx];

				// First we draw the BG's pixel (if there is one)
				if let Some(line) = maybe_line {
					let src_pixel = line[pixel_idx];
					dest_pixel = Self::blend_pixels(src_pixel, dest_pixel);
				}

				// Then we draw the OBJ's pixel on top if there is one at this priority.
				let obj_priority = obj_info.get_priority(pixel_idx);
				if obj_priority > 0 && (obj_priority - 1) == priority {
					let obj_pixel = obj_line[pixel_idx];
					dest_pixel = Self::blend_pixels(obj_pixel, dest_pixel);	
				}

				dest[pixel_idx] = dest_pixel;
			};

			for pix in 0..240 {
				// #TODO I'm drawing the OBJ's pixel multiple times. Is there any way to not do this?
				output[pix] = backdrop;
				process_pixel(rendering_order[0].0, pix, output, rendering_order[0].2);
				process_pixel(rendering_order[1].0, pix, output, rendering_order[1].2);
				process_pixel(rendering_order[2].0, pix, output, rendering_order[2].2);
				process_pixel(rendering_order[3].0, pix, output, rendering_order[3].2);
			}
		} else if blend_mode == 1 {
			/*
				4000050h - BLDCNT - Color Special Effects Selection (R/W)
					Bit   Expl.
					0     BG0 1st Target Pixel (Background 0)
					1     BG1 1st Target Pixel (Background 1)
					2     BG2 1st Target Pixel (Background 2)
					3     BG3 1st Target Pixel (Background 3)
					4     OBJ 1st Target Pixel (Top-most OBJ pixel)
					5     BD  1st Target Pixel (Backdrop)
					6-7   Color Special Effect (0-3, see below)
					     0 = None                (Special effects disabled)
					     1 = Alpha Blending      (1st+2nd Target mixed)
					     2 = Brightness Increase (1st Target becomes whiter)
					     3 = Brightness Decrease (1st Target becomes blacker)
					8     BG0 2nd Target Pixel (Background 0)
					9     BG1 2nd Target Pixel (Background 1)
					10    BG2 2nd Target Pixel (Background 2)
					11    BG3 2nd Target Pixel (Background 3)
					12    OBJ 2nd Target Pixel (Top-most OBJ pixel)
					13    BD  2nd Target Pixel (Backdrop)
					14-15 Not used
			*/
			let blend_sources = bldcnt & 0x3f;
			let blend_targets = (bldcnt >> 8) & 0x3f;

			let blend_alpha = memory.get_reg(ioreg::BLDALPHA);
			let blend_eva = min!(16, blend_alpha & 0x1f);
			let blend_evb = min!(16, (blend_alpha >> 8) & 0x1f);

			#[derive(Default)]
			struct BlendingShit {
				target_drawn: bool,
				source_on_top: bool,
				target_overwritten: bool,
				source_pixel: (u8, u8, u8),
				target_pixel: (u8, u8, u8)
			};

			let mut blending_shit = Default::default();

			// #TODO have this method return a darkened or brightened pixel as well, if we're doing that.
			let mut on_pixel_drawn = |layer_idx: u16, color: Pixel, blending_shit: &mut BlendingShit| {
				if color.3 != 0 {
					if ((blend_sources >> layer_idx) & 1) == 1 { // This is a source layer.
						blending_shit.source_on_top = true;
						blending_shit.source_pixel = (color.0, color.1, color.2);
					} else if ((blend_targets >> layer_idx) & 1) == 1 { // This is a target layer.
						if blending_shit.target_drawn {
							blending_shit.target_overwritten = true;
						} else {
							blending_shit.target_drawn = true;
							blending_shit.target_pixel = (color.0, color.1, color.2);
						}
						blending_shit.source_on_top = false;
					}
				}
			};

			on_pixel_drawn(5, (backdrop.0, backdrop.1, backdrop.2, 255), &mut blending_shit);

			let mut process_pixel = |priority: u8, bg: u16, pixel_idx: usize, dest: &mut [GbaPixel], maybe_line: Option<&GbaBGLine>,
					blending_shit: &mut BlendingShit| {
				let mut dest_pixel = dest[pixel_idx];

				// First we draw the BG's pixel (if there is one)
				if let Some(line) = maybe_line {
					let src_pixel = line[pixel_idx];
					dest_pixel = Self::blend_pixels(src_pixel, dest_pixel);
					on_pixel_drawn(bg, src_pixel, blending_shit);
				}

				// Then we draw the OBJ's pixel on top if there is one at this priority.
				let obj_priority = obj_info.get_priority(pixel_idx);
				if obj_priority > 0 && (obj_priority - 1) == priority {
					let obj_pixel = obj_line[pixel_idx];
					dest_pixel = Self::blend_pixels(obj_pixel, dest_pixel);	
					on_pixel_drawn(4, obj_pixel, blending_shit);
				}

				dest[pixel_idx] = dest_pixel;
			};

			for pix in 0..240 {
				// #TODO I'm drawing the OBJ's pixel multiple times. Is there any way to not do this?
				output[pix] = backdrop;
				process_pixel(rendering_order[0].0, rendering_order[0].1, pix, output, rendering_order[0].2, &mut blending_shit);
				process_pixel(rendering_order[1].0, rendering_order[1].1, pix, output, rendering_order[1].2, &mut blending_shit);
				process_pixel(rendering_order[2].0, rendering_order[2].1, pix, output, rendering_order[2].2, &mut blending_shit);
				process_pixel(rendering_order[3].0, rendering_order[3].1, pix, output, rendering_order[3].2, &mut blending_shit);

				if blending_shit.target_drawn && !blending_shit.target_overwritten && blending_shit.source_on_top {
					let out_pix = (
						min!(255, (((blending_shit.target_pixel.0 as u16) * blend_evb) >> 4) + (((blending_shit.source_pixel.0 as u16) * blend_eva) >> 4)) as u8,
						min!(255, (((blending_shit.target_pixel.1 as u16) * blend_evb) >> 4) + (((blending_shit.source_pixel.1 as u16) * blend_eva) >> 4)) as u8,
						min!(255, (((blending_shit.target_pixel.2 as u16) * blend_evb) >> 4) + (((blending_shit.source_pixel.2 as u16) * blend_eva) >> 4)) as u8,
					);
					output[pix] = out_pix;
				}

				blending_shit.target_drawn = false;
				blending_shit.source_on_top = false;
				blending_shit.target_overwritten = false;
			}
		} else if blend_mode == 2 {
			for pix in 0..240 {
				output[pix] = (pix as u8, line as u8, pix as u8)
			}
		} else if blend_mode == 3 {
			for pix in 0..240 {
				output[pix] = (pix as u8, pix as u8, line as u8)
			}
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
		if a.3 == 0 { return b }

		let aa = (a.3 as u32) + 1; // alpha component of a
		let aa_inv = 255 - (a.3 as u32);
		let _blend = |ca, cb| -> u8 {
			let ca = ca as u32; // color component of a
			let cb = cb as u32; // color component of b
			((aa * ca + aa_inv * cb) >> 8) as u8
		};
		(_blend(a.0, b.0), _blend(a.1, b.1), _blend(a.2, b.2))
	}
}

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
