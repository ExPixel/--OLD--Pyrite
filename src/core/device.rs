use sdl2;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;

pub struct GbaDevice<'a> {
	pub context: Sdl,
	pub event_pump: EventPump,
	pub renderer: Renderer<'a>,
	pub screen: Texture
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
		let renderer = window.renderer().build().expect("Failed to create a renderer.");

		let texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (240, 160)).unwrap();
		let event_pump = sdl_context.event_pump().expect("Failed to create event pump.");
		
		GbaDevice {
			context: sdl_context,
			event_pump: event_pump,
			renderer: renderer,
			screen: texture
		}
	}
}

