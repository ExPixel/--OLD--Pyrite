#[derive(RustcEncodable, RustcDecodable)]
pub struct PyriteSettings {
	// SOUND SETTINGS:
	pub sound_enabled: bool,
	pub channel1_enabled: bool,
	pub channel2_enabled: bool,
	pub channel3_enabled: bool,
	pub channel4_enabled: bool,
	pub channela_enabled: bool,
	pub channelb_enabled: bool,
}

// #TODO add volume.
pub struct PyriteSoundSettings {
}

impl Default for PyriteSettings {
	fn default() -> PyriteSettings {
		PyriteSettings {
			// SOUND SETTINGS
			sound_enabled: true,
			channel1_enabled: true,
			channel2_enabled: true,
			channel3_enabled: true,
			channel4_enabled: true,
			channela_enabled: true,
			channelb_enabled: true,
		}
	}
}