use super::memory::*;

// #TODO remove these allows
#[allow(unused_variables)]
pub mod mode0;
#[allow(unused_variables)]
pub mod mode1;
#[allow(unused_variables)]
pub mod mode2;
#[allow(unused_variables)]
pub mod mode3;
#[allow(unused_variables)]
pub mod mode4;
#[allow(unused_variables)]
pub mod mode5;

pub type GbaPixel = (u8, u8, u8);
pub type GbaLcdLine = Vec<GbaPixel>;

// No point in having a secondary screen buffer
// since the GBA renders in scan lines anyway.
pub type GbaLcdScreenBuffer = Vec<GbaLcdLine>;

pub struct GbaLcd {
	pub screen_buffer: GbaLcdScreenBuffer
}

impl GbaLcd {
	pub fn new() -> GbaLcd {
		GbaLcd {
			screen_buffer: vec![vec![(0u8, 0u8, 0u8); 240]; 160]
		}
	}

	/// Causes the LCD to render a single line.
	#[allow(unused_variables)] // #TODO remove this
	pub fn render_line(&mut self, memory: &mut GbaMemory, line: u16) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);

		match dispcnt & 0x7 {
			0 => mode0::render_mode_0(dispcnt, memory, line, &mut self.screen_buffer[line as usize].as_mut_slice()),
			1 => mode1::render_mode_1(dispcnt, memory, line, &mut self.screen_buffer[line as usize].as_mut_slice()),
			2 => mode2::render_mode_2(dispcnt, memory, line, &mut self.screen_buffer[line as usize].as_mut_slice()),
			3 => mode3::render_mode_3(dispcnt, memory, line, &mut self.screen_buffer[line as usize].as_mut_slice()),
			4 => mode4::render_mode_4(dispcnt, memory, line, &mut self.screen_buffer[line as usize].as_mut_slice()),
			5 => mode5::render_mode_5(dispcnt, memory, line, &mut self.screen_buffer[line as usize].as_mut_slice()),
			_ => unreachable!()
		}
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
