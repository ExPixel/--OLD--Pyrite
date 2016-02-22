use glium;
use glium::texture::Texture2d;
use glium::Surface;
use glium::texture::UncompressedFloatFormat;
use glium::texture::MipmapsOption;

use super::hw::lcd::GbaLcdScreenBuffer;
use ::util::frame_counter::FrameCounter;

const GBA_SCREEN_WIDTH: u32 = 240;
const GBA_SCREEN_HEIGHT: u32 = 160;

#[derive(Copy, Clone)]
struct Vertex { position: [f32; 2], tex_coords: [f32; 2] }
implement_vertex!(Vertex, position, tex_coords);

static VERTEX_SHADER_SRC: &'static str = r#"
#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER_SRC: &'static str = r#"
#version 140

// The Gamma of the GBA screen.
#define GAMMA 2.2

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
	vec4 tcolor = texture(tex, v_tex_coords);
	tcolor.rgb = pow(tcolor.rgb, vec3(GAMMA));
    color = tcolor;
}
"#;

const DEFAULT_SCALE: u32 = 1;

pub struct GbaDevice {
	pub display: glium::backend::glutin_backend::GlutinFacade, // #TODO wtf
	vertex_buffer: glium::vertex::VertexBuffer<Vertex>,
	indices: glium::index::NoIndices,
	program: glium::program::Program,
	pub screen_texture: Texture2d,
	pub fps_counter: FrameCounter,
}

impl GbaDevice {
	pub fn new() -> GbaDevice {
		use glium::DisplayBuild;
		let display = glium::glutin::WindowBuilder::new()
						.with_dimensions(GBA_SCREEN_WIDTH * DEFAULT_SCALE, GBA_SCREEN_HEIGHT * DEFAULT_SCALE)
						.build_glium().unwrap();

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
		let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

		let texture = Texture2d::empty_with_format(&display,
						UncompressedFloatFormat::U8U8U8, MipmapsOption::EmptyMipmaps,
						GBA_SCREEN_WIDTH, GBA_SCREEN_HEIGHT).expect("Failed to create screen texture.");

		GbaDevice {
			display: display,
			vertex_buffer: vertex_buffer,
			indices: indices,
			program: program,
			screen_texture: texture,
			fps_counter: FrameCounter::new()
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
		self.fps_counter.record_frame();
	}
}
