use portaudio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::f64::consts::PI;

type FloatType = f64;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
// const SAMPLE_RATE_I: u32 = 44_100;
const PHASE_INC: FloatType = (1.0 / SAMPLE_RATE) as FloatType;
const PHASE_MAX: FloatType = 1.0;
const FRAMES_PER_BUFFER: u32 = 256;
const INTERLEAVED: bool = true;

#[derive(Default)]
struct GbaSquareWave {
	frequency: FloatType, // 64Hz - 131072 Hz (131 KHz)
	duty_cycle: FloatType,
	amplitude: FloatType,
	on: bool
}

#[derive(Default)]
struct GbaNoise {
	on: bool
}

#[derive(Default)]
struct GbaChannels {
	channel1: GbaSquareWave,
	channel2: GbaSquareWave,
	channel4: GbaNoise
}

pub struct AudioDevice {
	running: Arc<AtomicBool>,
	channels: Arc<Mutex<GbaChannels>>,
	output_thread: Option<thread::JoinHandle<()>>
}

impl AudioDevice {
	pub fn new() -> AudioDevice {
		AudioDevice {
			running: Arc::new(AtomicBool::new(true)),
			channels: Arc::new(Mutex::new(Default::default())),
			output_thread: None
		}
	}

	pub fn start(&mut self) {
		{
			let mut channel = self.channels.lock().expect("Failed to aquire channels lock on main thread.");
			channel.channel1.frequency = 1000.0;
			channel.channel1.duty_cycle = 0.5;
			channel.channel1.amplitude = 1.0;
			channel.channel1.on = true;
		}

		let _running = self.running.clone();
		let _channels = self.channels.clone();

		let thread = thread::Builder::new().name("Audio".to_string()).spawn(move || {
			run_port_audio_output_loop(_running, _channels);
		}).expect("Failed to start audio thread.");
		
		self.output_thread = Some(thread);
	}

	pub fn stop(&mut self) {
		self.running.store(false, Ordering::Relaxed);
		debug_trace!("Waiting for audio output thread to stop...");
		match self.output_thread.take() {
			Some(t) => {
				match t.join() {
					Err(e) => debug_error!("Error while waiting for audio thread."),
					_ => {
						debug_trace!("Audio output thread stopped.");
					}
				}
			},
			None => {
				debug_error!("No audio thread to stop.");
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

	return (left, right);
}

fn run_port_audio_output_loop(running: Arc<AtomicBool>, gba_channels: Arc<Mutex<GbaChannels>>) {
	// SETUP:
	let pa = portaudio::PortAudio::new().expect("Failed to initialize port audio.");
	let output_device = pa.default_output_device().expect("Failed to get default output device.");
	let output_info = pa.device_info(output_device).expect("Failed to get output device info.");
	debug_trace!("Default output device info: {:#?}", &output_info);
	let latency = output_info.default_low_output_latency;
	let output_params = portaudio::StreamParameters::<f32>::new(output_device, CHANNELS, INTERLEAVED, latency);
	let settings = portaudio::stream::OutputSettings::new(output_params, SAMPLE_RATE, FRAMES_PER_BUFFER);
	let mut stream = pa.open_blocking_stream(settings)
		.expect("Failed to create PortAudio output stream.");
	debug_info!("Opened GBA audio stream.");

	fn wait_for_stream<F>(f: F) -> u32 where F: Fn() -> Result<portaudio::StreamAvailable, portaudio::error::Error> {
		'waiting_for_stream: loop {
			match f() {
				Ok(available) => match available {
					portaudio::StreamAvailable::Frames(frames) => return frames as u32,
					portaudio::StreamAvailable::InputOverflowed => debug_error!("Input stream has overflowed"),
					portaudio::StreamAvailable::OutputUnderflowed => debug_error!("Output stream has underflowed")
				},
				Err(err) => panic!("An error occurred while waiting for the audio write stream: {}", err)
			}
		}
	};

	stream.start().expect("Failed to start PortAudio stream.");
	debug_info!("Started GBA audio stream.");

	// let mut frames_passed: u32 = 0;

	let mut phase = 0.0;

	'audio_loop: while running.load(Ordering::Relaxed) {
		// debug_trace!("Checking available out frames...");
		let out_frames = wait_for_stream(|| stream.write_available());
		// debug_trace!("{} out frames available.", out_frames);

		if out_frames > 0 {
			let mut channel_lock = match gba_channels.lock() {
				Ok(lock) => lock,
				Err(e) => {
					debug_error!("Failed to acquire channels lock on audio output thread. Error: {}", e);
					break 'audio_loop
				}
			};

			let result = stream.write(out_frames, |output| {
				let mut idx = 0;
				for _ in 0..out_frames {
					let (left, right) = mix_gba_channels(phase, &mut channel_lock);
					output[idx] = left as f32;
					output[idx + 1] = right as f32;
					idx += 2;

					phase += PHASE_INC;
					if phase > PHASE_MAX { phase = 0.0 }
					// debug_trace!("wrote {} to ({}, {})", s, idx, idx + 1);
				}
				// debug_trace!("Wrote {} (>= {})  frames to the output stream.", wrote_fucking_fuck, out_frames);
			});

			match result {
				Err(portaudio::Error::OutputUnderflowed) => debug_warn!("Error while writing to PortAudio output stream: {}", portaudio::Error::OutputUnderflowed),
				Err(e) => panic!("Fatal error while writing to PortAudio output stream: {}", e),
				_ => {}
			}
		}

	}

	match stream.abort() {
		Ok(_) => {
			debug_info!("Stopped GBA audio stream.");
		},
		Err(e) => {
			debug_warn!("Failed to stop GBA Audio Stream. Error: {}", e);
		}
	}

	match stream.close() {
		Ok(_) => {
			debug_info!("Closed GBA audio stream.");
		},
		Err(e) => {
			debug_warn!("Failed to close GBA Audio Stream. Error: {}", e);
		}
	}
}
