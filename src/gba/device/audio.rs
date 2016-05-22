use portaudio;
use std::thread;
use std::sync::Arc;
use ::util::async_ring_buffer::AsyncRingBuffer;
use std;
// use std::f64::consts::PI;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 256;
const AUDIO_DATA_BUFFER_SIZE: usize = FRAMES_PER_BUFFER as usize;
const AUDIO_DATA_BUFFER_COUNT: usize = 8;

// Size of audio data buffer in bytes is around:
// AUDIO_DATA_BUFFER_SIZE * AUDIO_DATA_BUFFER_COUNT * 8

pub type AudioBufferType = [(i16, i16); AUDIO_DATA_BUFFER_SIZE as usize];

pub struct AudioDevice {
	pub ring_buffer: Arc<AsyncRingBuffer<AudioBufferType>>,
	output_thread: Option<thread::JoinHandle<()>>,
	pub sample_rate: u32,
	pub sample_rate_f: f32
}

impl AudioDevice {
	pub fn new() -> AudioDevice {
		let generator_fn = || [(std::i16::MIN, std::i16::MIN); AUDIO_DATA_BUFFER_SIZE as usize];
		AudioDevice {
			ring_buffer: Arc::new(AsyncRingBuffer::new(AUDIO_DATA_BUFFER_COUNT, generator_fn)),
			output_thread: None,
			sample_rate: 44_100,
			sample_rate_f: 44_100.0
		}
	}

	#[inline(always)]
	pub fn millis_to_frames(&self, milliseconds: u32, millisecond_tenths: u32) -> u32 {
		// 1 / 44_100 s - sample
		// 1 / 1000 s - milliseconds
		return milliseconds * 44 + millisecond_tenths * 4;
	}

	pub fn start(&mut self) {
		let ring_buffer = self.ring_buffer.clone();
		let thread = thread::Builder::new().name("Audio".to_string()).spawn(move || {
			start_port_audio(ring_buffer);
		}).expect("Failed to start audio thread.");
		self.output_thread = Some(thread);
	}

	pub fn stop(&mut self) {
		debug_trace!("Waiting for audio output thread to stop...");
		match self.output_thread.take() {
			Some(t) => {
				t.thread().unpark();
				debug_trace!("Unparked PortAudio thread.");
				match t.join() {
					// #FIXME the type of the error is Any + Send so I can't display it. What do?
					Err(_) => debug_error!("Error while waiting for audio thread."),
					_ => {
						debug_trace!("Audio output thread stopped.");
					}
				}
			},
			None => {
				debug_warn!("No audio thread to stop.");
			}
		}
	}
}

/// Hearing is logarithmic or something or other,
/// so just multiplying our signal by 1/10 won't translate
/// exactly to 1/10 of perceived volume.
pub fn volume_to_signal_multiplier(volume: f32) -> f32 {
	let level_change = 10.0f32 * volume.log2();
	let sound_pressure = (10.0f32).powf(level_change / 20.0);
	return sound_pressure;
}

fn start_port_audio(ring_buffer: Arc<AsyncRingBuffer<AudioBufferType>>) {
	// SETUP:
	let pa = portaudio::PortAudio::new().expect("Failed to initialize port audio.");
	let settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
		.expect("Failed to get PortAudio default output stream settings.");
	// settings.flags = portaudio::stream_flags::CLIP_OFF;
	// let mut volume_multiplier = volume_to_signal_multiplier(0.2);
	let mut remaining_audio_data_index = 0;

	let mut last_left = 0;
	let mut last_right = 0;

	let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {
		let mut idx = 0;
		let buffer_len = frames * 2;


		let mut continue_reading = true;
		while continue_reading {
			let read_status = ring_buffer.try_read(|audio_data| {
				while remaining_audio_data_index < AUDIO_DATA_BUFFER_SIZE {
					let (left, right) = audio_data[remaining_audio_data_index];

					last_left = left;
					last_right = right;

					buffer[idx] = left;
					buffer[idx + 1] = right;

					idx += 2;
					remaining_audio_data_index += 1;

					if idx >= buffer_len {
						continue_reading = false;
						if remaining_audio_data_index < AUDIO_DATA_BUFFER_SIZE {
							// Didn't finish reading this buffer but the device
							// doesn't require anymore frames at the moment.
							return false;
						} else {
							remaining_audio_data_index = 0;
							// Finished reading this buffer and the device doesn't
							// require anymore frames.
							return true;
						}
					}
				}
				remaining_audio_data_index = 0;
				return true;
			});
			continue_reading &= read_status;
		}

		while idx < buffer_len {
			buffer[idx] = last_left;
			buffer[idx + 1] = last_right;
			idx += 2;
		}

		return portaudio::Continue;
	};

	let mut stream = pa.open_non_blocking_stream(settings, callback)
		.expect("Failed to create PortAudio output stream.");
	debug_info!("Opened PortAudio stream.");

	match stream.start() {
		Ok(_) => {
			debug_info!("Started PortAudio output stream");
		},
		Err(e) => {
			debug_error!("Failed to start PortAudio output stream. Error: {}", e);
			return
		}
	}

	debug_trace!("Parked PortAudio thread.");
	thread::park(); // And now we wait...

	match stream.abort() {
		Ok(_) => {
			debug_info!("Stopped PortAudio stream.");
		},
		Err(e) => {
			debug_warn!("Failed to stop PortAudio Stream. Error: {}", e);
		}
	}

	match stream.close() {
		Ok(_) => {
			debug_info!("Closed PortAudio stream.");
		},
		Err(e) => {
			debug_warn!("Failed to close GBA Audio Stream. Error: {}", e);
		}
	}
}
