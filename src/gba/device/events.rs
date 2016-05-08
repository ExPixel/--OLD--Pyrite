use glium::glutin::{Event, ElementState, VirtualKeyCode};
use super::super::Gba;
use super::super::GbaEventPoll;
use super::super::INT_KEYPAD;

impl GbaEventPoll for Gba {
	fn poll_device_events(&mut self) {
		for event in self.device.video.display.poll_events() {
			match event {
				Event::Closed => self.request_exit = true,
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
					self.request_exit = true
				},

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
				
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::H)) => { // The '>' key for me.
					self.extras.request_debugger = true;
					println!("Request Debugger...");
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

		if self.joypad.tick(&mut self.cpu) {
			self.hardware_interrupt(INT_KEYPAD);
		}
	}
}