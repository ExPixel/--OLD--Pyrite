use time;


const NS_IN_SECOND: u64 = 1000000000;

pub struct FrameCounter {
	pub last_time: u64,
	pub delta_frames: u64,
	pub last_delta: u64,
	pub fps_delta: u64,

	pub seconds: u64,
	pub total_fps: u64,
	pub avg_fps: u64,

	pub fps: u64
}

impl FrameCounter {
	pub fn new() -> FrameCounter {
		FrameCounter {
			last_delta: 0,
			last_time: time::precise_time_ns(),
			delta_frames: 0,
			fps_delta: 0,

			seconds: 0,
			total_fps: 0,
			avg_fps: 0,

			fps: 0
		}
	}

	pub fn record_frame(&mut self) {
		self.delta_frames += 1;

		let current_time = time::precise_time_ns();

		let delta = current_time - self.last_time;
		self.last_delta = delta;
		self.last_time = current_time;
		self.fps_delta += delta;

		if self.fps_delta >= NS_IN_SECOND {
			self.fps = self.delta_frames;
			self.delta_frames = 0;
			self.fps_delta = 0;
			self.total_fps += self.fps;
			self.seconds += 1;
			self.avg_fps = self.total_fps / self.seconds;
		}
	}
}