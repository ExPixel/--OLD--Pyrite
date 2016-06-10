// Copyright (c) 2015 The imgui-rs Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::borrow::Cow;
use std::convert::From;
use libc::c_char;

#[macro_export]
macro_rules! imstr {
	($e:tt) => ({
		let value = concat!($e, "\0");
		$crate::imgui::imstr::ImStr::from_bytes_unchecked(value.as_bytes())
	});
	($e:tt, $($arg:tt)*) => ({
		$crate::imgui::imstr::ImStr::from(format!($e, $($arg)*))
	})
}

#[derive(Clone)]
pub struct ImStr<'a> {
	bytes: Cow<'a, [u8]>
}

impl<'a> ImStr<'a> {
	pub fn from_bytes_unchecked(bytes: &'a [u8]) -> ImStr<'a> {
		ImStr {
			bytes: Cow::Borrowed(bytes)
		}
	}
	pub fn as_ptr(&self) -> *const c_char { self.bytes.as_ptr() as *const c_char }

	pub fn begin(&self) -> *const c_char {
		self.as_ptr()
	}

	pub fn end(&self) -> *const c_char {
		unsafe {
			let len = self.bytes.len();
			self.as_ptr().offset(len as isize)
		}
	}
}

impl<'a> From<&'a str> for ImStr<'a> {
	fn from(value: &'a str) -> ImStr<'a> {
		let mut bytes: Vec<u8> = value.bytes().collect();
		bytes.push(0);
		ImStr {
			bytes: Cow::Owned(bytes)
		}
	}
}

impl From<String> for ImStr<'static> {
	fn from(mut value: String) -> ImStr<'static> {
		value.push('\0');
		ImStr {
			bytes: Cow::Owned(value.into_bytes())
		}
	}
}