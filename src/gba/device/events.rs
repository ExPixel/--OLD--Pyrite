use glutin::{Event, ElementState, VirtualKeyCode};
use super::super::Gba;
use super::super::GbaEventPoll;
use super::super::INT_KEYPAD;
use super::imgui_support;
use rust_imgui as imgui;

impl GbaEventPoll for Gba {
	fn poll_device_events(&mut self) {
		for event in self.device.video.display.poll_events() {
			match event {
				Event::Closed => self.request_exit = true,
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
					self.request_exit = true
				},
				_ => {}
			}

			imgui_support::imgui_check_event(&mut self.device.video.im_support, &event);
			let io = imgui::get_io();
			let imgui_not_capturing = (io.want_text_input | io.want_capture_keyboard) == 0;

			if imgui_not_capturing {
				match event {
				// GENERAL DEBUGGING STUFF:
					Event::KeyboardInput(state, _, Some(VirtualKeyCode::D)) => {
						match state {
							ElementState::Pressed => set_pyrite_dyn_debug!(true),
							_ => set_pyrite_dyn_debug!(false)
						}
					},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::R)) => {
						self.cpu.reg_dump_pretty();
					},

					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::P)) => {
						self.extras.request_pause = !self.extras.paused;
					},

				// DEBUGGING LAYERS IN GRAPHICS:
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key1)) => {debug_toggle_layer!(0);},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key2)) => {debug_toggle_layer!(1);},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key3)) => {debug_toggle_layer!(2);},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key4)) => {debug_toggle_layer!(3);},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Key5)) => {debug_toggle_layer!(4);},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::L)) => {debug_turn_off_all_layers!();},
					Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::K)) => {debug_turn_on_all_layers!();},

				// ACTUAL GBA SHIT:
					Event::KeyboardInput(ElementState::Pressed, _, Some(keycode)) => {
						self.joypad.key_pressed(keycode);
					},
					Event::KeyboardInput(ElementState::Released, _, Some(keycode)) => {
						self.joypad.key_released(keycode);
					},
					_ => {}
				}
			}
		}

		if self.joypad.tick(&mut self.cpu) {
			self.hardware_interrupt(INT_KEYPAD);
		}

	}
}

