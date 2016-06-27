#[macro_export]
macro_rules! console_log_with_color {
	($color:expr, $message:expr, $($arg:tt)+) => (
		console_log_with_color!($color, format!($message, $($arg)+));
	);

	($color:expr, $message:expr) => (
		::debug::debugger::get_debugger().console.log($color, $message.into());
	);
}

macro_rules! console_log {
	($message:expr, $($arg:tt)+) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_NORMAL, $message, $($arg)+);
	);

	($message:expr) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_NORMAL, $message);
	);
}

macro_rules! console_warn {
	($message:expr, $($arg:tt)+) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_WARNING, $message, $($arg)+);
	);

	($message:expr) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_WARNING, $message);
	);
}

macro_rules! console_error {
	($message:expr, $($arg:tt)+) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_ERROR, $message, $($arg)+);
	);

	($message:expr) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_ERROR, $message);
	);
}

pub mod console;
pub mod memory_editor;

use rust_imgui as imgui;
use rust_imgui::ImVec4;
use rust_imgui::imstr::ImStr;
use ::gba::Gba;
use ::gba::core::memory::*;
use self::console::ImGuiConsole;
use self::memory_editor::MemoryEditor;
use std::marker::PhantomData;
use ::util::sync_unsafe_cell::SyncUnsafeCell;

pub const CONSOLE_COLOR_NORMAL: ImVec4 = ImVec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }; // #FFFFFF
pub const CONSOLE_COLOR_WARNING: ImVec4 = ImVec4 { x: 1.0, y: 0.922, z: 0.231, w: 1.0 }; // #FFEB3B
pub const CONSOLE_COLOR_ERROR: ImVec4 = ImVec4 { x: 0.957, y: 0.263, z: 0.212, w: 1.0 }; // #F44336

lazy_static! {
	pub static ref DEBUGGER: SyncUnsafeCell<DebugData> = SyncUnsafeCell::new(DebugData::new());
}

pub fn get_debugger() -> &'static mut DebugData {
	unsafe {
		DEBUGGER.get().as_mut().expect("Failed to get a reference to the debugger.")
	}
}

pub struct DebugData {
	pub console: ImGuiConsole,
	pub console_window_opened: bool,

	pub memory_window: MemoryEditor,
	pub memory_window_opened: bool,

	pub frame_build_time: f64,
	pub frame_render_time: f64,
	pub full_frame_time: f64,
	pub emulator_performance_opened: bool,
	pub emulator_delay_plot: DataPlot<f32>,

	// SOUND:
	pub sound_info_window_opened: bool,
	pub sound_channel_1_plot: DataPlot<f32>,
	pub sound_channel_2_plot: DataPlot<f32>,
	pub sound_channel_3_plot: DataPlot<f32>,
	pub sound_channel_4_plot: DataPlot<f32>,
	pub sound_channel_a_plot: DataPlot<f32>,
	pub sound_channel_b_plot: DataPlot<f32>,
	pub sound_plot: DataPlot<f32>,

	ioreg_window_opened: bool,
}

impl DebugData {
	pub fn new() -> DebugData {
		DebugData {
			console: ImGuiConsole::new(100, true),
			console_window_opened: false,

			memory_window: MemoryEditor::new(),
			memory_window_opened: false,

			frame_build_time: 0.0,
			frame_render_time: 0.0,
			full_frame_time: 0.0,
			emulator_performance_opened: false,
			emulator_delay_plot: DataPlot::new("Delay", "Frame Delay", 64, 0.0, 100.0),

			sound_info_window_opened: false,
			sound_plot: DataPlot::with_skip("Signal", "Sound Output", 128, -32768.0, 32767.0, 16),
			sound_channel_1_plot: DataPlot::with_skip("Signal", "Channel 1", 128, -32768.0, 32767.0, 16),
			sound_channel_2_plot: DataPlot::with_skip("Signal", "Channel 2", 128, -32768.0, 32767.0, 16),
			sound_channel_3_plot: DataPlot::with_skip("Signal", "Channel 3", 128, -32768.0, 32767.0, 16),
			sound_channel_4_plot: DataPlot::with_skip("Signal", "Channel 4", 128, -32768.0, 32767.0, 16),
			sound_channel_a_plot: DataPlot::with_skip("Signal", "Channel A", 128, -32768.0, 32767.0, 16),
			sound_channel_b_plot: DataPlot::with_skip("Signal", "Channel B", 128, -32768.0, 32767.0, 16),

			ioreg_window_opened: false,
		}
	}
}

