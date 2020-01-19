use glutin;
use gl;
use std::mem;
use std::ptr;
use super::imgui_support;
use super::imgui_support::ImGuiSupport;
use super::super::hw::lcd::GbaLcdScreenBuffer;
// use ::util::frame_counter::FrameCounter;

const ENABLE_VSYNC: bool = true;
const GBA_SCREEN_WIDTH: u32 = 240;
const GBA_SCREEN_HEIGHT: u32 = 160;

macro_rules! gl_check_error {
	() => ({
		__gl_check_error(file!(), line!());
	})
}

pub struct VideoDevice {
	pub display: glutin::Window,
	pub im_support: ImGuiSupport,
	pub gl_vbo_handle: u32,
	pub gl_vao_handle: u32,
	pub gl_ebo_handle: u32,
	pub gl_vert_shader_handle: u32,
	pub gl_frag_shader_handle: u32,
	pub gl_shader_program_handle: u32,
	pub gl_texture_handle: u32,
	pub gl_attrib_location_tex: u32,
}

impl VideoDevice {
	pub fn new() -> VideoDevice {
		let mut builder = glutin::WindowBuilder::new()
			.with_dimensions(GBA_SCREEN_WIDTH * 4, GBA_SCREEN_HEIGHT * 4);

		if ENABLE_VSYNC {
			builder = builder.with_vsync();
		}

		let window = builder.build().expect("Failed to build pyrite window.");
		window.set_title("Pyrite");
		unsafe {
			gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
			window.make_current().expect("Failed to make pyrite window current context.");
		}

		let mut ret = VideoDevice {
			display: window,
			im_support: ImGuiSupport::new(),
			gl_vbo_handle: 0,
			gl_vao_handle: 0,
			gl_ebo_handle: 0,
			gl_vert_shader_handle: 0,
			gl_frag_shader_handle: 0,
			gl_shader_program_handle: 0,
			gl_texture_handle: 0,
			gl_attrib_location_tex: 0
		};
		ret.init();
		return ret;
	}

