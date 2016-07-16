use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::boxed::Box;
use std::ptr;

pub struct Atomic<T: Clone> {
	ptr: AtomicPtr<T>
}

impl<T: Clone> Atomic<T> {
	pub fn new(data: T) -> Atomic<T> {
		Atomic {
			ptr: AtomicPtr::new(Box::into_raw(Box::new(data)))
		}
	}

	pub fn store(&self, data: T, ordering: Ordering) {
		use std::mem;
		let old_data = self.ptr.swap(Box::into_raw(Box::new(data)), ordering);
		if !old_data.is_null() {
			let old_boxed = unsafe { Box::from_raw(old_data) };
			mem::drop(old_boxed);
		}
	}

	pub fn load(&self, ordering: Ordering) -> Option<T> {
		let data = self.ptr.load(ordering);
		if !data.is_null() {
			let data_ref = unsafe { data.as_mut() }.expect("Failed to turn mutable pointer into reference.");
			Some((*data_ref).clone())
		} else {
			None
		}
	}

	pub unsafe fn take(&self, ordering: Ordering) -> Option<Box<T>> {
		let old_data = self.ptr.swap(ptr::null_mut(), ordering);
		if !old_data.is_null() {
			Some(Box::from_raw(old_data))
		} else {
			None
		}
	}
}

impl<T: Clone> Drop for Atomic<T> {
	fn drop(&mut self) {
		let _ = unsafe { self.take(Ordering::Release) }; // We just consume the value of the box and drop it.
	}
}