pub fn render_debugger(gba: &mut Gba) {
	use rust_imgui::ImGuiSelectableFlags_SpanAllColumns;

	let debugger = get_debugger();

	if imgui::get_io().mouse_clicked[1] != 0 {
		imgui::open_popup(imstr!("main_menu"));
	}

	if imgui::begin_popup(imstr!("main_menu")) {

		if imgui::menu_item(imstr!("Performance")) {
			debugger.emulator_performance_opened = true;
		}

		if imgui::menu_item(imstr!("Sound")) {
			debugger.sound_info_window_opened = true;
		}

		if imgui::menu_item(imstr!("IO Registers")) {
			debugger.ioreg_window_opened = true;
		}

		if imgui::menu_item(imstr!("Console")) {
			debugger.console_window_opened = true;
		}

		if imgui::menu_item(imstr!("Memory")) {
			debugger.memory_window_opened = true;
		}

		imgui::end_popup();
	}

	if debugger.console_window_opened {
		imgui::set_next_window_size(imgui::vec2(320.0, 400.0), imgui::ImGuiSetCond::FirstUseEver);
		imgui::begin(imstr!("Console"), &mut debugger.console_window_opened, imgui::ImGuiWindowFlags_None);
		imgui::push_style_var_vec(imgui::ImGuiStyleVar::ItemSpacing, imgui::vec2(4.0, 1.0));
		debugger.console.render();
		imgui::pop_style_var(1);
		imgui::end();
	}

	if debugger.memory_window_opened {
		imgui::set_next_window_size(imgui::vec2(320.0, 400.0), imgui::ImGuiSetCond::FirstUseEver);
		imgui::begin(imstr!("Memory Viewer"), &mut debugger.memory_window_opened, imgui::ImGuiWindowFlags_None);
		debugger.memory_window.render(&gba.cpu.memory);
		imgui::end();
	}

	if debugger.emulator_performance_opened {
		imgui::begin(imstr!("Emulator Performance"), &mut debugger.emulator_performance_opened, imgui::ImGuiWindowFlags_None);
		imgui::text(imstr!("Frame Build Time: {:.2}ms", debugger.frame_build_time));
		imgui::text(imstr!("Frame Render Time: {:.2}ms", debugger.frame_render_time));
		imgui::text(imstr!("Frame Time: {:.2}ms", debugger.full_frame_time));
		
		debugger.emulator_delay_plot.render_histogram();

		if !imgui::is_item_hovered() {
			debugger.emulator_delay_plot.plot(debugger.frame_build_time as f32);	
		}

		if imgui::collapsing_header(imstr!("Audio Buffer"), imstr!("audio_buffer_clpshr"), true, false) {
			use std::sync::atomic::Ordering::Relaxed;
			imgui::label_text(imstr!("Read Misses"), imstr!("{}", gba.device.audio.ring_buffer._stat_read_misses.load(Relaxed)));
			imgui::label_text(imstr!("Write Misses"), imstr!("{}", gba.device.audio.ring_buffer._stat_write_misses.load(Relaxed)));
		}

		imgui::end();
	}

	if debugger.ioreg_window_opened {
		imgui::begin(imstr!("IO Registers"), &mut debugger.ioreg_window_opened, imgui::ImGuiWindowFlags_None);
		if imgui::collapsing_header(imstr!("DMA"), imstr!("dma_ioreg_clpshr"), true, false) {
			render_dma_register(gba, 0, ioreg::DMA0CNT_L, ioreg::DMA0CNT_H, ioreg::DMA0SAD, ioreg::DMA0DAD);
			render_dma_register(gba, 1, ioreg::DMA1CNT_L, ioreg::DMA1CNT_H, ioreg::DMA1SAD, ioreg::DMA1DAD);
			render_dma_register(gba, 2, ioreg::DMA2CNT_L, ioreg::DMA2CNT_H, ioreg::DMA2SAD, ioreg::DMA2DAD);
			render_dma_register(gba, 3, ioreg::DMA3CNT_L, ioreg::DMA3CNT_H, ioreg::DMA3SAD, ioreg::DMA3DAD);
		}

		if imgui::collapsing_header(imstr!("Timers"), imstr!("timer_ioreg_clpshr"), true, false) {
			render_timer_register(gba, 0, ioreg::TM0CNT_L, ioreg::TM0CNT_H);
			render_timer_register(gba, 1, ioreg::TM1CNT_L, ioreg::TM1CNT_H);
			render_timer_register(gba, 2, ioreg::TM2CNT_L, ioreg::TM2CNT_H);
			render_timer_register(gba, 3, ioreg::TM3CNT_L, ioreg::TM3CNT_H);
		}

		if imgui::collapsing_header(imstr!("Sound"), imstr!("sound_ioreg_clpshr"), true, false) {
			imgui::columns(2, imstr!("sound_reg_table"), true);
			imgui::selectable_fl(imstr!("SOUNDCNT_L"), ImGuiSelectableFlags_SpanAllColumns);
			imgui::selectable_fl(imstr!("SOUNDCNT_H"), ImGuiSelectableFlags_SpanAllColumns);
			imgui::selectable_fl(imstr!("SOUNDCNT_X"), ImGuiSelectableFlags_SpanAllColumns);
			imgui::next_column();
			imgui::text(imstr!("{:04X}", gba.cpu.memory.get_reg(ioreg::SOUNDCNT_L)));
			imgui::text(imstr!("{:04X}", gba.cpu.memory.get_reg(ioreg::SOUNDCNT_H)));
			imgui::text(imstr!("{:04X}", gba.cpu.memory.get_reg(ioreg::SOUNDCNT_X)));
			imgui::columns(1, imstr!("sound_reg_table_end"), false);
		}
		imgui::end();
	}

	if debugger.sound_info_window_opened {
		imgui::begin(imstr!("Emulator Sound"), &mut debugger.sound_info_window_opened, imgui::ImGuiWindowFlags_None);
		debugger.sound_plot.render_lines();

		if imgui::collapsing_header(imstr!("Channel 1"), imstr!("sc1_clpshr"), true, false) {
			debugger.sound_channel_1_plot.render_lines();
		}

		if imgui::collapsing_header(imstr!("Channel 2"), imstr!("sc2_clpshr"), true, false) {
			debugger.sound_channel_2_plot.render_lines();
		}

		if imgui::collapsing_header(imstr!("Channel 3"), imstr!("sc3_clpshr"), true, false) {
			debugger.sound_channel_3_plot.render_lines()
		}

		if imgui::collapsing_header(imstr!("Channel 4"), imstr!("sc4_clpshr"), true, false) {
			debugger.sound_channel_4_plot.render_lines();
		}

		if imgui::collapsing_header(imstr!("Channel A"), imstr!("sca_clpshr"), true, false) {
			debugger.sound_channel_a_plot.render_lines();
		}

		if imgui::collapsing_header(imstr!("Channel B"), imstr!("scb_clpshr"), true, false) {
			debugger.sound_channel_b_plot.render_lines();
		}
		imgui::end();
	}
}

