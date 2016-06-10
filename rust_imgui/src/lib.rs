extern crate libc;
#[macro_use] 
extern crate bitflags;

#[macro_use]
pub mod imgui;

pub use imgui::structs::*;
pub use imgui::enums::*;
pub use imgui::api::*;

pub use imgui::raw::types::{
	ImU32,
	ImWchar,
	ImTextureID,
	ImGuiID,
	ImDrawIdx,
	ImGuiTextEditCallback,
	ImGuiSizeConstraintCallback,
	ImDrawCallback
};

pub fn vec2(x: f32, y: f32) -> ImVec2 {
	ImVec2::new(x, y)
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
	ImVec4::new(x, y, z, w)
}

#[cfg(test)]
mod tests {
	use super::imgui::api as imgui;
	use std::ffi::CStr;

	#[test]
	fn filename_check() {
		let io = imgui::get_io();
		let ini_filename = unsafe { CStr::from_ptr(io.ini_filename) };
		let log_filename = unsafe { CStr::from_ptr(io.log_filename) };
		assert_eq!(ini_filename.to_str(), Ok("imgui.ini"));
		assert_eq!(log_filename.to_str(), Ok("imgui_log.txt"));
		imgui::shutdown();
	}

	#[test]
	fn version_check() {
		let version = imgui::get_version();
		assert_eq!(version.to_str(), Ok("1.48")); // Will break whenever the cimgui version changes.
		imgui::shutdown();
	}
}
