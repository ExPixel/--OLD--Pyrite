extern crate glutin;
extern crate gl;
extern crate time;
#[macro_use] extern crate rust_imgui;
use rust_imgui as imgui;

pub fn imgui_example_draw() {
	let mut opened = true;
	imgui::begin(imstr!("Not A Window"), &mut opened, imgui::ImGuiWindowFlags_None);
	imgui::text(imstr!("Hello World"));
	imgui::text(imstr!("Application average {:.3} ms/frame ({:.1} FPS)",
		1000.0 / imgui::get_io().framerate,
		imgui::get_io().framerate));
	imgui::end();

	opened = true;
	imgui::show_test_window(&mut opened);
}

/// The main rendering function of ImGui. Can also be wrapped in an extern "C" and used as a callback.
/// but you will need a way to pass around your open GL state.
pub unsafe fn imgui_render_draw_lists(state: &DemoGLState, draw_data: &mut imgui::ImDrawData) {
	use std::mem;

	let io = imgui::get_io();
	let fb_width = io.display_size.x * io.display_framebuffer_scale.x;
	let fb_height = io.display_size.y * io.display_framebuffer_scale.y;
	if fb_width == 0.0 || fb_height == 0.0 { return; }
	// draw_data.scale_clip_rects(io.display_framebuffer_scale);
	placeholder_scale_clip_rects(draw_data, &io.display_framebuffer_scale);

	// Backup GL state.
	let mut last_program = 0; gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut last_program);
	let mut last_texture = 0; gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);
	let mut last_active_texture = 0; gl::GetIntegerv(gl::ACTIVE_TEXTURE, &mut last_active_texture);
	let mut last_array_buffer = 0; gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut last_array_buffer);
	let mut last_element_array_buffer = 0; gl::GetIntegerv(gl::ELEMENT_ARRAY_BUFFER_BINDING, &mut last_element_array_buffer);
	let mut last_vertex_array = 0; gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut last_vertex_array);
	let mut last_blend_src = 0; gl::GetIntegerv(gl::BLEND_SRC, &mut last_blend_src);
	let mut last_blend_dst = 0; gl::GetIntegerv(gl::BLEND_DST, &mut last_blend_dst);
	let mut last_blend_equation_rgb = 0; gl::GetIntegerv(gl::BLEND_EQUATION_RGB, &mut last_blend_equation_rgb);
	let mut last_blend_equation_alpha = 0; gl::GetIntegerv(gl::BLEND_EQUATION_ALPHA, &mut last_blend_equation_alpha);
	let mut last_viewport = [0i32; 4]; gl::GetIntegerv(gl::VIEWPORT, last_viewport[0..4].as_mut_ptr());
	let last_enable_blend = gl::IsEnabled(gl::BLEND);
	let last_enable_cull_face = gl::IsEnabled(gl::CULL_FACE);
	let last_enable_depth_test = gl::IsEnabled(gl::DEPTH_TEST);
	let last_enable_scissor_test = gl::IsEnabled(gl::SCISSOR_TEST);

	// Setup render state: alpha-blending enabled, no face culling, no depth testing, scissor enabled
	gl::Enable(gl::BLEND);
	gl::BlendEquation(gl::FUNC_ADD);
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
	gl::Disable(gl::CULL_FACE);
	gl::Disable(gl::DEPTH_TEST);
	gl::Enable(gl::SCISSOR_TEST);
	gl::ActiveTexture(gl::TEXTURE0);

	// Setup viewport, orthographic projection matrix
	gl::Viewport(0, 0, fb_width as i32, fb_height as i32);
	let ortho_projection: [[f32; 4]; 4] = [
		[ 2.0/io.display_size.x, 0.0,                   0.0, 0.0 ],
		[ 0.0,                   2.0/-io.display_size.y, 0.0, 0.0 ],
		[ 0.0,                   0.0,                   -1.0, 0.0 ],
		[-1.0,                   1.0,                    0.0, 1.0 ]
	];
	gl::UseProgram(state.shader_handle);
	gl::Uniform1i(state.attrib_location_tex as i32, 0);
	gl::UniformMatrix4fv(state.attrib_location_proj_mtx as i32, 1, gl::FALSE, &ortho_projection[0][0]);
	gl::BindVertexArray(state.vao_handle);

	for i in 0..draw_data.cmd_lists_count {
		let cmd_list = (*draw_data.cmd_lists.offset(i as isize)).as_mut()
			.expect("Indexing CMD List in imgui_render_draw_lists");
		let mut idx_buffer_offset: *const imgui::ImDrawIdx = mem::transmute(0usize);

		gl::BindBuffer(gl::ARRAY_BUFFER, state.vbo_handle);
		gl::BufferData(
			gl::ARRAY_BUFFER, 
			(cmd_list.vtx_buffer.size * mem::size_of::<imgui::ImDrawVert>() as i32) as isize,
			mem::transmute(&cmd_list.vtx_buffer[0]), 
			gl::STREAM_DRAW
		);

		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, state.elements_handle);
		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER,
			(cmd_list.idx_buffer.size * mem::size_of::<imgui::ImDrawIdx>() as i32) as isize,
			mem::transmute(&cmd_list.idx_buffer[0]),
			gl::STREAM_DRAW
		);

		for pcmd_idx in 0..cmd_list.cmd_buffer.size {
			let pcmd = &cmd_list.cmd_buffer[pcmd_idx as usize];
			if pcmd.user_callback.is_some() {
				let _callback = pcmd.user_callback.expect("Failed to get command list user callback.");
				_callback(cmd_list, pcmd);
			} else {
				gl::BindTexture(gl::TEXTURE_2D, (mem::transmute::<_, usize>(pcmd.texture_id)) as u32);
				gl::Scissor(pcmd.clip_rect.x as i32, (fb_height - pcmd.clip_rect.w) as i32, (pcmd.clip_rect.z - pcmd.clip_rect.x) as i32, (pcmd.clip_rect.w - pcmd.clip_rect.y) as i32);
				let _s = if mem::size_of::<imgui::ImDrawIdx>() == 2 {gl::UNSIGNED_SHORT} else {gl::UNSIGNED_INT};
				gl::DrawElements(gl::TRIANGLES, pcmd.elem_count as i32, _s, mem::transmute(idx_buffer_offset));
			}
			idx_buffer_offset = idx_buffer_offset.offset(pcmd.elem_count as isize);
		}
	}

	// Restore GL state
	gl::UseProgram(last_program as u32);
	gl::ActiveTexture(last_active_texture as u32);
	gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
	gl::BindVertexArray(last_vertex_array as u32);
	gl::BindBuffer(gl::ARRAY_BUFFER, last_array_buffer as u32);
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, last_element_array_buffer as u32);
	gl::BlendEquationSeparate(last_blend_equation_rgb as u32, last_blend_equation_alpha as u32);
	gl::BlendFunc(last_blend_src as u32, last_blend_dst as u32);

	if last_enable_blend != 0 { gl::Enable(gl::BLEND); }
	else { gl::Disable(gl::BLEND); }

	if last_enable_cull_face != 0 { gl::Enable(gl::CULL_FACE); }
	else { gl::Disable(gl::CULL_FACE) }

	if last_enable_depth_test != 0 { gl::Enable(gl::DEPTH_TEST); }
	else { gl::Disable(gl::DEPTH_TEST); }

	if last_enable_scissor_test != 0 { gl::Enable(gl::SCISSOR_TEST); }
	else { gl::Disable(gl::SCISSOR_TEST); }

	gl::Viewport(last_viewport[0], last_viewport[1], last_viewport[2], last_viewport[3]);
}

