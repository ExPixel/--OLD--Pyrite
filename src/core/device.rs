use sdl2;
use sdl2::render::Renderer;
// use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use super::lcd::GbaLcdLine;

use ::util::frame_counter::FrameCounter;

pub struct GbaDevice<'a> {
	pub context: Sdl,
	pub event_pump: EventPump,
	pub renderer: Renderer<'a>,
	pub gba_screen: Texture,
	pub fps_counter: FrameCounter
}

impl<'a> GbaDevice<'a> {
	pub fn new<'b>() -> GbaDevice<'b> {
		let sdl_context = sdl2::init().expect("Failed to initialize SDL2.");
		let video_subsystem = sdl_context.video().expect("Failed to initialize SDL2 video.");
		
		let window = video_subsystem.window("Pyrite", 800, 600)
				.position_centered()
				.opengl()
				.build()
				.expect("Failed to create a window.");

		// let ctx = window.gl_create_context().unwrap();
		// window.gl_make_current(&ctx).expect("Failed to make the window the current GL context.");
		video_subsystem.gl_set_swap_interval(1);

		let renderer = window.renderer().build().expect("Failed to create a renderer.");

		let texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (240, 160)).unwrap();
		let event_pump = sdl_context.event_pump().expect("Failed to create event pump.");


		GbaDevice {
			context: sdl_context,
			event_pump: event_pump,
			renderer: renderer,
			gba_screen: texture,
			fps_counter: FrameCounter::new()
		}
	}

	#[allow(unused_variables)]
	pub fn render_line(&mut self, line: usize, line_data: &GbaLcdLine) {
	}

	/// Renders the screen texture.
	pub fn render(&mut self) {
		self.renderer.clear();
		self.renderer.copy(&self.gba_screen, None, Some(Rect::new_unwrap(0, 0, 800, 600)));
		self.renderer.present();
		self.fps_counter.record_frame();
	}
}
