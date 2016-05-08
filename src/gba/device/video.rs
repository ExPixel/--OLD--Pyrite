use glium;
use glium::texture::Texture2d;
use glium::Surface;
use glium::texture::UncompressedFloatFormat;
use glium::texture::MipmapsOption;

use super::super::hw::lcd::GbaLcdScreenBuffer;
// use ::util::frame_counter::FrameCounter;

const ENABLE_VSYNC: bool = true;

const GBA_SCREEN_WIDTH: u32 = 240;
const GBA_SCREEN_HEIGHT: u32 = 160;

#[derive(Copy, Clone)]
struct Vertex { position: [f32; 2], tex_coords: [f32; 2] }
implement_vertex!(Vertex, position, tex_coords);

const DEFAULT_SCALE: u32 = 1;

pub struct VideoDevice {
	pub display: glium::backend::glutin_backend::GlutinFacade, // #TODO wtf
	vertex_buffer: glium::vertex::VertexBuffer<Vertex>,
	indices: glium::index::NoIndices,
	program: glium::program::Program,
	pub screen_texture: Texture2d
}

impl VideoDevice {
	pub fn new() -> VideoDevice {
		use glium::DisplayBuild;
		let mut display_builder = glium::glutin::WindowBuilder::new()
						.with_dimensions(GBA_SCREEN_WIDTH * DEFAULT_SCALE, GBA_SCREEN_HEIGHT * DEFAULT_SCALE);
		if ENABLE_VSYNC { display_builder = display_builder.with_vsync(); }
		let display = display_builder.build_glium().unwrap();

		// #TODO fixy fixy
		let vertex_shader_src = ::util::io::read_file_into_string("data/shaders/gba.vert").unwrap();
		let fragment_shader_src = ::util::io::read_file_into_string("data/shaders/gba.frag").unwrap();

		let shape = vec![
			Vertex { position: [1.0, 1.0], tex_coords: [1.0, 0.0] },
			Vertex { position: [-1.0, 1.0], tex_coords: [0.0, 0.0] },
			Vertex { position: [1.0, -1.0], tex_coords: [1.0, 1.0] },
			Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 1.0] },
			Vertex { position: [-1.0, 1.0], tex_coords: [0.0, 0.0] },
			Vertex { position: [1.0, -1.0], tex_coords: [1.0, 1.0] }
		];
		let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

		let texture = Texture2d::empty_with_format(&display,
						UncompressedFloatFormat::U8U8U8, MipmapsOption::EmptyMipmaps,
						GBA_SCREEN_WIDTH, GBA_SCREEN_HEIGHT).expect("Failed to create screen texture.");

		VideoDevice {
			display: display,
			vertex_buffer: vertex_buffer,
			indices: indices,
			program: program,
			screen_texture: texture
		}
	}

	/// Renders the screen texture.
	pub fn render(&mut self, buffer: &GbaLcdScreenBuffer) {
		self.screen_texture.write(
			glium::Rect {
				left: 0, 
				bottom: 0,
				width: GBA_SCREEN_WIDTH, 
				height: GBA_SCREEN_HEIGHT
			},
			buffer.clone() // #TODO ...but why, tomaka...why?
		);
		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		let uniform = uniform! {
			tex: self.screen_texture.sampled()
					.magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
					.minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
		};

		target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniform,
            &Default::default()).unwrap();
		target.finish().unwrap();
	}
}
