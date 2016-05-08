use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS: i32 = 4;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

pub type GbaAudioFrame = (f32, f32, f32, f32);

pub struct AudioDevice {
	pa: pa::PortAudio,
	// buffer: [GbaAudioFrame; 64]
}

impl AudioDevice {
	pub fn new() -> AudioDevice {
		let pa = pa::PortAudio::new().expect("Initialize PortAudio.");
		AudioDevice {
			pa: pa
		}
	}

	pub fn open(&mut self) {
		// let mut settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
		// 	.expect("Configure PortAudio.");
		// // we won't output out of range samples so don't bother clipping them.
		// settings.flags = pa::stream_flags::CLIP_OFF;

		// let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
		// }
	}

	pub fn close(&mut self) {
	}
}

// impl Drop for AudioDevice {
// 	pub fn drop(&mut self) {
// 	}
// }