// #NOTE Placeholder functions until cimgui implements the ScaleClipRects
//       function in ImGui.
pub fn placeholder_scale_clip_rects(draw_data: &mut imgui::ImDrawData, scale: &imgui::ImVec2) {
	for i in 0..draw_data.cmd_lists_count {
		let cmd_list = unsafe {
			(*draw_data.cmd_lists.offset(i as isize)).as_mut()
				.expect("Indexing CMD list in placeholder_scale_clip_rects")
		};
		for cmd_i in 0..cmd_list.cmd_buffer.size {
			let imgui::ImVec4 {x, y, z, w} = cmd_list.cmd_buffer[cmd_i as usize].clip_rect;
			cmd_list.cmd_buffer[cmd_i as usize].clip_rect = imgui::vec4(
				x * scale.x,
				y * scale.y,
				z * scale.x,
				w * scale.y
			);
		}
	}
}

pub fn imgui_new_frame(state: &mut DemoGLState, window_size: (u32, u32), hidpi_factor: f32) {
	if state.font_texture == 0 {
		unsafe { imgui_create_device_objects(state) };
	}

	let io = imgui::get_io();

	let (w, h) = window_size;
	let hidpi_factor = hidpi_factor;

	io.display_size = imgui::vec2(w as f32, h as f32);
	io.display_framebuffer_scale = imgui::vec2(hidpi_factor, hidpi_factor); // for hidpi displays (e.g. Macs with retina displays)

	// Setup time step:
	let current_time = time::precise_time_ns();
	io.delta_time = if state.time > 0{
		((current_time - state.time) as f64 / 1000000000.0) as f32
	} else {
		1.0 / 60.0
	};
	state.time = current_time;

	// Update Mouse:
	io.mouse_pos = imgui::vec2(state.mouse_position.0 as f32 / hidpi_factor, state.mouse_position.1 as f32 / hidpi_factor);
	io.mouse_down[0] = imgui::cbool(state.mouse_pressed[0]);
	io.mouse_down[1] = imgui::cbool(state.mouse_pressed[1]);
	io.mouse_down[2] = imgui::cbool(state.mouse_pressed[2]);
	io.mouse_wheel = state.mouse_wheel;
	state.mouse_wheel = 0.0;

	imgui::new_frame();
}

