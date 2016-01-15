use time;


const NS_IN_SECOND: u64 = 1000000000;

pub struct FrameCounter {
	pub total_frames: u64,
	pub total_time: u64,
	pub last_time: u64,
	pub delta_frames: u64,
	pub fps: u64 // #FIXME make this a u32?
}

impl FrameCounter {
	pub fn new() -> FrameCounter {
		FrameCounter {
			total_frames: 0,
			total_time: 0,
			last_time: time::precise_time_ns(),
			delta_frames: 0,
			fps: 0
		}
	}

	pub fn record_frame(&mut self) {
		self.total_frames += 1;
		self.delta_frames += 1;

		let current_time = time::precise_time_ns();

		let delta = current_time - self.last_time;
		self.total_time += delta;

		if delta >= NS_IN_SECOND {
			self.fps = self.delta_frames;
			self.delta_frames = 0;
			self.last_time = current_time;
			println!("FPS: {} - TOTAL FRAMES: {} - TOTAL NS: {}", self.fps, self.total_frames, self.total_time);
		}
	}
}