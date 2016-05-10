use portaudio;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
// use std::f64::consts::PI;

type FloatType = f32;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
// const SAMPLE_RATE_I: u32 = 44_100;
const PHASE_INC: FloatType = (1.0 / SAMPLE_RATE) as FloatType;
const PHASE_MAX: FloatType = 1.0;
const FRAMES_PER_BUFFER: u32 = 256;
// const INTERLEAVED: bool = true;

#[derive(Default, Copy, Clone)]
pub struct GbaSquareWave {
	frequency: FloatType, // 64Hz - 131072 Hz (131 KHz)
	duty_cycle: FloatType,
	amplitude: FloatType,
	pub on: bool
}

#[derive(Default, Copy, Clone)]
struct GbaNoise {
	on: bool
}

#[derive(Default, Copy, Clone)]
pub struct GbaChannels {
	pub channel1: GbaSquareWave,
	channel2: GbaSquareWave,
	channel4: GbaNoise
}

pub struct AudioDevice {
	pub channels: GbaChannels,
	sender: Option<Sender<GbaChannels>>,
	output_thread: Option<thread::JoinHandle<()>>
}

impl AudioDevice {
	pub fn new() -> AudioDevice {
		AudioDevice {
			channels: Default::default(),
			sender: None,
			output_thread: None
		}
	}

	pub fn start(&mut self) {
		let (tx, rx) = mpsc::channel();

		let thread = thread::Builder::new().name("Audio".to_string()).spawn(move || {
			start_port_audio(rx);
		}).expect("Failed to start audio thread.");
		
		self.sender = Some(tx);
		self.output_thread = Some(thread);

		self.first_send();
	}

	fn first_send(&mut self) {
		self.channels.channel1.frequency = 1000.0;
		self.channels.channel1.duty_cycle = 0.5;
		self.channels.channel1.amplitude = 1.0;
		self.channels.channel1.on = false;
		self.commit();
	}

	// Sends its channel data to the audio output thread.
	pub fn commit(&mut self) {
		if let Some(sender) = self.sender.as_ref() {
			match sender.send(self.channels) {
				Ok(_) => {},
				Err(e) => {
					debug_error!("Error while sending data to the audio output thread. Error: {}", e);
					panic!("Error while sending audio");
				}
			}
		}
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

fn mix_gba_channels(phase: FloatType, channels: &mut GbaChannels) -> (FloatType, FloatType) {
	let mut left = 0.0;
	let mut right = 0.0;

	if channels.channel1.on {
		let period = 1.0 / channels.channel1.frequency;
		let a = (phase / period) % 1.0;
		let b = 2.0 * channels.channel1.duty_cycle;
		let c = a / b;
		let s = if c > 0.5 { -channels.channel1.amplitude } else { channels.channel1.amplitude };

		left = s;
		right = s;
	}

	// if channels.channel2.on {
	// }

	return (left, right);
}

fn start_port_audio(rx: Receiver<GbaChannels>) {
	// SETUP:
	let pa = portaudio::PortAudio::new().expect("Failed to initialize port audio.");
	let mut settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
		.expect("Failed to get PortAudio default output stream settings.");
	settings.flags = portaudio::stream_flags::CLIP_OFF;

	let mut phase = 0.0;
	let mut channels: GbaChannels = Default::default();
	let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {
		match rx.try_recv() {
			Ok(data) => {
				channels = data;
			},
			Err(e) => {
				match e {
					mpsc::TryRecvError::Empty => {}
					mpsc::TryRecvError::Disconnected => {
						debug_error!("PortAudio receiver was disconnected.");
						return portaudio::Abort
					}
				}
			}
		}

		let mut idx = 0;
		for _ in 0..frames {
			let (left, right) = mix_gba_channels(phase, &mut channels);
			buffer[idx] = left as f32;
			buffer[idx + 1] = right as f32;
			idx += 2;
			phase += PHASE_INC;
			if phase > PHASE_MAX { phase = 0.0}
		}
		portaudio::Continue
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