pub unsafe fn imgui_create_device_objects(state: &mut DemoGLState) {
	use std::mem;

	// Backing up the GL state.
	let mut last_texture = 0;
	let mut last_array_buffer = 0;
	let mut last_vertex_array = 0;
	gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);
	gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut last_array_buffer);
	gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut last_vertex_array);

	let vertex_shader = "
		#version 330\n
		uniform mat4 ProjMtx;\n
		in vec2 Position;\n
		in vec2 UV;\n
		in vec4 Color;\n
		out vec2 Frag_UV;\n
		out vec4 Frag_Color;\n
		void main()\n
		{\n
			Frag_UV = UV;\n
			Frag_Color = Color;\n
			gl_Position = ProjMtx * vec4(Position.xy,0,1);\n
		}\n
	";

	let fragment_shader = "
		#version 330\n
		uniform sampler2D Texture;\n
		in vec2 Frag_UV;\n
		in vec4 Frag_Color;\n
		out vec4 Out_Color;\n
		void main()\n
		{\n
			Out_Color = Frag_Color * texture( Texture, Frag_UV.st);\n
		}\n
	";

	state.shader_handle = gl::CreateProgram();
	state.vert_handle = gl::CreateShader(gl::VERTEX_SHADER);
	state.frag_handle = gl::CreateShader(gl::FRAGMENT_SHADER);

	let vertex_shader_ptr = mem::transmute(vertex_shader.as_ptr());
	let fragment_shader_ptr = mem::transmute(fragment_shader.as_ptr());
	let vertex_shader_len = vertex_shader.len() as i32;
	let fragment_shader_len = fragment_shader.len() as i32;
	gl::ShaderSource(state.vert_handle, 1, &vertex_shader_ptr, &vertex_shader_len);
	gl::ShaderSource(state.frag_handle, 1, &fragment_shader_ptr, &fragment_shader_len);
	gl::CompileShader(state.vert_handle);

	{
		let mut is_compiled = 0;
		gl::GetShaderiv(state.vert_handle, gl::COMPILE_STATUS, &mut is_compiled);
		if is_compiled == (gl::FALSE as i32) {
			println!("Error while compiling vertex shader.");
		}
	}

	gl::CompileShader(state.frag_handle);

	{
		let mut is_compiled = 0;
		gl::GetShaderiv(state.frag_handle, gl::COMPILE_STATUS, &mut is_compiled);
		if is_compiled == (gl::FALSE as i32) {
			println!("Error while compiling fragment shader.");
		}
	}

	// #NOTE you should probably make sure that your shaders compiled successfully here.
	gl::AttachShader(state.shader_handle, state.vert_handle);
	gl::AttachShader(state.shader_handle, state.frag_handle);
	gl::LinkProgram(state.shader_handle);

	state.attrib_location_tex = gl::GetUniformLocation(state.shader_handle, imstr!("Texture").as_ptr()) as u32;
	state.attrib_location_proj_mtx = gl::GetUniformLocation(state.shader_handle, imstr!("ProjMtx").as_ptr()) as u32;
	state.attrib_location_position = gl::GetAttribLocation(state.shader_handle, imstr!("Position").as_ptr()) as u32;
	state.attrib_location_uv = gl::GetAttribLocation(state.shader_handle, imstr!("UV").as_ptr()) as u32;
	state.attrib_location_color = gl::GetAttribLocation(state.shader_handle, imstr!("Color").as_ptr()) as u32;

	gl::GenBuffers(1, &mut state.vbo_handle);
	gl::GenBuffers(1, &mut state.elements_handle);

	gl::GenVertexArrays(1, &mut state.vao_handle);
	gl::BindVertexArray(state.vao_handle);
	gl::BindBuffer(gl::ARRAY_BUFFER, state.vbo_handle);
	gl::EnableVertexAttribArray(state.attrib_location_position);
	gl::EnableVertexAttribArray(state.attrib_location_uv);
	gl::EnableVertexAttribArray(state.attrib_location_color);

	let dv_size = mem::size_of::<imgui::ImDrawVert>() as i32;
	gl::VertexAttribPointer(state.attrib_location_position, 2, gl::FLOAT, gl::FALSE, dv_size, mem::transmute(0usize));
	gl::VertexAttribPointer(state.attrib_location_uv, 2, gl::FLOAT, gl::FALSE, dv_size, mem::transmute(8usize));
	gl::VertexAttribPointer(state.attrib_location_color, 4, gl::UNSIGNED_BYTE, gl::TRUE, dv_size, mem::transmute(16usize));

	imgui_create_fonts_texture(state);

	gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
	gl::BindBuffer(gl::ARRAY_BUFFER, last_array_buffer as u32);
	gl::BindVertexArray(last_vertex_array as u32);
}

