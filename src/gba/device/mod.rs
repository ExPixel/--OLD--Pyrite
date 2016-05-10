//! The device module contains methods for the emualtor
//! to use in order to interface with your actual hardware.

pub mod video;
pub mod audio;
pub mod events;

use self::video::*;
use self::audio::*;
pub use self::events::*;

pub struct GbaDevice {
	pub video: VideoDevice,
	pub audio: AudioDevice
}

impl GbaDevice {
	pub fn new() -> GbaDevice {
		let mut ret = GbaDevice {
			video: VideoDevice::new(),
			audio: AudioDevice::new()
		};
		ret.audio.start();
		return ret;
	}

	pub fn close(&mut self) {
		self.audio.stop();
	}
}