pub fn idx_to_vkc(idx: usize) -> VirtualKeyCode {
	match idx {
		0 => VirtualKeyCode::Key1,
		1 => VirtualKeyCode::Key2,
		2 => VirtualKeyCode::Key3,
		3 => VirtualKeyCode::Key4,
		4 => VirtualKeyCode::Key5,
		5 => VirtualKeyCode::Key6,
		6 => VirtualKeyCode::Key7,
		7 => VirtualKeyCode::Key8,
		8 => VirtualKeyCode::Key9,
		9 => VirtualKeyCode::Key0,
		10 => VirtualKeyCode::A,
		11 => VirtualKeyCode::B,
		12 => VirtualKeyCode::C,
		13 => VirtualKeyCode::D,
		14 => VirtualKeyCode::E,
		15 => VirtualKeyCode::F,
		16 => VirtualKeyCode::G,
		17 => VirtualKeyCode::H,
		18 => VirtualKeyCode::I,
		19 => VirtualKeyCode::J,
		20 => VirtualKeyCode::K,
		21 => VirtualKeyCode::L,
		22 => VirtualKeyCode::M,
		23 => VirtualKeyCode::N,
		24 => VirtualKeyCode::O,
		25 => VirtualKeyCode::P,
		26 => VirtualKeyCode::Q,
		27 => VirtualKeyCode::R,
		28 => VirtualKeyCode::S,
		29 => VirtualKeyCode::T,
		30 => VirtualKeyCode::U,
		31 => VirtualKeyCode::V,
		32 => VirtualKeyCode::W,
		33 => VirtualKeyCode::X,
		34 => VirtualKeyCode::Y,
		35 => VirtualKeyCode::Z,
		36 => VirtualKeyCode::Escape,
		37 => VirtualKeyCode::F1,
		38 => VirtualKeyCode::F2,
		39 => VirtualKeyCode::F3,
		40 => VirtualKeyCode::F4,
		41 => VirtualKeyCode::F5,
		42 => VirtualKeyCode::F6,
		43 => VirtualKeyCode::F7,
		44 => VirtualKeyCode::F8,
		45 => VirtualKeyCode::F9,
		46 => VirtualKeyCode::F10,
		47 => VirtualKeyCode::F11,
		48 => VirtualKeyCode::F12,
		49 => VirtualKeyCode::F13,
		50 => VirtualKeyCode::F14,
		51 => VirtualKeyCode::F15,
		52 => VirtualKeyCode::Snapshot,
		53 => VirtualKeyCode::Scroll,
		54 => VirtualKeyCode::Pause,
		55 => VirtualKeyCode::Insert,
		56 => VirtualKeyCode::Home,
		57 => VirtualKeyCode::Delete,
		58 => VirtualKeyCode::End,
		59 => VirtualKeyCode::PageDown,
		60 => VirtualKeyCode::PageUp,
		61 => VirtualKeyCode::Left,
		62 => VirtualKeyCode::Up,
		63 => VirtualKeyCode::Right,
		64 => VirtualKeyCode::Down,
		65 => VirtualKeyCode::Back,
		66 => VirtualKeyCode::Return,
		67 => VirtualKeyCode::Space,
		68 => VirtualKeyCode::Numlock,
		69 => VirtualKeyCode::Numpad0,
		70 => VirtualKeyCode::Numpad1,
		71 => VirtualKeyCode::Numpad2,
		72 => VirtualKeyCode::Numpad3,
		73 => VirtualKeyCode::Numpad4,
		74 => VirtualKeyCode::Numpad5,
		75 => VirtualKeyCode::Numpad6,
		76 => VirtualKeyCode::Numpad7,
		77 => VirtualKeyCode::Numpad8,
		78 => VirtualKeyCode::Numpad9,
		79 => VirtualKeyCode::AbntC1,
		80 => VirtualKeyCode::AbntC2,
		81 => VirtualKeyCode::Add,
		82 => VirtualKeyCode::Apostrophe,
		83 => VirtualKeyCode::Apps,
		84 => VirtualKeyCode::At,
		85 => VirtualKeyCode::Ax,
		86 => VirtualKeyCode::Backslash,
		87 => VirtualKeyCode::Calculator,
		88 => VirtualKeyCode::Capital,
		89 => VirtualKeyCode::Colon,
		90 => VirtualKeyCode::Comma,
		91 => VirtualKeyCode::Convert,
		92 => VirtualKeyCode::Decimal,
		93 => VirtualKeyCode::Divide,
		94 => VirtualKeyCode::Equals,
		95 => VirtualKeyCode::Grave,
		96 => VirtualKeyCode::Kana,
		97 => VirtualKeyCode::Kanji,
		98 => VirtualKeyCode::LAlt,
		99 => VirtualKeyCode::LBracket,
		100 => VirtualKeyCode::LControl,
		101 => VirtualKeyCode::LMenu,
		102 => VirtualKeyCode::LShift,
		103 => VirtualKeyCode::LWin,
		104 => VirtualKeyCode::Mail,
		105 => VirtualKeyCode::MediaSelect,
		106 => VirtualKeyCode::MediaStop,
		107 => VirtualKeyCode::Minus,
		108 => VirtualKeyCode::Multiply,
		109 => VirtualKeyCode::Mute,
		110 => VirtualKeyCode::MyComputer,
		111 => VirtualKeyCode::NavigateForward,
		112 => VirtualKeyCode::NavigateBackward,
		113 => VirtualKeyCode::NextTrack,
		114 => VirtualKeyCode::NoConvert,
		115 => VirtualKeyCode::NumpadComma,
		116 => VirtualKeyCode::NumpadEnter,
		117 => VirtualKeyCode::NumpadEquals,
		118 => VirtualKeyCode::OEM102,
		119 => VirtualKeyCode::Period,
		120 => VirtualKeyCode::PlayPause,
		121 => VirtualKeyCode::Power,
		122 => VirtualKeyCode::PrevTrack,
		123 => VirtualKeyCode::RAlt,
		124 => VirtualKeyCode::RBracket,
		125 => VirtualKeyCode::RControl,
		126 => VirtualKeyCode::RMenu,
		127 => VirtualKeyCode::RShift,
		128 => VirtualKeyCode::RWin,
		129 => VirtualKeyCode::Semicolon,
		130 => VirtualKeyCode::Slash,
		131 => VirtualKeyCode::Sleep,
		132 => VirtualKeyCode::Stop,
		133 => VirtualKeyCode::Subtract,
		134 => VirtualKeyCode::Sysrq,
		135 => VirtualKeyCode::Tab,
		136 => VirtualKeyCode::Underline,
		137 => VirtualKeyCode::Unlabeled,
		138 => VirtualKeyCode::VolumeDown,
		139 => VirtualKeyCode::VolumeUp,
		140 => VirtualKeyCode::Wake,
		141 => VirtualKeyCode::WebBack,
		142 => VirtualKeyCode::WebFavorites,
		143 => VirtualKeyCode::WebForward,
		144 => VirtualKeyCode::WebHome,
		145 => VirtualKeyCode::WebRefresh,
		146 => VirtualKeyCode::WebSearch,
		147 => VirtualKeyCode::WebStop,
		148 => VirtualKeyCode::Yen,
		_ => panic!("Key index {} not supported!", idx)
	}
}

