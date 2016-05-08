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
		GbaDevice {
			video: VideoDevice::new(),
			audio: AudioDevice::new()
		}
	}
}