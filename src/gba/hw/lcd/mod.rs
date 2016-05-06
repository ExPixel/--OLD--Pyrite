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

const MASK_BG0: u16 = 0x01;
const MASK_BG1: u16 = 0x02;
const MASK_BG2: u16 = 0x04;
const MASK_BG3: u16 = 0x08;
const MASK_OBJ: u16 = 0x10;
const MASK_SBD: u16 = 0x20; // Screen backdrop.

pub type GbaPixel = u16;
pub type OutputPixel = (u8, u8, u8);
pub type GbaLcdLine = Vec<OutputPixel>;
pub type GbaBGLine = [GbaPixel; 240];

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
		(self.data[idx] & 0x8) != 0
	}

	#[inline(always)]
	pub fn set_window(&mut self, idx: usize) {
		self.data[idx] |= 0x8;
	}

	#[inline(always)]
	pub fn is_semi_transparent(&self, idx: usize) -> bool {
		(self.data[idx] & 0x10) != 0
	}

	#[inline(always)]
	pub fn set_semi_transparent(&mut self, idx: usize) {
		self.data[idx] |= 0x10;
	}

	#[inline(always)]
	pub fn clear_semi_transparent(&mut self, idx: usize) {
		self.data[idx] &= !0x10;
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
				bg0: [0; 240],
				bg1: [0; 240],
				bg2: [0; 240],
				bg3: [0; 240],
				obj: [0; 240],
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

		self.blend_line(line, memory);
	}

	fn clear_obj_line(&mut self) {
		for i in 0..240 {
			self.lines.obj[i] = 0;
			self.lines.obj_info.data[i] = 0;
		}
	}

	// #TODO Consider making a second version of this function that doesn't use windows
	//       at all. This seems to be how VBA does it but right now I'm not sure if the performance
	//       gain will be worth the added complexity of maintaining both functions.
	fn blend_line(&mut self, line: u16, memory: &GbaMemory) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);

		let backdrop = opaque_rgb5(memory.read16(0x05000000));
		let output = &mut self.screen_buffer[line as usize][0..240];

		let bg0_enabled = self.lines.bg0_enable && (((dispcnt >> 8) & 1) != 0) && debug_layer_on!(0); // #TODO remove debug layer code.
		let bg1_enabled = self.lines.bg1_enable && (((dispcnt >> 9) & 1) != 0) && debug_layer_on!(1); // #TODO remove debug layer code.
		let bg2_enabled = self.lines.bg2_enable && (((dispcnt >> 10) & 1) != 0) && debug_layer_on!(2); // #TODO remove debug layer code.
		let bg3_enabled = self.lines.bg3_enable && (((dispcnt >> 11) & 1) != 0) && debug_layer_on!(3); // #TODO remove debug layer code.

		let obj_enabled = ((dispcnt >> 12) & 1) != 0 && debug_layer_on!(4); // #TODO remove debug layer code.

		let bg0_priority = memory.get_reg(ioreg::BG0CNT) & 0x3;
		let bg1_priority = memory.get_reg(ioreg::BG1CNT) & 0x3;
		let bg2_priority = memory.get_reg(ioreg::BG2CNT) & 0x3;
		let bg3_priority = memory.get_reg(ioreg::BG3CNT) & 0x3;

		let bldcnt = memory.get_reg(ioreg::BLDCNT);
		let blend_mode = (bldcnt >> 6) & 0x3;

		let blend_sources = bldcnt & 0x3f;
		let blend_targets = (bldcnt >> 8) & 0x3f;

		let blend_alpha = memory.get_reg(ioreg::BLDALPHA);
		let blend_eva = min!(16, blend_alpha & 0x1f);
		let blend_evb = min!(16, (blend_alpha >> 8) & 0x1f);

		let bldy = memory.get_reg(ioreg::BLDY);
		let blend_evy = bldy & 0x1f;

		let pixel_brightness_fn: fn(u16, GbaPixel) -> OutputPixel = match blend_mode {
			2 => brighten_pixel,
			3 => darken_pixel,
			_ => pixel_lum_nop
		};

		let win0_enabled = ((dispcnt >> 13) & 1) == 1;
		let win1_enabled = ((dispcnt >> 14) & 1) == 1;
		let win_obj_enabled = ((dispcnt >> 15) & 1) == 1;

		let win0h = memory.get_reg(ioreg::WIN0H);
		let win1h = memory.get_reg(ioreg::WIN1H);
		let win0v = memory.get_reg(ioreg::WIN0V);
		let win1v = memory.get_reg(ioreg::WIN1V);
		let winin = memory.get_reg(ioreg::WININ);
		let winout = memory.get_reg(ioreg::WINOUT);

		let win0_left = (win0h >> 8) & 0xff; // inclusive
		let win0_right = min!(240, win0h & 0xff); // exclusive
		let win0_top = (win0v >> 8) & 0xff; // inclusive
		let win0_bottom = min!(160, win0v & 0xff); // exclusive

		let win1_left = (win1h >> 8) & 0xff; // inclusive
		let win1_right = min!(240, win1h & 0xff); // exclusive
		let win1_top = (win1v >> 8) & 0xff; // inclusive
		let win1_bottom = min!(160, win1v & 0xff); // exclusive

		let win0_in = winin & 0x3f;
		let win1_in = (winin >> 8) & 0x3f;
		let winout_in = winout & 0x3f;
		let win_obj_in = (winout >> 8) & 0x3f;

		for pidx in 0..240 {
			let win_in_bits = if win0_enabled || win1_enabled || win_obj_enabled {
				if win0_enabled && window_contains(pidx as u16, line, win0_left, win0_right, win0_top, win0_bottom) {
					win0_in
				} else if win1_enabled && window_contains(pidx as u16, line, win1_left, win1_right, win1_top, win1_bottom) {
					win1_in
				} else if self.lines.obj_info.is_window(pidx) {
					win_obj_in
				} else {
					winout_in
				}
			} else {
				0xFFFF // Just turn on everything.
			};

			let allow_blending = blend_mode != 0 && (win_in_bits & 0x20) != 0;

			let mut semi_transparent_obj = false;
			let mut output_color = backdrop;

			let mut blend_target_layer = 0;
			let mut blend_target_color = 0;

			let mut top_layer = MASK_SBD;

			for priority in (0..4).rev() {

				if bg3_enabled && priority == bg3_priority && !is_transparent(self.lines.bg3[pidx]) && (win_in_bits & MASK_BG3) != 0 {
					blend_target_layer = top_layer;
					blend_target_color = output_color;
					output_color = self.lines.bg3[pidx];
					top_layer = MASK_BG3;
				}

				if bg2_enabled && priority == bg2_priority && !is_transparent(self.lines.bg2[pidx]) && (win_in_bits & MASK_BG2) != 0 {
					blend_target_layer = top_layer;
					blend_target_color = output_color;
					output_color = self.lines.bg2[pidx];
					top_layer = MASK_BG2;
				}

				if bg1_enabled && priority == bg1_priority && !is_transparent(self.lines.bg1[pidx]) && (win_in_bits & MASK_BG1) != 0 {
					blend_target_layer = top_layer;
					blend_target_color = output_color;
					output_color = self.lines.bg1[pidx];
					top_layer = MASK_BG1;
				}

				if bg0_enabled && priority == bg0_priority && !is_transparent(self.lines.bg0[pidx]) && (win_in_bits & MASK_BG0) != 0 {
					blend_target_layer = top_layer;
					blend_target_color = output_color;
					output_color = self.lines.bg0[pidx];
					top_layer = MASK_BG0;
				}

				if obj_enabled {
					let obj_priority = self.lines.obj_info.get_priority(pidx) as u16;
					if obj_priority!= 0 && priority == (obj_priority - 1) && !is_transparent(self.lines.obj[pidx]) && (win_in_bits & MASK_OBJ) != 0 {
						semi_transparent_obj = self.lines.obj_info.is_semi_transparent(pidx);
						blend_target_layer = top_layer;
						blend_target_color = output_color;
						output_color = self.lines.obj[pidx];
						top_layer = MASK_OBJ;
					}
				}
			}

			if allow_blending {
				if semi_transparent_obj && (top_layer & MASK_OBJ) != 0 {
					// semi transparent objs are always treated as blending 1st target (source)
					// getting here also means that the OBJ layer is the top layer
					// so now we try to get the next layer down.
					if (blend_targets & blend_target_layer) != 0 { // This is indeed a blending 2nd target pixel
						output[pidx] = blend_pixels(blend_eva, blend_evb, output_color, blend_target_color);
					} else {
						output[pidx] = pixel_brightness_fn(blend_evy, output_color);
					}
				} else {
					// Here we check that the top layer is a source layer.
					if (blend_sources & top_layer) != 0 {
						// Then if we check if we're blending or changing brightness.
						if blend_mode != 1 { // changing brightness
							output[pidx] = pixel_brightness_fn(blend_evy, output_color);
						} else if(blend_targets & blend_target_layer) != 0 { // blending two layers (also checking the second layer down is a 2nd target.)
							output[pidx] = blend_pixels(blend_eva, blend_evb, output_color, blend_target_color);
						} else { // No blending / brightness changes will occur.
							output[pidx] = expand_color(output_color);
						}
					} else {
						output[pidx] = expand_color(output_color);
					}
				}
			} else {
				output[pidx] = expand_color(output_color);
			}
		}
	}
}

