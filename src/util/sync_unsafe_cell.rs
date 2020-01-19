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

	pub unsafe fn replace(&self, new_value: T) -> T {
		use std::mem;
		let mut ret = new_value;
		mem::swap(&mut ret, self._inner.get().as_mut().expect("Failed to get inner of unsafe cell."));
		return ret;
	}

	pub fn get(&self) -> *mut T {
		return self._inner.get()
	}

	pub unsafe fn get_mut_fast<'a>(&self) -> &'a mut T {
		use std::mem;
		mem::transmute(self.get())
	}

	pub unsafe fn into_inner(self) -> T {
		return self._inner.into_inner()
	}
}

unsafe impl<T> Sync for SyncUnsafeCell<T> {}