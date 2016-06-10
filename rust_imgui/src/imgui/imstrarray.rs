use libc::c_char;
use super::imstr::ImStr;

// So this all kind of relies on the values in the Cow pointers not moving
// in ImStr. They're smart pointer doodly bops or what not so I'm not really sure how they work.

#[macro_export]
macro_rules! imstr_array {
	() => {
		::imgui::imstrarray::ImStrArray::new()
	};

	($first:expr $(,$item:expr)*) => ({
		let mut arr = ::imgui::imstrarray::ImStrArray::new();
		arr.add($first);
		$(
			arr.add($item);
		)*
		arr
	});
}

#[macro_export]
macro_rules! imstr_array_r {
	() => {
		::imgui::imstrarray::ImStrArray::new()
	};

	($first:expr $(,$item:expr)*) => ({
		let mut arr = ::imgui::imstrarray::ImStrArray::new();
		arr.add(imstr!($first));
		$(
			arr.add(imstr!($item));
		)*
		arr
	});
}

pub struct ImStrArray<'a> {
	strings: Vec<ImStr<'a>>,
	pointers: Vec<*const c_char>,
}

impl<'a> ImStrArray<'a> {
	pub fn new() -> ImStrArray<'a> {
		ImStrArray {
			strings: Vec::new(),
			pointers: Vec::new()
		}
	}

	pub fn add(&mut self, im: ImStr<'a>) {
		self.pointers.push(im.as_ptr());
		self.strings.push(im);
	}

	pub fn remove(&mut self, idx: usize) {
		self.pointers.remove(idx);
		self.strings.remove(idx);
	}

	pub fn as_mut_ptr(&mut self) -> *mut *const c_char {
		self.pointers.as_mut_ptr()
	}

	pub fn len(&self) -> usize {
		self.pointers.len()
	}

	pub fn len_c(&self) -> i32 {
		self.pointers.len() as i32
	}
}