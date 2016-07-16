//! This module contains things that aren't really part of the gba
//! but are part of the emulator. e.g. the debug module should actually be in
//! here but it's temporary until I can start using a better UI library so
//! it gets to stay where it is for now.

pub mod settings;

use ::util::sync_unsafe_cell::SyncUnsafeCell;

lazy_static! {
	pub static ref PYRITE_SETTINGS: SyncUnsafeCell<settings::PyriteSettings> = SyncUnsafeCell::new(Default::default());
}

pub fn load_settings() {
	unsafe {
		let mut settings = settings::PyriteSettings::load();
		settings.init();
		PYRITE_SETTINGS.replace(settings);
	}
}

pub fn unsaved_changes() -> bool {
	unsafe {
		let settings = ::pyrite::PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.");
		settings.changed
	}
}

pub fn settings_changed() {
	unsafe {
		let mut settings = ::pyrite::PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.");
		settings.changed = true;
	}
}

pub fn get_settings() -> &'static mut settings::PyriteSettings {
	unsafe {
		PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.")
	}
}

#[macro_export]
macro_rules! psetting {
	($setting_name:ident, $setting_value:expr) => ({
		let mut settings = unsafe { ::pyrite::PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.") };
		settings.$setting_name = $setting_value;
		settings.changed = true;
	});

	($setting_name:ident) => ({
		let settings = unsafe { ::pyrite::PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.") };
		settings.$setting_name
	});
}

#[macro_export]
macro_rules! psetting_ptr {
	($setting_name:ident) => ({
		let mut settings = unsafe {
			::pyrite::PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.")
		};
		&mut settings.$setting_name
	})
}