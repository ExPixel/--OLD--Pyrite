use super::memory::GbaMemory;

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

pub type GbaLcdLine = [u32; 240];

// No point in having a secondary screen buffer
// since the GBA renders in scan lines anyway.
// pub type GbaLcdScreenBuffer = [GbaLcdLine; 160];

pub struct GbaLcd {
	pub line: GbaLcdLine
}

impl GbaLcd {
	pub fn new() -> GbaLcd {
		GbaLcd {
			line: [0u32; 240]
		}
	}

	/// Causes the LCD to render a single line.
	#[allow(unused_variables)] // #TODO remove this
	pub fn render_line(&mut self, memory: &mut GbaMemory) {
	}
}