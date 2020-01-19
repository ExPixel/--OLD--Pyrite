// #TODO I will have to do serialization and deserialization by hand,
//       somehow because if settings change and I try to oad from the file
//       of an old settings format then things get kind of weird.

use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;

use std::sync::Arc;
use ::util::atomic::Atomic;

lazy_static! {
	pub static ref ATOMIC_MASTER_VOLUME: Arc<Atomic<f32>> = Arc::new(Atomic::new(0.0));
}

const SETTINGS_SAVE_LOCATION: &'static str = "data/settings.json";

#[derive(RustcEncodable, RustcDecodable)]
pub struct PyriteSettings {
	// UI SETTINGS:
	pub window_width: u32,
	pub window_height: u32,

	// SOUND SETTINGS:
	pub master_volume: f32,
	pub sound_enabled: bool,
	pub channel1_enabled: bool,
	pub channel2_enabled: bool,
	pub channel3_enabled: bool,
	pub channel4_enabled: bool,
	pub channela_enabled: bool,
	pub channelb_enabled: bool,

	pub changed: bool,
}

impl PyriteSettings {
	pub fn init(&mut self) {
		self.commit_volume();
	}

	pub fn commit_volume(&self) {
		use std::sync::atomic::Ordering;
		ATOMIC_MASTER_VOLUME.store(self.master_volume, Ordering::Relaxed);
	}

	/// Saves only if there are changes to save.
	pub fn save_changes(&mut self) -> bool {
		if self.changed {
			self.save();
			return true;
		} else {
			return false;
		}
	}

	pub fn save(&mut self) {
		self.changed = false;
		let mut save_file = File::create(SETTINGS_SAVE_LOCATION).expect("Failed to open save file for writing.");
		match json::encode(self) {
			Ok(json) => write!(save_file, "{}", json).expect("Failed to write json to file."),
			Err(e) => panic!("Failed to serialize settings. ERROR: {}", e),
		}
	}

	pub fn load() -> PyriteSettings {
		if let Ok(mut file) = File::open(SETTINGS_SAVE_LOCATION) {
			let mut buffer = String::new();
			let ret = match file.read_to_string(&mut buffer) {
				Ok(_) => json::decode(&buffer).unwrap_or_else(|_| Default::default()),
				Err(e) => {
					console_error!("Error while reading settings: {}", e);
					Default::default()
				},
			};
			return ret
		} else {
			Default::default()
		}
	}
}

impl Default for PyriteSettings {
	fn default() -> PyriteSettings {
		PyriteSettings {
			// UI SETTINGS:
			window_width: 240 * 3,
			window_height: 160 * 3,

			// SOUND SETTINGS
			master_volume: 1.0,
			sound_enabled: true,
			channel1_enabled: true,
			channel2_enabled: true,
			channel3_enabled: true,
			channel4_enabled: true,
			channela_enabled: true,
			channelb_enabled: true,

			changed: false,
		}
	}
}