pub unsafe fn imgui_create_fonts_texture(state: &mut DemoGLState) {
	use std::ptr;
	use std::mem;

	let io = imgui::get_io();
	let mut pixels: *mut u8 = ptr::null_mut();
	let mut width = 0;
	let mut height = 0;
	let mut bytes_per_pixel = 0;
	io.fonts.get_tex_data_as_rgba32(&mut pixels, &mut width, &mut height, &mut bytes_per_pixel);

	let mut last_texture = 0;
	gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);
	gl::GenTextures(1, &mut state.font_texture);
	gl::BindTexture(gl::TEXTURE_2D, state.font_texture);
	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
	gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(pixels));

	io.fonts.tex_id = mem::transmute(state.font_texture as usize);

	gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
}

/// Setup the ImGui bindings.
pub fn imgui_init() {
	let io = imgui::get_io();
	io.key_map[imgui::ImGuiKey::Tab as usize] = 0;
	io.key_map[imgui::ImGuiKey::LeftArrow as usize] = 1;
	io.key_map[imgui::ImGuiKey::RightArrow as usize] = 2;
	io.key_map[imgui::ImGuiKey::UpArrow as usize] = 3;
	io.key_map[imgui::ImGuiKey::DownArrow as usize] = 4;
	io.key_map[imgui::ImGuiKey::PageUp as usize] = 5;
	io.key_map[imgui::ImGuiKey::PageDown as usize] = 6;
	io.key_map[imgui::ImGuiKey::Home as usize] = 7;
	io.key_map[imgui::ImGuiKey::End as usize] = 8;
	io.key_map[imgui::ImGuiKey::Delete as usize] = 9;
	io.key_map[imgui::ImGuiKey::Backspace as usize] = 10;
	io.key_map[imgui::ImGuiKey::Enter as usize] = 11;
	io.key_map[imgui::ImGuiKey::Escape as usize] = 12;
	io.key_map[imgui::ImGuiKey::A as usize] = 13;
	io.key_map[imgui::ImGuiKey::C as usize] = 14;
	io.key_map[imgui::ImGuiKey::V as usize] = 15;
	io.key_map[imgui::ImGuiKey::X as usize] = 16;
	io.key_map[imgui::ImGuiKey::Y as usize] = 17;
	io.key_map[imgui::ImGuiKey::Z as usize] = 18;
}