pub fn render_timer_register(gba: &mut Gba, timer_index: usize, low: ioreg::IORegister16, high: ioreg::IORegister16) {
	use rust_imgui::ImGuiSelectableFlags_SpanAllColumns;

	if imgui::tree_node(imstr!("Timer {}", timer_index)) {
		imgui::columns(2, imstr!("timer_{}_table", timer_index), true);
		imgui::selectable_fl(imstr!("TIMER{}CNT_L", timer_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("TIMER{}CNT_H", timer_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Reload"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Prescaler"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Count-Up"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("IRQ-Enabled"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Operate"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Counter"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("~Unscaled Counter"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::next_column();
		imgui::text(imstr!("{:04X}", gba.cpu.memory.get_reg(low)));
		let high = gba.cpu.memory.get_reg(high);
		imgui::text(imstr!("{:04X}", high));
		{
			let timer = &gba.cpu.memory.internal_regs.timers[timer_index];
			imgui::text(imstr!("{:04X}", timer.reload));

			let prescaler_n = high & 0x3;
			imgui::text(imstr!("{} (1/{})", prescaler_n, 1 << timer.prescaler));

			imgui::text(imstr!("{}", timer.count_up));
			imgui::text(imstr!("{}", timer.irq_enabled));
			imgui::text(imstr!("{}", timer.operate));
			imgui::text(imstr!("{:04X}", timer.counter));
			imgui::text(imstr!("{:04X}", timer.unscaled_counter));
		}
		imgui::columns(1, imstr!("timer_{}_table_end", timer_index), false);
		imgui::tree_pop();
	}
}


pub fn render_dma_register(gba: &mut Gba, channel_index: usize, low: ioreg::IORegister16, high: ioreg::IORegister16, sad: ioreg::IORegister32, dad: ioreg::IORegister32) {
	use rust_imgui::ImGuiSelectableFlags_SpanAllColumns;

	if imgui::tree_node(imstr!("DMA{}", channel_index)) {
		imgui::columns(2, imstr!("dma_{}_table", channel_index), true);
		imgui::selectable_fl(imstr!("DMA{}CNT_L", channel_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("DMA{}CNT_H", channel_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("DMA{}SAD", channel_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("DMA{}DAD", channel_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Reload"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Repeat"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Transfer Len"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Gamepak DRQ"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Timing"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("IRQ"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Enabled"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Dest Inc"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Source Inc"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Repeat"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Units"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Dest Addr"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Source Addr"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("Units Remaining"), ImGuiSelectableFlags_SpanAllColumns);
		imgui::next_column();
		imgui::text(imstr!("{:04X}", gba.cpu.memory.get_reg(low)));
		imgui::text(imstr!("{:04X}", gba.cpu.memory.get_reg(high)));
		imgui::text(imstr!("{:08X}", gba.cpu.memory.get_reg(sad)));
		imgui::text(imstr!("{:08X}", gba.cpu.memory.get_reg(dad)));
		{
			let dma_internal_reg = &gba.cpu.memory.internal_regs.dma_registers[channel_index];
			imgui::text(imstr!("{}", dma_internal_reg.reload));
			imgui::text(imstr!("{}", dma_internal_reg.repeat));
			if dma_internal_reg.transfer_word {
				imgui::text(imstr!("32bit"));
			} else {
				imgui::text(imstr!("16bit"));
			}
			imgui::text(imstr!("{}", dma_internal_reg.gamepak_drq));
			match dma_internal_reg.start_timing {
				0 => imgui::text(imstr!("Immediate")),
				1 => imgui::text(imstr!("VBlank")),
				2 => imgui::text(imstr!("HBlank")),
				3 => imgui::text(imstr!("Special")),
				_ => imgui::text(imstr!("??? ({:04X})", dma_internal_reg.start_timing)),
			}
			imgui::text(imstr!("{}", dma_internal_reg.irq));
			imgui::text(imstr!("{}", dma_internal_reg.enabled));
			imgui::text(imstr!("{:+}", dma_internal_reg.dest_addr_inc as i32));
			imgui::text(imstr!("{:+}", dma_internal_reg.source_addr_inc as i32));
			imgui::text(imstr!("{}", dma_internal_reg.is_repeat));
			imgui::text(imstr!("{} ({:04X})", dma_internal_reg.units, dma_internal_reg.units));
			imgui::text(imstr!("{:08X}", dma_internal_reg.destination_addr));
			imgui::text(imstr!("{:08X}", dma_internal_reg.source_addr));
			imgui::text(imstr!("{} ({:04X})", dma_internal_reg.units_remaining, dma_internal_reg.units_remaining));
		}
		imgui::columns(1, imstr!("dma_{}_table_end", channel_index), false);
		imgui::tree_pop();
	}
}

pub trait IntoExt<T>: Sized {
    fn into_ext(self) -> T;
}
impl IntoExt<f32> for i8 { fn into_ext(self) -> f32 { self as f32 } }
impl IntoExt<f32> for u8 { fn into_ext(self) -> f32 { self as f32 } }
impl IntoExt<f32> for i16 { fn into_ext(self) -> f32 { self as f32 } }
impl IntoExt<f32> for u16 { fn into_ext(self) -> f32 { self as f32 } }
impl IntoExt<f32> for i32 { fn into_ext(self) -> f32 { self as f32 } }
impl IntoExt<f32> for u32 { fn into_ext(self) -> f32 { self as f32 } }
impl IntoExt<f32> for f32 { fn into_ext(self) -> f32 { self } }
impl IntoExt<f32> for f64 { fn into_ext(self) -> f32 { self as f32 } }

pub struct DataPlot<T: Clone + Copy + IntoExt<f32>> {
	label: String,
	overlay: String,

	data: Vec<f32>,
	plot_max: f32,
	plot_min: f32,
	max_size: usize,
	read_cursor: usize,
	write_cursor: usize,

	skip: usize,
	skipped: usize,

	_phantom: PhantomData<T>
}

impl<T: Clone + Copy + IntoExt<f32>> DataPlot<T> {
	pub fn new<A: Into<String>, B: Into<String>>(label: A, overlay: B, max_size: usize, plot_min: T, plot_max: T) -> DataPlot<T> {
		Self::with_skip(label, overlay, max_size, plot_min, plot_max, 0)
	}

	pub fn with_skip<A: Into<String>, B: Into<String>>(label: A, overlay: B, max_size: usize, plot_min: T, plot_max: T, skip: usize) -> DataPlot<T> {
		let mut _label = label.into();
		let mut _overlay = overlay.into();

		_label.push('\0');
		_overlay.push('\0');

		let mut ret = DataPlot {
			label: _label,
			overlay: _overlay,
			data: Vec::with_capacity(max_size),
			max_size: max_size,
			read_cursor: 0,
			write_cursor: 0,
			plot_min: plot_min.into_ext(),
			plot_max: plot_max.into_ext(),
			skip: skip,
			skipped: 0,

			_phantom: PhantomData,
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
		let point = point.into_ext();
		self.skipped += 1;
		if self.skipped >= self.skip {
			self.skipped = 0;
			if self.data.len() < self.max_size {
				self.data.push(point);
				self.write_cursor += 1;
				if self.write_cursor >= self.max_size { self.write_cursor = 0; }
			} else {
				if self.write_cursor == self.read_cursor && self.len() > 0 {
					self.read_cursor += 1;
					if self.read_cursor >= self.max_size {
						self.read_cursor = 0;
					}
				}
				self.data[self.write_cursor] = point;
				self.write_cursor += 1;
				if self.write_cursor >= self.max_size { self.write_cursor = 0; }
			}
		}
	}

	fn render_histogram(&self) {
		let _label_imstr = ImStr::from_bytes_unchecked(self.label.as_bytes());
		let _overlay_imstr = ImStr::from_bytes_unchecked(self.overlay.as_bytes());
		imgui::plot_histogram(_label_imstr,
			&self.data,
			self.len(), self.offset(), 
			_overlay_imstr,
			self.plot_min, self.plot_max,
			imgui::vec2(256.0, 128.0), 4);
	}

	fn render_lines(&self) {
		let _label_imstr = ImStr::from_bytes_unchecked(self.label.as_bytes());
		let _overlay_imstr = ImStr::from_bytes_unchecked(self.overlay.as_bytes());
		imgui::plot_lines(_label_imstr,
			&self.data,
			self.len(), self.offset(), 
			_overlay_imstr,
			self.plot_min, self.plot_max,
			imgui::vec2(256.0, 128.0), 4);
	}
}