#[inline(always)]
pub fn expand_color(rgb5: u16) -> OutputPixel {
	(
		(((rgb5 & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 5) & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 10) & 0x1f) * 527 + 23 ) >> 6) as u8
	)
}


#[inline(always)]
pub fn is_transparent(pixel: GbaPixel) -> bool {
	(pixel & 0x8000) == 0
}

/// Bit   Expl.
/// 0-4   Red Intensity   (0-31)
/// 5-9   Green Intensity (0-31)
/// 10-14 Blue Intensity  (0-31)
/// 15    Not used in GBA Mode (in NDS Mode: Alpha=0=Transparent, Alpha=1=Normal)
#[inline(always)]
pub fn opaque_rgb5(rgb5: u16) -> u16 {
	// We use the NDS's system because it uses less memory than passing RGB8 everywhere.
	rgb5 | 0x8000 // Setting the alpha bit to 1
}

#[inline(always)]
fn window_contains(x: u16, y: u16, w_left: u16, w_right: u16, w_top: u16, w_bottom: u16) -> bool {
	// #TODO make this handle the cross pattern that occurs when w_right is less than w_left
	(x >= w_left) && (x < w_right) &&
	(y >= w_top) && (y < w_bottom)
}

fn blend_pixels(eva: u16, evb: u16, front: GbaPixel, back: GbaPixel) -> OutputPixel {
	let (f_r, f_g, f_b) = expand_color(front);
	let (b_r, b_g, b_b) = expand_color(back);

	return (
		min!(255, (((f_r as u16) * eva) >> 4) + (((b_r as u16) * evb) >> 4)) as u8,
		min!(255, (((f_g as u16) * eva) >> 4) + (((b_g as u16) * evb) >> 4)) as u8,
		min!(255, (((f_b as u16) * eva) >> 4) + (((b_b as u16) * evb) >> 4)) as u8,
	);
}

// PIXEL BRIGHTNESS FUNCTIONS:

/// I1st + (31-I1st)*EVY
fn brighten_pixel(blend_evy: u16, color: GbaPixel) -> OutputPixel {
	let color = expand_color(color);
	(
		((color.0 as u16) + (((255 - (color.0 as u16)) * blend_evy) >> 4)) as u8,
		((color.1 as u16) + (((255 - (color.1 as u16)) * blend_evy) >> 4)) as u8,
		((color.2 as u16) + (((255 - (color.2 as u16)) * blend_evy) >> 4)) as u8
	)
}

/// I1st - (I1st)*EVY
fn darken_pixel(blend_evy: u16, color: GbaPixel) -> OutputPixel {
	let color = expand_color(color);
	(
		((color.0 as u16) - (((color.0 as u16) * blend_evy) >> 4)) as u8,
		((color.1 as u16) - (((color.1 as u16) * blend_evy) >> 4)) as u8,
		((color.2 as u16) - (((color.2 as u16) * blend_evy) >> 4)) as u8
	)
}

/// Just does nothing to the pixel
fn pixel_lum_nop(_: u16, color: GbaPixel) -> OutputPixel {
	expand_color(color)
}