pub fn imgui_shutdown(state: &mut DemoGLState) {
	unsafe {
		if state.vao_handle != 0 { gl::DeleteVertexArrays(1, &state.vao_handle); }
		if state.vbo_handle != 0 { gl::DeleteBuffers(1, &state.vbo_handle); }
		if state.elements_handle != 0 { gl::DeleteBuffers(1, &state.elements_handle); }
		state.vao_handle = 0;
		state.vbo_handle = 0;
		state.elements_handle = 0;

		gl::DetachShader(state.shader_handle, state.vert_handle);
		gl::DeleteShader(state.vert_handle);
		state.vert_handle = 0;

		gl::DetachShader(state.shader_handle, state.frag_handle);
		gl::DeleteShader(state.frag_handle);
		state.frag_handle = 0;

		gl::DeleteProgram(state.shader_handle);
		state.shader_handle = 0;

		if state.font_texture != 0 {
			use std::ptr;
			gl::DeleteTextures(1, &state.font_texture);
			imgui::get_io().fonts.tex_id = ptr::null_mut();
			state.font_texture = 0;
		}
	}
	imgui::shutdown();
}

// You can also use a callback by wrapping imgui_render_draw_lists
// in an extern C function.
pub fn imgui_render(state: &DemoGLState) {
	imgui::render();
	let draw_data = imgui::get_draw_data().expect("null imgui draw data.");
	unsafe { imgui_render_draw_lists(state, draw_data) };
}

pub fn imgui_check_event(ui_state: &mut DemoGLState, event: &glutin::Event) {
	use glutin::{VirtualKeyCode, Event, ElementState, 
			MouseButton, MouseScrollDelta, TouchPhase};

	let io = imgui::get_io();
	match *event {
		Event::KeyboardInput(state, _, code) => {
			let pressed = imgui::cbool(state == ElementState::Pressed);
			match code {
				Some(VirtualKeyCode::Tab) => io.keys_down[0] = pressed,
				Some(VirtualKeyCode::Left) => io.keys_down[1] = pressed,
				Some(VirtualKeyCode::Right) => io.keys_down[2] = pressed,
				Some(VirtualKeyCode::Up) => io.keys_down[3] = pressed,
				Some(VirtualKeyCode::Down) => io.keys_down[4] = pressed,
				Some(VirtualKeyCode::PageUp) => io.keys_down[5] = pressed,
				Some(VirtualKeyCode::PageDown) => io.keys_down[6] = pressed,
				Some(VirtualKeyCode::Home) => io.keys_down[7] = pressed,
				Some(VirtualKeyCode::End) => io.keys_down[8] = pressed,
				Some(VirtualKeyCode::Delete) => io.keys_down[9] = pressed,
				Some(VirtualKeyCode::Back) => io.keys_down[10] = pressed,
				Some(VirtualKeyCode::Return) => io.keys_down[11] = pressed,
				Some(VirtualKeyCode::Escape) => io.keys_down[12] = pressed,
				Some(VirtualKeyCode::A) => io.keys_down[13] = pressed,
				Some(VirtualKeyCode::C) => io.keys_down[14] = pressed,
				Some(VirtualKeyCode::V) => io.keys_down[15] = pressed,
				Some(VirtualKeyCode::X) => io.keys_down[16] = pressed,
				Some(VirtualKeyCode::Y) => io.keys_down[17] = pressed,
				Some(VirtualKeyCode::Z) => io.keys_down[18] = pressed,
				Some(VirtualKeyCode::LControl) | Some(VirtualKeyCode::RControl) => {
					io.key_ctrl = pressed;
				},
				Some(VirtualKeyCode::LShift) | Some(VirtualKeyCode::RShift) => {
					io.key_shift = pressed;
				},
				Some(VirtualKeyCode::LAlt) | Some(VirtualKeyCode::RAlt) => {
					io.key_alt = pressed;
				}, // #TODO super key.
				_ => {}
			}
		},
		Event::MouseMoved(x, y) => ui_state.mouse_position = (x, y),
		Event::MouseInput(state, MouseButton::Left) => ui_state.mouse_pressed[0] = state == ElementState::Pressed,
		Event::MouseInput(state, MouseButton::Right) => ui_state.mouse_pressed[1] = state == ElementState::Pressed,
		Event::MouseInput(state, MouseButton::Middle) => ui_state.mouse_pressed[2] = state == ElementState::Pressed,
		Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) => ui_state.mouse_wheel = y,
		Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => ui_state.mouse_wheel = y,
		Event::ReceivedCharacter(c) => {
			io.add_input_character(c as u16);
		},
		_ => {}
	}
}