pub fn vkc_to_idx(code: VirtualKeyCode) -> usize {
	match code {
		VirtualKeyCode::Key1 => 0,
		VirtualKeyCode::Key2 => 1,
		VirtualKeyCode::Key3 => 2,
		VirtualKeyCode::Key4 => 3,
		VirtualKeyCode::Key5 => 4,
		VirtualKeyCode::Key6 => 5,
		VirtualKeyCode::Key7 => 6,
		VirtualKeyCode::Key8 => 7,
		VirtualKeyCode::Key9 => 8,
		VirtualKeyCode::Key0 => 9,
		VirtualKeyCode::A => 10,
		VirtualKeyCode::B => 11,
		VirtualKeyCode::C => 12,
		VirtualKeyCode::D => 13,
		VirtualKeyCode::E => 14,
		VirtualKeyCode::F => 15,
		VirtualKeyCode::G => 16,
		VirtualKeyCode::H => 17,
		VirtualKeyCode::I => 18,
		VirtualKeyCode::J => 19,
		VirtualKeyCode::K => 20,
		VirtualKeyCode::L => 21,
		VirtualKeyCode::M => 22,
		VirtualKeyCode::N => 23,
		VirtualKeyCode::O => 24,
		VirtualKeyCode::P => 25,
		VirtualKeyCode::Q => 26,
		VirtualKeyCode::R => 27,
		VirtualKeyCode::S => 28,
		VirtualKeyCode::T => 29,
		VirtualKeyCode::U => 30,
		VirtualKeyCode::V => 31,
		VirtualKeyCode::W => 32,
		VirtualKeyCode::X => 33,
		VirtualKeyCode::Y => 34,
		VirtualKeyCode::Z => 35,
		VirtualKeyCode::Escape => 36,
		VirtualKeyCode::F1 => 37,
		VirtualKeyCode::F2 => 38,
		VirtualKeyCode::F3 => 39,
		VirtualKeyCode::F4 => 40,
		VirtualKeyCode::F5 => 41,
		VirtualKeyCode::F6 => 42,
		VirtualKeyCode::F7 => 43,
		VirtualKeyCode::F8 => 44,
		VirtualKeyCode::F9 => 45,
		VirtualKeyCode::F10 => 46,
		VirtualKeyCode::F11 => 47,
		VirtualKeyCode::F12 => 48,
		VirtualKeyCode::F13 => 49,
		VirtualKeyCode::F14 => 50,
		VirtualKeyCode::F15 => 51,
		VirtualKeyCode::Snapshot => 52,
		VirtualKeyCode::Scroll => 53,
		VirtualKeyCode::Pause => 54,
		VirtualKeyCode::Insert => 55,
		VirtualKeyCode::Home => 56,
		VirtualKeyCode::Delete => 57,
		VirtualKeyCode::End => 58,
		VirtualKeyCode::PageDown => 59,
		VirtualKeyCode::PageUp => 60,
		VirtualKeyCode::Left => 61,
		VirtualKeyCode::Up => 62,
		VirtualKeyCode::Right => 63,
		VirtualKeyCode::Down => 64,
		VirtualKeyCode::Back => 65,
		VirtualKeyCode::Return => 66,
		VirtualKeyCode::Space => 67,
		VirtualKeyCode::Numlock => 68,
		VirtualKeyCode::Numpad0 => 69,
		VirtualKeyCode::Numpad1 => 70,
		VirtualKeyCode::Numpad2 => 71,
		VirtualKeyCode::Numpad3 => 72,
		VirtualKeyCode::Numpad4 => 73,
		VirtualKeyCode::Numpad5 => 74,
		VirtualKeyCode::Numpad6 => 75,
		VirtualKeyCode::Numpad7 => 76,
		VirtualKeyCode::Numpad8 => 77,
		VirtualKeyCode::Numpad9 => 78,
		VirtualKeyCode::AbntC1 => 79,
		VirtualKeyCode::AbntC2 => 80,
		VirtualKeyCode::Add => 81,
		VirtualKeyCode::Apostrophe => 82,
		VirtualKeyCode::Apps => 83,
		VirtualKeyCode::At => 84,
		VirtualKeyCode::Ax => 85,
		VirtualKeyCode::Backslash => 86,
		VirtualKeyCode::Calculator => 87,
		VirtualKeyCode::Capital => 88,
		VirtualKeyCode::Colon => 89,
		VirtualKeyCode::Comma => 90,
		VirtualKeyCode::Convert => 91,
		VirtualKeyCode::Decimal => 92,
		VirtualKeyCode::Divide => 93,
		VirtualKeyCode::Equals => 94,
		VirtualKeyCode::Grave => 95,
		VirtualKeyCode::Kana => 96,
		VirtualKeyCode::Kanji => 97,
		VirtualKeyCode::LAlt => 98,
		VirtualKeyCode::LBracket => 99,
		VirtualKeyCode::LControl => 100,
		VirtualKeyCode::LMenu => 101,
		VirtualKeyCode::LShift => 102,
		VirtualKeyCode::LWin => 103,
		VirtualKeyCode::Mail => 104,
		VirtualKeyCode::MediaSelect => 105,
		VirtualKeyCode::MediaStop => 106,
		VirtualKeyCode::Minus => 107,
		VirtualKeyCode::Multiply => 108,
		VirtualKeyCode::Mute => 109,
		VirtualKeyCode::MyComputer => 110,
		VirtualKeyCode::NavigateForward => 111,
		VirtualKeyCode::NavigateBackward => 112,
		VirtualKeyCode::NextTrack => 113,
		VirtualKeyCode::NoConvert => 114,
		VirtualKeyCode::NumpadComma => 115,
		VirtualKeyCode::NumpadEnter => 116,
		VirtualKeyCode::NumpadEquals => 117,
		VirtualKeyCode::OEM102 => 118,
		VirtualKeyCode::Period => 119,
		VirtualKeyCode::PlayPause => 120,
		VirtualKeyCode::Power => 121,
		VirtualKeyCode::PrevTrack => 122,
		VirtualKeyCode::RAlt => 123,
		VirtualKeyCode::RBracket => 124,
		VirtualKeyCode::RControl => 125,
		VirtualKeyCode::RMenu => 126,
		VirtualKeyCode::RShift => 127,
		VirtualKeyCode::RWin => 128,
		VirtualKeyCode::Semicolon => 129,
		VirtualKeyCode::Slash => 130,
		VirtualKeyCode::Sleep => 131,
		VirtualKeyCode::Stop => 132,
		VirtualKeyCode::Subtract => 133,
		VirtualKeyCode::Sysrq => 134,
		VirtualKeyCode::Tab => 135,
		VirtualKeyCode::Underline => 136,
		VirtualKeyCode::Unlabeled => 137,
		VirtualKeyCode::VolumeDown => 138,
		VirtualKeyCode::VolumeUp => 139,
		VirtualKeyCode::Wake => 140,
		VirtualKeyCode::WebBack => 141,
		VirtualKeyCode::WebFavorites => 142,
		VirtualKeyCode::WebForward => 143,
		VirtualKeyCode::WebHome => 144,
		VirtualKeyCode::WebRefresh => 145,
		VirtualKeyCode::WebSearch => 146,
		VirtualKeyCode::WebStop => 147,
		VirtualKeyCode::Yen => 148,
	}
}