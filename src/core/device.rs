use sdl2;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;

pub struct GbaDevice<'a> {
	context: Option<Sdl>,
	event_pump: Option<EventPump>,
	renderer: Option<Renderer<'a>>,
	pub screen: Option<Texture>
}

impl<'a> GbaDevice<'a> {
	pub fn new<'b>() -> GbaDevice<'b> {
		GbaDevice {
			// window: None,
			context: None,
			event_pump: None,
			renderer: None,
			screen: None
		}
	}

	/// Initializes the LCD.
	pub fn init(&mut self) {
		let sdl_context = sdl2::init().expect("Failed to initialize SDL2.");
		let video_subsystem = sdl_context.video().expect("Failed to initialize SDL2 video.");
		let window = video_subsystem.window("Pyrite", 800, 600)
				.position_centered()
				.opengl()
				.build()
				.expect("Failed to create a window.");
		let renderer = window.renderer().build().expect("Failed to create a renderer.");

		let texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (240, 160)).unwrap();
		self.screen = Some(texture);

		self.renderer = Some(renderer);
		let event_pump = sdl_context.event_pump().expect("Failed to create event pump.");
		self.event_pump = Some(event_pump);
		self.context = Some(sdl_context);
	}
}