pub struct DemoGLState {
	pub time: u64,
	pub mouse_position: (i32, i32),
	pub mouse_pressed: [bool; 3],
	pub mouse_wheel: f32,
	pub font_texture: u32,
	pub shader_handle: u32,
	pub vert_handle: u32,
	pub frag_handle: u32,
	pub attrib_location_tex: u32,
	pub attrib_location_proj_mtx: u32,
	pub attrib_location_position: u32,
	pub attrib_location_uv: u32,
	pub attrib_location_color: u32,
	pub vbo_handle: u32,
	pub vao_handle: u32,
	pub elements_handle: u32,
}

pub fn main() {
	let builder = glutin::WindowBuilder::new()
		.with_dimensions(800, 600)
		.with_vsync();
	let window = builder.build().unwrap();

	let mut demo_state = DemoGLState {
		time: 0,
		mouse_position: (0, 0),
		mouse_pressed: [false; 3],
		mouse_wheel: 0.0,
		font_texture: 0,
		shader_handle: 0,
		vert_handle: 0,
		frag_handle: 0,
		attrib_location_tex: 0,
		attrib_location_proj_mtx: 0,
		attrib_location_position: 0,
		attrib_location_uv: 0,
		attrib_location_color: 0,
		vbo_handle: 0,
		vao_handle: 0,
		elements_handle: 0
	};

	unsafe { window.make_current().unwrap() };

	window.set_title("ImGUI-rs Demo");

	unsafe {
		gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
		gl::ClearColor(0.1, 0.08, 0.7, 1.0);
	}

	imgui_init();
	'main_loop: loop {
		// Poll for events first or you're going to have a bad time.
		for event in window.poll_events() {
			imgui_check_event(&mut demo_state, &event);
			match event {
				glutin::Event::Closed => break 'main_loop,
				_ => ()
			}
		}

		let window_size = window.get_inner_size().expect("Unable to retrieve glutin window size.");
		let hidpi_factor = window.hidpi_factor();
		// #WARNING calling imgui functions that try to draw widgets
		// before new frame will cause a segfault.
		imgui_new_frame(&mut demo_state, window_size, hidpi_factor);
		imgui_example_draw();

		let (w, h) = window.get_inner_size().expect("Unable to retrieve window dimensions.");
		unsafe { gl::Viewport(0, 0, w as i32, h as i32) };
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

		// #WARINING calling imgui functions that try to draw widgets
		// after here (really imgui::render) will cause a segfault.
		imgui_render(&demo_state);
		window.swap_buffers().expect("Swapping glutin window buffers.");
	}
	imgui_shutdown(&mut demo_state);
}