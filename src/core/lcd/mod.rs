use super::memory::GbaMemory;

use sdl2::render::Texture;

pub mod mode0;
pub mod mode1;
pub mod mode2;
pub mod mode3;
pub mod mode4;
pub mod mode5;

pub type GbaLcdLine = [u32; 240];

// No point in having a secondary screen buffer
// since the GBA renders in scan lines anyway.
// pub type GbaLcdScreenBuffer = [GbaLcdLine; 160];

pub struct GbaLcd;

impl GbaLcd {
	pub fn new() -> GbaLcd {
		GbaLcd
	}

	/// Causes the LCD to render a single line.
	pub fn render_line(&mut self, screen: &mut Texture, memory: &mut GbaMemory) {
		self.render_to_screen();
	}

	/// Renders the current screen buffer.
	fn render_to_screen(&mut self) {
	}
}