	pub fn init(&mut self) {

		unsafe {
			let mut vert_shader_handle = 0;
			let vertex_shader_src = ::util::io::read_file_into_string("data/shaders/gba.vert").unwrap();
			compile_shader(&mut vert_shader_handle, gl::VERTEX_SHADER, &vertex_shader_src)
				.expect("Failed to compile pyrite vertex shader.");

			let mut frag_shader_handle = 0;
			let fragment_shader_src = ::util::io::read_file_into_string("data/shaders/gba.frag").unwrap();
			compile_shader(&mut frag_shader_handle, gl::FRAGMENT_SHADER, &fragment_shader_src)
				.expect("Failed to compile pyrite fragment shader.");

			let mut program_handle = 0;
			link_program(&mut program_handle, &mut vert_shader_handle, &mut frag_shader_handle)
				.expect("Failed to link pyrite shader program.");

			gl_check_error!();

			self.gl_vert_shader_handle = vert_shader_handle;
			self.gl_frag_shader_handle = frag_shader_handle;
			self.gl_shader_program_handle = program_handle;

			// gl::UseProgram(self.gl_shader_program_handle);
			self.gl_attrib_location_tex = gl::GetUniformLocation(self.gl_shader_program_handle, imstr!("tex").as_ptr()) as u32;
			let gl_position_attrib = gl::GetAttribLocation(self.gl_shader_program_handle, imstr!("position").as_ptr()) as u32;
			let gl_tex_coords_attrib = gl::GetAttribLocation(self.gl_shader_program_handle, imstr!("tex_coords").as_ptr()) as u32;

			// let vertices = [
			// 	// position    tex-coords
			// 	 -1.0, -1.0,   0.0, 0.0, // bottom left
			// 	 -1.0,  1.0,   0.0, 1.0, // top left
			// 	  1.0, -1.0,   1.0, 0.0, // bottom right
			// 	  1.0,  1.0,   1.0, 1.0, // top right
			// ];

			let vertices: [f32; 16] = [
				 // position    tex-coords
				 -1.0,  1.0,   0.0, 0.0, // 0. top left
				 -1.0, -1.0,   0.0, 1.0, // 1. bottom left
				  1.0, -1.0,   1.0, 1.0, // 2. bottom right
				  1.0,  1.0,   1.0, 0.0, // 3. top right
			];

			gl::GenBuffers(1, &mut self.gl_vbo_handle);
			gl::GenBuffers(1, &mut self.gl_ebo_handle);
			gl::GenVertexArrays(1, &mut self.gl_vao_handle);
			gl::BindVertexArray(self.gl_vao_handle);


			gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_vbo_handle);
			gl::BufferData(gl::ARRAY_BUFFER, _size_of(vertices) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);
			gl_check_error!();

			let elements: [i32; 6] = [
				0, 2, 1,
				0, 2, 3
			];
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_ebo_handle);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, _size_of(elements) as isize, elements.as_ptr() as *const _, gl::STATIC_DRAW);
			gl_check_error!();


			gl::VertexAttribPointer(gl_position_attrib, 2, gl::FLOAT, gl::FALSE, (4 * mem::size_of::<f32>()) as i32, ptr::null());
			gl::VertexAttribPointer(gl_tex_coords_attrib, 2, gl::FLOAT, gl::FALSE, (4 * mem::size_of::<f32>()) as i32, (2 * mem::size_of::<f32>()) as *const () as *const _);
			gl::EnableVertexAttribArray(gl_position_attrib);
			gl::EnableVertexAttribArray(gl_tex_coords_attrib);
			gl_check_error!();

			let border_color = [0.5, 0.5, 0.5, 1.0];
			gl::GenTextures(1, &mut self.gl_texture_handle);
			gl::BindTexture(gl::TEXTURE_2D, self.gl_texture_handle);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
			gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, &border_color[0]);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

			let pixels = vec![1.0; 240 * 160 * 24];
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 240, 160, 0, gl::RGB, gl::FLOAT, pixels.as_ptr() as *const _);
			gl_check_error!();
		}

		imgui_support::imgui_init();

		debug_info!("Successfully initialized video device.");
	}

	pub fn uninit(&mut self) {
		unsafe {
			if self.gl_shader_program_handle != 0 {
				gl::DetachShader(self.gl_shader_program_handle, self.gl_vert_shader_handle);
				gl::DetachShader(self.gl_shader_program_handle, self.gl_frag_shader_handle);
			}
			if self.gl_vert_shader_handle != 0 { gl::DeleteShader(self.gl_vert_shader_handle); }
			if self.gl_frag_shader_handle != 0 { gl::DeleteShader(self.gl_frag_shader_handle); }
			if self.gl_shader_program_handle != 0 { gl::DeleteProgram(self.gl_shader_program_handle); }

			if self.gl_vbo_handle != 0 { gl::DeleteBuffers(1, &mut self.gl_vbo_handle); }
			if self.gl_ebo_handle != 0 { gl::DeleteBuffers(1, &mut self.gl_ebo_handle); }
			if self.gl_vao_handle != 0 { gl::DeleteVertexArrays(1, &mut self.gl_vao_handle); }

			if self.gl_texture_handle != 0 { gl::DeleteTextures(1, &mut self.gl_texture_handle); }
		}
		debug_info!("Successfully destroyed video device.");
	}

	pub fn prepare_imgui(&mut self) {
		let window_size = self.display.get_inner_size().expect("Unable to retrieve window dimensions.");
		let hidpi_factor = self.display.hidpi_factor();
		imgui_support::imgui_new_frame(&mut self.im_support, window_size, hidpi_factor);
	}

	/// Renders the screen texture.
	pub fn render(&mut self, buffer: &GbaLcdScreenBuffer) {
		profiler_begin!("Render GBA Frame");
		unsafe {
			gl::ClearColor(1.0, 0.0, 1.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			let (w, h) = self.display.get_inner_size().expect("Unable to retrieve window dimensions.");
			let hidpi_factor = self.display.hidpi_factor();
			let fw = (w as f32 * hidpi_factor) as i32;
			let fh = (h as f32 * hidpi_factor) as i32;
			gl::Viewport(0, 0, fw, fh);

			gl::UseProgram(self.gl_shader_program_handle);
			gl::ActiveTexture(gl::TEXTURE0);
			gl::Uniform1i(self.gl_attrib_location_tex as i32, 0);

			gl::TexSubImage2D(
				gl::TEXTURE_2D, 0,
				0, 0, 240, 160,
				gl::RGB, gl::UNSIGNED_BYTE,
				buffer.as_ptr() as *const _
			);

			gl::BindVertexArray(self.gl_vao_handle);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_vbo_handle);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_ebo_handle);
			gl::BindTexture(gl::TEXTURE_2D, self.gl_texture_handle);

			gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		}
		profiler_end!();
		profiler_begin!("ImGUI Render");
		imgui_support::imgui_render(&self.im_support);
		profiler_end!();

		profiler_begin!("Swap Buffers");
		self.display.swap_buffers().expect("Swapping glutin window buffers.");
		profiler_end!();
	}
}

