//! This module contains things that aren't really part of the gba
//! but are part of the emulator. e.g. the debug module should actually be in
//! here but it's temporary until I can start using a better UI library so
//! it gets to stay where it is for now.

pub mod settings;

use ::util::sync_unsafe_cell::SyncUnsafeCell;

lazy_static! {
	pub static ref PYRITE_SETTINGS: SyncUnsafeCell<settings::PyriteSettings> = SyncUnsafeCell::new(Default::default());
}


#[macro_export]
macro_rules! psetting {
	($setting_name:ident) => ({
		let settings = unsafe {
			::pyrite::PYRITE_SETTINGS.get().as_mut().expect("Failed to get an instance of pyrite settings.")
		};
		settings.$setting_name
	})
}