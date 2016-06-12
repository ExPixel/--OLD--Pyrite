use rust_imgui as imgui;
use ::gba::Gba;
use ::gba::core::memory::*;
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
	pub emulator_delay_plot: DataPlot<f32>,

	// SOUND:
	pub sound_info_window: bool,
	pub sound_channel_1_plot: DataPlot<f32>,
	pub sound_channel_2_plot: DataPlot<f32>,
	pub sound_channel_3_plot: DataPlot<f32>,
	pub sound_channel_4_plot: DataPlot<f32>,
	pub sound_channel_a_plot: DataPlot<f32>,
	pub sound_channel_b_plot: DataPlot<f32>,
	pub sound_plot: DataPlot<f32>,

	ioreg_window: bool,
}

impl DebugData {
	pub fn new() -> DebugData {
		DebugData {
			frame_build_time: 0.0,
			frame_render_time: 0.0,
			full_frame_time: 0.0,
			emulator_performance_opened: false,
			emulator_delay_plot: DataPlot::new(64, 0.0, 100.0),

			sound_info_window: false,
			sound_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_1_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_2_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_3_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_4_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_a_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),
			sound_channel_b_plot: DataPlot::with_skip(128, -32768.0, 32767.0, 16),

			ioreg_window: false,
		}
	}
}

pub fn render_debugger(gba: &mut Gba) {
	use rust_imgui::ImGuiSelectableFlags_SpanAllColumns;

	let debugger = get_debugger();


	// Debugging:
	{
		imgui::text(imstr!("A timer: {}", gba.cpu.memory.internal_regs.audio_fifo_a.timer));
		imgui::text(imstr!("B timer: {}", gba.cpu.memory.internal_regs.audio_fifo_b.timer));
		imgui::text(imstr!("freq A: {}", gba.cpu.memory.internal_regs.audio_fifo_a.frequency));
		imgui::text(imstr!("freq B: {}", gba.cpu.memory.internal_regs.audio_fifo_b.frequency));
		imgui::text(imstr!("freq A inc: {}", gba.cpu.memory.internal_regs.audio_fifo_a.freq_inc));
		imgui::text(imstr!("freq B inc: {}", gba.cpu.memory.internal_regs.audio_fifo_a.freq_inc));
	}

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

		if imgui::menu_item(imstr!("IO Registers")) {
			debugger.ioreg_window = true;
		}

		imgui::end_popup();
	}

	if debugger.emulator_performance_opened {
		imgui::begin(imstr!("Emulator Performance"), &mut debugger.emulator_performance_opened, imgui::ImGuiWindowFlags_None);
		imgui::text(imstr!("Frame Build Time: {:.2}ms", debugger.frame_build_time));
		imgui::text(imstr!("Frame Render Time: {:.2}ms", debugger.frame_render_time));
		imgui::text(imstr!("Frame Time: {:.2}ms", debugger.full_frame_time));
		
		imgui::plot_histogram(
			imstr!("Frame Delay"),
			&debugger.emulator_delay_plot.data,
			debugger.emulator_delay_plot.len(), debugger.emulator_delay_plot.offset(), 
			imstr!("Delay"), 
			debugger.emulator_delay_plot.plot_min, debugger.emulator_delay_plot.plot_max,
			imgui::vec2(256.0, 128.0), 4
		);

		if !imgui::is_item_hovered() {
			debugger.emulator_delay_plot.plot(debugger.frame_build_time as f32);	
		}

		imgui::end();
	}

	if debugger.ioreg_window {
		imgui::begin(imstr!("IO Registers"), &mut debugger.ioreg_window, imgui::ImGuiWindowFlags_None);
		if imgui::collapsing_header(imstr!("DMA"), imstr!("dma_ioreg_clpshr"), true, false) {
			render_dma_register(gba, 0, ioreg::DMA0CNT_L, ioreg::DMA0CNT_H);
			render_dma_register(gba, 1, ioreg::DMA1CNT_L, ioreg::DMA1CNT_H);
			render_dma_register(gba, 2, ioreg::DMA2CNT_L, ioreg::DMA2CNT_H);
			render_dma_register(gba, 3, ioreg::DMA3CNT_L, ioreg::DMA3CNT_H);
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


pub fn render_dma_register(gba: &mut Gba, channel_index: usize, low: ioreg::IORegister16, high: ioreg::IORegister16) {
	use rust_imgui::ImGuiSelectableFlags_SpanAllColumns;

	if imgui::tree_node(imstr!("DMA{}", channel_index)) {
		imgui::columns(2, imstr!("dma_{}_table", channel_index), true);
		imgui::selectable_fl(imstr!("DMA{}CNT_L", channel_index), ImGuiSelectableFlags_SpanAllColumns);
		imgui::selectable_fl(imstr!("DMA{}CNT_H", channel_index), ImGuiSelectableFlags_SpanAllColumns);
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

/*
pub struct DMAInternalReg {
	pub reload: bool,
	pub repeat: bool,
	pub transfer_word: bool, // transfers halfwords if false
	pub gamepak_drq: bool,  // #TODO I'm not even sure what this is.
	pub start_timing: u16, // (0=Immediately, 1=VBlank, 2=HBlank, 3=Special)
	pub irq: bool,
	pub enabled: bool,

	pub dest_addr_inc: u32,
	pub source_addr_inc: u32,

	// Everything below here is set and controlled by dma.rs:
	pub is_repeat: bool,
	pub units: u32,
	pub original_destination_addr: u32,
	pub destination_addr: u32,
	pub source_addr: u32,
	pub units_remaining: u32,
	pub first_transfer: bool
}
*/

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