impl Drop for VideoDevice {
	fn drop(&mut self) {
		self.uninit();
	}
}

unsafe fn link_program(program_handle_ptr: &mut u32, vert_shader: &mut u32, frag_shader: &mut u32) -> Result<(), String> {
	*program_handle_ptr = gl::CreateProgram();
	gl::AttachShader(*program_handle_ptr, *vert_shader);
	gl::AttachShader(*program_handle_ptr, *frag_shader);
	gl::LinkProgram(*program_handle_ptr);

	let mut status = 0;
	gl::GetProgramiv(*program_handle_ptr, gl::LINK_STATUS, &mut status);
	if (status as u8) != gl::TRUE {
		let mut info_log_length = 0;
		gl::GetProgramiv(*program_handle_ptr, gl::INFO_LOG_LENGTH, &mut info_log_length);
		let mut v = Vec::<u8>::with_capacity(info_log_length as usize);
		v.set_len(info_log_length as usize);
		let v_ptr = v.as_mut_ptr() as *mut i8;
		gl::GetProgramInfoLog(*program_handle_ptr, info_log_length, ptr::null_mut(), v_ptr);
		let log_string = String::from_utf8(v).expect("Failed to get linked program log");

		gl::DeleteProgram(*program_handle_ptr);
		gl::DeleteShader(*vert_shader);
		gl::DeleteShader(*frag_shader);

		*program_handle_ptr = 0;
		*vert_shader = 0;
		*frag_shader = 0;

		return Err(log_string);
	}

	return Ok(());
}

unsafe fn compile_shader(shader_handle_ptr: &mut u32, shader_type: u32, shader_source: &str) -> Result<(), String> {
	use std::ptr;

	*shader_handle_ptr = gl::CreateShader(shader_type);
	let shader_source_ptr = shader_source.as_ptr() as *const i8;
	let mut shader_length = shader_source.len() as i32;
	gl::ShaderSource(*shader_handle_ptr, 1, &shader_source_ptr, &mut shader_length);
	gl::CompileShader(*shader_handle_ptr);

	let mut status = 0;
	gl::GetShaderiv(*shader_handle_ptr, gl::COMPILE_STATUS, &mut status);

	if (status as u8) != gl::TRUE {
		let mut info_log_length = 0;
		gl::GetShaderiv(*shader_handle_ptr, gl::INFO_LOG_LENGTH, &mut info_log_length);
		let mut v = Vec::<u8>::with_capacity(info_log_length as usize);
		v.set_len(info_log_length as usize);
		let v_ptr = v.as_mut_ptr() as *mut i8;
		gl::GetShaderInfoLog(*shader_handle_ptr, info_log_length, ptr::null_mut(), v_ptr);
		let log_string = String::from_utf8(v).expect(&format!("Failed to get compiled shader log. shader_type: {}", shader_type));
		gl::DeleteShader(*shader_handle_ptr);
		*shader_handle_ptr = 0;

		return Err(log_string);
	}

	return Ok(());
}

pub fn __gl_check_error(filename: &'static str, fileline: u32) {
	let err = unsafe { gl::GetError() };
	if err != 0 {
		let err_string = match err {
			gl::INVALID_ENUM => "Invalid Enum",
			gl::INVALID_VALUE => "Invalid Value",
			gl::INVALID_OPERATION => "Invalid Operation",
			gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid FrameBuffer Operation",
			gl::OUT_OF_MEMORY => "Out of Memory",
			_ => "Unknown Error"
		};
		debug_error!("[{}:{}] OPENGL ERROR `{}`", filename, fileline, err_string);
	} else {
		debug_trace!("[{}:{}] OPENGL SUCCESS", filename, fileline);
	}
}

fn _size_of<T>(_: T) -> usize{
	use std::mem;
	mem::size_of::<T>()
}