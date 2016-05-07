use time;

#[derive(Copy, Clone)]
pub struct MeasureInfo {
	pub name: &'static str,
	start_time: u64,
	pub total_time: u64,
	pub iterations: u64
}

impl MeasureInfo {
	#[allow(dead_code)]
	fn begin(&mut self) {
		self.start_time = time::precise_time_ns();
	}

	#[allow(dead_code)]
	fn add_it(&mut self, iterations: u64) {
		self.iterations += iterations;
	}

	#[allow(dead_code)]
	fn end(&mut self) {
		let delta_time = time::precise_time_ns() - self.start_time;
		let (ntotal, ov) = self.total_time.overflowing_add(delta_time);
		if ov {
			self.total_time = 0;
			self.iterations = 0;
		} else {
			self.total_time = ntotal;
		}
	}

	pub fn iteration_time_ns(&self) -> f64 {
		if self.total_time == 0 { return 0.0f64 }
		if self.iterations == 0 { return 0.0f64 }
		let _total_time = self.total_time as f64;
		let _iterations = self.iterations as f64;
		return _total_time / _iterations;
	}

	pub fn iteration_time_ms(&self) -> f64 {
		return self.iteration_time_ns() / 1000000.0f64;
	}

	pub fn iteration_time_s(&self) -> f64 {
		return self.iteration_time_ms() / 1000.0f64;
	}
}

macro_rules! new_measure {
    ($name:expr) => (
    	MeasureInfo {
    		name: $name,
    		start_time: 0,
    		total_time: 0,
    		iterations: 0
    	}
    )
}

pub const MEASURE_CPU_TICKS_TIME: usize = 0;
pub const MEASURE_DMA_TICKS_TIME: usize = 1;
pub const MEASURE_TILE_RENDER_TIME: usize = 2;
pub const MEASURE_OBJ_RENDER_TIME: usize = 3;
pub const AVAILABLE_MEASURES: usize = 4;

#[cfg(feature = "measure")]
static mut measures: [MeasureInfo; AVAILABLE_MEASURES] = [
	new_measure!("CPU Cycles"),
	new_measure!("DMA Cycles"),
	new_measure!("Render Text Tiles"),
	new_measure!("Render OBJs")
];

#[cfg(feature = "measure")]
pub fn get_measure_info(measure_index: usize) -> Option<MeasureInfo> {
	if measure_index >= AVAILABLE_MEASURES {
		None
	} else {
		Some(unsafe {
			measures[measure_index]
		})
	}
}

#[cfg(feature = "measure")]
pub fn measure_start(measure_index: usize) {
	unsafe {
		&mut measures[measure_index]
	}.begin();
}

#[cfg(feature = "measure")]
pub fn measure_iteration(measure_index: usize) {
	unsafe {
		&mut measures[measure_index]
	}.add_it(1);
}

#[cfg(feature = "measure")]
pub fn measure_iterations(measure_index: usize, iterations: u64) {
	unsafe {
		&mut measures[measure_index]
	}.add_it(iterations);
}

#[cfg(feature = "measure")]
pub fn measure_end(measure_index: usize) {
	unsafe {
		&mut measures[measure_index]
	}.end();
}

#[cfg(not(feature = "measure"))]
pub fn get_measure_info(_: usize) -> Option<MeasureInfo> {
	return None
}

#[cfg(not(feature = "measure"))]
pub fn measure_start(_:usize) {
	/* NOP */
}

#[cfg(not(feature = "measure"))]
pub fn measure_iteration(_:usize) {
	/* NOP */
}

#[cfg(not(feature = "measure"))]
pub fn measure_iterations(_:usize, _:u64) {
	/* NOP */
}

#[cfg(not(feature = "measure"))]
pub fn measure_end(_:usize) {
	/* NOP */
}