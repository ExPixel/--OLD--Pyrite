use rust_imgui as imgui;
use ::gba::Gba;
use std::cell::UnsafeCell;

pub struct DebugDataRuleBreaker {
	data: UnsafeCell<DebugData>
}
impl DebugDataRuleBreaker {
	pub fn new() -> DebugDataRuleBreaker {
		DebugDataRuleBreaker {
			data: UnsafeCell::new(DebugData::new())
		}
	}
}
unsafe impl Sync for DebugDataRuleBreaker {}

lazy_static! {
	pub static ref DEBUGGER: DebugDataRuleBreaker = DebugDataRuleBreaker::new();
}

pub fn get_debugger() -> &'static mut DebugData {
	unsafe {
		DEBUGGER.data.get().as_mut().expect("Failed to get a reference to the debugger.")
	}
}

pub struct DebugData {
	pub frame_build_time: f64,
	pub frame_render_time: f64,
	pub full_frame_time: f64,
	pub emulator_performance_opened: bool,

	// SOUND:
	pub sound_info_window: bool,
	pub sound_channel_1_plot: DataPlot<f32>,
	pub sound_channel_2_plot: DataPlot<f32>,
	pub sound_channel_3_plot: DataPlot<f32>,
	pub sound_channel_4_plot: DataPlot<f32>,
	pub sound_channel_a_plot: DataPlot<f32>,
	pub sound_channel_b_plot: DataPlot<f32>,
	pub sound_plot: DataPlot<f32>,
}

impl DebugData {
	pub fn new() -> DebugData {
		DebugData {
			frame_build_time: 0.0,
			frame_render_time: 0.0,
			full_frame_time: 0.0,
			emulator_performance_opened: false,

			sound_info_window: false,
			sound_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_1_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_2_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_3_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_4_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_a_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_b_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
		}
	}
}

