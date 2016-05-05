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
	pub fn is_transparent(&self, idx: usize) -> bool {
		(self.data[idx] & 0x10) != 0
	}

	#[inline(always)]
	pub fn set_transparent(&mut self, idx: usize) {
		self.data[idx] |= 0x10;
	}

	#[inline(always)]
	pub fn clear_transparent(&mut self, idx: usize) {
		self.data[idx] &= !0x10;
	}
}

// No point in having a secondary screen buffer
// since the GBA renders in scan lines anyway.
pub type GbaLcdScreenBuffer = Vec<GbaLcdLine>;

#[derive(Default)]
struct BlendingParams {
	target_drawn: bool,
	source_on_top: bool,
	target_overwritten: bool,
	source_pixel: GbaPixel,
	target_pixel: GbaPixel,
	force_obj_blend: bool,
	current_window_prio: u8,
	window_blending_disabled: bool
}

impl BlendingParams {
	fn reset_for_window(&mut self, window_prio: u8, window_blending_disabled: bool) {
		self.target_drawn = false;
		self.source_on_top = false;
		self.target_overwritten = false;
		self.force_obj_blend = false;
		self.current_window_prio = window_prio;
		self.window_blending_disabled = window_blending_disabled;
	}
}

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

		let pixel_brightness_fn: fn(u16, OutputPixel) -> OutputPixel = match blend_mode {
			2 => brighten_pixel,
			3 => darken_pixel,
			_ => pixel_lum_nop
		};

		let mut blending_params: BlendingParams = Default::default();

		let change_pixel_brightness = |blend_evy: u16, color: OutputPixel, blending_params: &mut BlendingParams| -> OutputPixel {
			if !blending_params.window_blending_disabled {
				return pixel_brightness_fn(blend_evy, color);
			} else {
				return color
			}
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

		let win0_in = winin & 0x1f;
		let win1_in = (winin >> 8) & 0x1f;
		let winout_in = winout & 0x1f;
		let win_obj_in = (winout >> 8) & 0x1f;

		let on_pixel_drawn = |layer_idx: u16, color: GbaPixel, force_source: bool, blending_params: &mut BlendingParams| {
			if !is_transparent(color) && !blending_params.window_blending_disabled {
				if force_source || ((blend_sources >> layer_idx) & 1) == 1 { // This is a source layer.
					blending_params.source_on_top = true;
					blending_params.source_pixel = color;
				} else if ((blend_targets >> layer_idx) & 1) == 1 { // This is a target layer.
					if blending_params.target_drawn {
						blending_params.target_overwritten = true;
					} else {
						blending_params.target_drawn = true;
						blending_params.target_pixel = color;
					}
					blending_params.source_on_top = false;
				} else {
					blending_params.source_on_top = false;
					blending_params.target_overwritten = true;
				}
			}
		};

		on_pixel_drawn(5, backdrop, false, &mut blending_params);
		let darkened_backdrop = backdrop;

		// I'll have borrowing issues if I don't define this here like so:
		let obj_info = &self.lines.obj_info;

		let window_clip_pixel = |line: u16, column: u16, _: GbaPixel, layer_idx: u16, dest_pixel: &mut GbaPixel, blending_params: &mut BlendingParams| -> bool {
			if !win0_enabled && !win1_enabled && !win_obj_enabled { // Windowing is turned off.
				return true;
			}
			let pwindow_priority;
			let window_disables_blending;
			if win0_enabled && window_contains(column, line, win0_left, win0_right, win0_top, win0_bottom) {
				pwindow_priority = 4;
				window_disables_blending = ((winin >> 5) & 1) == 0;
				if ((win0_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending); // #FIXME do I need to be doing this?
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			} else if win1_enabled && window_contains(column, line, win1_left, win1_right, win1_top, win1_bottom) {
				pwindow_priority = 3;
				window_disables_blending = ((winin >> 13) & 1) == 0;
				if ((win1_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending);
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			} else if obj_info.is_window(column as usize) {
				pwindow_priority = 2;
				window_disables_blending = ((winout >> 13) & 1) == 0;
				if ((win_obj_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending);
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			} else {
				pwindow_priority = 1;
				window_disables_blending = ((winout >> 5) & 1) == 0;
				if ((winout_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending);
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			}

			if pwindow_priority < blending_params.current_window_prio {
				return false
			} else if pwindow_priority > blending_params.current_window_prio {
				blending_params.reset_for_window(pwindow_priority, window_disables_blending);
				// we basically act like the backdrop has been drawn again.
				*dest_pixel = darkened_backdrop;
				on_pixel_drawn(5, backdrop, false, blending_params);
			}
			return true
		};

		for pix in 0..240 {
			let mut output_pixel = darkened_backdrop;

			for priority in (0..4).rev() {
				if bg3_enabled && bg3_priority == priority {
					let src_pixel = self.lines.bg3[pix];
					if window_clip_pixel(line, pix as u16, src_pixel, 3, &mut output_pixel, &mut blending_params) {
						on_pixel_drawn(3, src_pixel, false, &mut blending_params);
						output_pixel = Self::blend_pixels(src_pixel, output_pixel);
					}
				}

				if bg2_enabled && bg2_priority == priority {
					let src_pixel = self.lines.bg2[pix];
					if window_clip_pixel(line, pix as u16, src_pixel, 2, &mut output_pixel, &mut blending_params) {
						on_pixel_drawn(2, src_pixel, false, &mut blending_params);
						output_pixel = Self::blend_pixels(src_pixel, output_pixel);
					}
				}

				if bg1_enabled && bg1_priority == priority {
					let src_pixel = self.lines.bg1[pix];
					if window_clip_pixel(line, pix as u16, src_pixel, 1, &mut output_pixel, &mut blending_params) {
						on_pixel_drawn(1, src_pixel, false, &mut blending_params);
						output_pixel = Self::blend_pixels(src_pixel, output_pixel);
					}
				}

				if bg0_enabled && bg0_priority == priority {
					let src_pixel = self.lines.bg0[pix];
					if window_clip_pixel(line, pix as u16, src_pixel, 0, &mut output_pixel, &mut blending_params) {
						on_pixel_drawn(0, src_pixel, false, &mut blending_params);
						output_pixel = Self::blend_pixels(src_pixel, output_pixel);
					}
				}

				let obj_priority = self.lines.obj_info.get_priority(pix);
				if obj_enabled && obj_priority > 0 && (obj_priority - 1) == (priority as u8) {
					let obj_pixel = self.lines.obj[pix];
					if window_clip_pixel(line, pix as u16, obj_pixel, 4, &mut output_pixel, &mut blending_params) {
						on_pixel_drawn(4, obj_pixel, self.lines.obj_info.is_transparent(pix), &mut blending_params);
						output_pixel = Self::blend_pixels(obj_pixel, output_pixel);
						blending_params.force_obj_blend |= self.lines.obj_info.is_transparent(pix);
					}
				}
			}

			if !blending_params.window_blending_disabled && (blend_mode == 1 || (blending_params.force_obj_blend && blend_mode > 0)) && (blending_params.target_drawn && !blending_params.target_overwritten && blending_params.source_on_top) {
				let (t_r, t_g, t_b) = expand_color(blending_params.target_pixel);
				let (s_r, s_g, s_b) = expand_color(blending_params.source_pixel);

				// I = MIN ( 31, I1st*EVA + I2nd*EVB )
				output[pix] = (
					min!(255, (((s_r as u16) * blend_eva) >> 4) + (((t_r as u16) * blend_evb) >> 4)) as u8,
					min!(255, (((s_g as u16) * blend_eva) >> 4) + (((t_g as u16) * blend_evb) >> 4)) as u8,
					min!(255, (((s_b as u16) * blend_eva) >> 4) + (((t_b as u16) * blend_evb) >> 4)) as u8,
				);
			} else {
				let expanded = expand_color(output_pixel); 
				if blending_params.source_on_top {
					output[pix] = change_pixel_brightness(blend_evy, expanded, &mut blending_params);
				} else {
					output[pix] = expanded;
				}
			}

			blending_params.target_drawn = false;
			blending_params.source_on_top = false;
			blending_params.target_overwritten = false;
			blending_params.force_obj_blend = false;
			blending_params.window_blending_disabled = false;
			blending_params.current_window_prio = 0; // reset it to winout.
		}
	}

	#[inline(always)]
	fn blend_pixels(a: GbaPixel, b: GbaPixel) -> GbaPixel {
		if is_transparent(a) { b }
		else { a }
	}
}

pub fn expand_color(rgb5: u16) -> (u8, u8, u8) {
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


// PIXEL BRIGHTNESS FUNCTIONS:

/// I1st + (31-I1st)*EVY
fn brighten_pixel(blend_evy: u16, color: OutputPixel) -> OutputPixel {
	(
		((color.0 as u16) + (((255 - (color.0 as u16)) * blend_evy) >> 4)) as u8,
		((color.1 as u16) + (((255 - (color.1 as u16)) * blend_evy) >> 4)) as u8,
		((color.2 as u16) + (((255 - (color.2 as u16)) * blend_evy) >> 4)) as u8
	)
}

/// I1st - (I1st)*EVY
fn darken_pixel(blend_evy: u16, color: OutputPixel) -> OutputPixel {
	(
		((color.0 as u16) - (((color.0 as u16) * blend_evy) >> 4)) as u8,
		((color.1 as u16) - (((color.1 as u16) * blend_evy) >> 4)) as u8,
		((color.2 as u16) - (((color.2 as u16) * blend_evy) >> 4)) as u8
	)
}

/// Just does nothing to the pixel
fn pixel_lum_nop(_: u16, color: OutputPixel) -> OutputPixel {
	color
}
