use super::super::core::cpu::ArmCpu;
use super::super::core::memory::*;
use glium::glutin::VirtualKeyCode;

pub struct GbaJoypad {
	key_input: u16,
	dirty: bool
}

// 4000130h - KEYINPUT - Key Status (R)
//   Bit   Expl.
//   0     Button A        (0=Pressed, 1=Released)
//   1     Button B        (etc.)
//   2     Select          (etc.)
//   3     Start           (etc.)
//   4     Right           (etc.)
//   5     Left            (etc.)
//   6     Up              (etc.)
//   7     Down            (etc.)
//   8     Button R        (etc.)
//   9     Button L        (etc.)
//   10-15 Not used

const GBA_BTN_A: u16 = 0x1;
const GBA_BTN_B: u16 = 0x2;
const GBA_BTN_SELECT: u16 = 0x4;
const GBA_BTN_START: u16 = 0x8;
const GBA_BTN_RIGHT: u16 = 0x10;
const GBA_BTN_LEFT: u16 = 0x20; 
const GBA_BTN_UP: u16 = 0x40;
const GBA_BTN_DOWN: u16 = 0x80;
const GBA_BTN_R: u16 = 0x100;
const GBA_BTN_L: u16 = 0x200;


impl GbaJoypad {
	pub fn new() -> GbaJoypad {
		GbaJoypad {
			key_input: 0xffff,
			dirty: true
		}
	}

	pub fn tick(&mut self, cpu: &mut ArmCpu) {
		if self.dirty {
			cpu.memory.set_reg(ioreg::KEYINPUT, self.key_input);
			self.dirty = false;
			// #TODO check for interrupt here and return true if there is one.
		}
	}

	pub fn key_pressed(&mut self, keycode: VirtualKeyCode) {
		if let Some(mask) = self.map_keycode_to_mask(keycode) {
			self.set_button_pressed(mask);
		}
	}

	pub fn key_released(&mut self, keycode: VirtualKeyCode) {
		if let Some(mask) = self.map_keycode_to_mask(keycode) {
			self.set_button_released(mask);
		}
	}

	pub fn map_keycode_to_mask(&self, keycode: VirtualKeyCode) -> Option<u16> {
		match keycode {
			VirtualKeyCode::Z => Some(GBA_BTN_A),
			VirtualKeyCode::X => Some(GBA_BTN_B),
			VirtualKeyCode::Back => Some(GBA_BTN_SELECT),
			VirtualKeyCode::Return => Some(GBA_BTN_START),
			VirtualKeyCode::Right => Some(GBA_BTN_RIGHT),
			VirtualKeyCode::Left => Some(GBA_BTN_LEFT),
			VirtualKeyCode::Up => Some(GBA_BTN_UP),
			VirtualKeyCode::Down => Some(GBA_BTN_DOWN),
			VirtualKeyCode::A => Some(GBA_BTN_L),
			VirtualKeyCode::S => Some(GBA_BTN_R),
			_ => None
		}
	}

	fn set_button_pressed(&mut self, mask: u16) {
		self.key_input &= !mask; // 0 = button pressed
		self.dirty = true;
	}

	fn set_button_released(&mut self, mask: u16) {
		self.key_input |= mask; // 1 = button released
		self.dirty = true;
	}
}