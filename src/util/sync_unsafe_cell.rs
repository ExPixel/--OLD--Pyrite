use std::cell::UnsafeCell;

pub struct SyncUnsafeCell<T> {
	_inner: UnsafeCell<T>
}

impl<T> SyncUnsafeCell<T> {
	pub fn new(data: T) -> SyncUnsafeCell<T> {
		SyncUnsafeCell {
			_inner: UnsafeCell::new(data)
		}
	}

	pub fn get(&self) -> *mut T {
		return self._inner.get()
	}

	pub unsafe fn into_inner(self) -> T {
		return self._inner.into_inner()
	}
}

unsafe impl<T> Sync for SyncUnsafeCell<T> {}