pub fn render_debugger(_: &mut Gba) {
	let debugger = get_debugger();

	if imgui::get_io().mouse_clicked[1] != 0 {
		imgui::open_popup(imstr!("main_menu"));
	}

	if imgui::begin_popup(imstr!("main_menu")) {

		if imgui::menu_item(imstr!("Performance")) {
			debugger.emulator_performance_opened = true;
		}

		if imgui::menu_item(imstr!("Sound")) {
			debugger.sound_info_window = true;
		}

		imgui::end_popup();
	}

	if debugger.emulator_performance_opened {
		imgui::begin(imstr!("Emulator Performance"), &mut debugger.emulator_performance_opened, imgui::ImGuiWindowFlags_None);
		imgui::text(imstr!("Frame Build Time: {:.2}ms", debugger.frame_build_time));
		imgui::text(imstr!("Frame Render Time: {:.2}ms", debugger.frame_render_time));
		imgui::text(imstr!("Frame Time: {:.2}ms", debugger.full_frame_time));

		// imgui::plot_lines(imstr!("plot test"),
		// 	&data, 8, 4, imstr!("pline"), 1.0, 8.0, imgui::vec2(256.0, 128.0), 4);

		// let _c = debugger.random_plot.write_cursor as f32;
		// debugger.random_plot.plot(_c);
		// imgui::plot_lines(imstr!("random plot"),
		// 	&data, 
		// 	debugger.random_plot.len(), debugger.random_plot.offset(), 
		// 	imstr!("random plot"), 
		// 	0.0, 32.0,
		// 	imgui::vec2(256.0, 128.0), 4);

		imgui::end();
	}

	if debugger.sound_info_window {
		imgui::begin(imstr!("Emulator Sound"), &mut debugger.sound_info_window, imgui::ImGuiWindowFlags_None);
		imgui::plot_lines(imstr!("Sound Output"),
			&debugger.sound_plot.data,
			debugger.sound_plot.len(), debugger.sound_plot.offset(), 
			imstr!("Signal"), 
			debugger.sound_plot.plot_min, debugger.sound_plot.plot_max,
			imgui::vec2(256.0, 128.0), 4);

		if imgui::collapsing_header(imstr!("Channel 1"), imstr!("sc1_clpshr"), true, false) {
			imgui::plot_lines(imstr!("Sound Channel 1"),
				&debugger.sound_channel_1_plot.data,
				debugger.sound_channel_1_plot.len(), debugger.sound_channel_1_plot.offset(), 
				imstr!("Signal"), 
				debugger.sound_channel_1_plot.plot_min, debugger.sound_channel_1_plot.plot_max,
				imgui::vec2(256.0, 128.0), 4);
		}

		if imgui::collapsing_header(imstr!("Channel 2"), imstr!("sc2_clpshr"), true, false) {
			imgui::plot_lines(imstr!("Sound Channel 2"),
				&debugger.sound_channel_2_plot.data,
				debugger.sound_channel_2_plot.len(), debugger.sound_channel_2_plot.offset(), 
				imstr!("Signal"), 
				debugger.sound_channel_2_plot.plot_min, debugger.sound_channel_2_plot.plot_max,
				imgui::vec2(256.0, 128.0), 4);
		}

		if imgui::collapsing_header(imstr!("Channel 3"), imstr!("sc3_clpshr"), true, false) {
			imgui::plot_lines(imstr!("Sound Channel 3"),
				&debugger.sound_channel_3_plot.data,
				debugger.sound_channel_3_plot.len(), debugger.sound_channel_3_plot.offset(), 
				imstr!("Signal"), 
				debugger.sound_channel_3_plot.plot_min, debugger.sound_channel_3_plot.plot_max,
				imgui::vec2(256.0, 128.0), 4);
		}

		if imgui::collapsing_header(imstr!("Channel 4"), imstr!("sc4_clpshr"), true, false) {
			imgui::plot_lines(imstr!("Sound Channel 4"),
				&debugger.sound_channel_4_plot.data,
				debugger.sound_channel_4_plot.len(), debugger.sound_channel_4_plot.offset(), 
				imstr!("Signal"), 
				debugger.sound_channel_4_plot.plot_min, debugger.sound_channel_4_plot.plot_max,
				imgui::vec2(256.0, 128.0), 4);
		}

		if imgui::collapsing_header(imstr!("Channel A"), imstr!("sca_clpshr"), true, false) {
			imgui::plot_lines(imstr!("Sound Channel A"),
				&debugger.sound_channel_a_plot.data,
				debugger.sound_channel_a_plot.len(), debugger.sound_channel_a_plot.offset(), 
				imstr!("Signal"), 
				debugger.sound_channel_a_plot.plot_min, debugger.sound_channel_a_plot.plot_max,
				imgui::vec2(256.0, 128.0), 4);
		}

		if imgui::collapsing_header(imstr!("Channel B"), imstr!("scb_clpshr"), true, false) {
			imgui::plot_lines(imstr!("Sound Channel B"),
				&debugger.sound_channel_b_plot.data,
				debugger.sound_channel_b_plot.len(), debugger.sound_channel_b_plot.offset(), 
				imstr!("Signal"), 
				debugger.sound_channel_b_plot.plot_min, debugger.sound_channel_b_plot.plot_max,
				imgui::vec2(256.0, 128.0), 4);
		}
		imgui::end();
	}
}

pub struct DataPlot<T: Clone> {
	data: Vec<T>,
	plot_max: T,
	plot_min: T,
	max_size: usize,
	read_cursor: usize,
	write_cursor: usize,

	skip: usize,
	skipped: usize,
}

impl<T: Clone> DataPlot<T> {
	pub fn new(max_size: usize, plot_min: T, plot_max: T) -> DataPlot<T> {
		Self::with_skip(max_size, plot_min, plot_max, 0)
	}

	pub fn with_skip(max_size: usize, plot_min: T, plot_max: T, skip: usize) -> DataPlot<T> {
		let mut ret = DataPlot {
			data: Vec::with_capacity(max_size),
			max_size: max_size,
			read_cursor: 0,
			write_cursor: 0,
			plot_min: plot_min.clone(),
			plot_max: plot_max,
			skip: skip,
			skipped: 0,
		};
		ret.skipped = ret.skip;
		ret.plot(plot_min.clone());
		return ret;
	}

	fn offset(&self) -> i32 {
		self.read_cursor as i32
	}

	fn len(&self) -> i32 {
		self.data.len() as i32
	}

	pub fn plot(&mut self, point: T) {
		self.skipped += 1;
		if self.skipped >= self.skip {
			self.skipped = 0;
			if self.data.len() < self.max_size {
				self.data.push(point);
				self.write_cursor += 1;
				if self.write_cursor >= self.max_size { self.write_cursor = 0; }
			} else {
				self.data[self.write_cursor] = point;
				self.write_cursor += 1;
				if self.write_cursor >= self.max_size { self.write_cursor = 0; }
				if self.write_cursor == self.read_cursor {
					self.read_cursor += 1;
					if self.read_cursor >= self.max_size {
						self.read_cursor = 0;
					}
				}
			}
		}
	}
}