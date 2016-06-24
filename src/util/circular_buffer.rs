pub struct CircularBuffer<T: Clone> {
	inner: Vec<T>,
	writer: usize,
	reader: usize,
	size: usize,
}

impl<T: Clone> CircularBuffer<T> {
	pub fn new(capacity: usize) -> CircularBuffer<T> {
		let mut v = Vec::with_capacity(capacity);
		unsafe { v.set_len(capacity) };
		CircularBuffer {
			inner: v,
			writer: 0,
			reader: 0,
			size: 0,
		}
	}

	pub fn push(&mut self, a: T) {
		if self.writer == self.reader && self.len() > 0 {
			self.reader += 1;
			if self.reader >= self.inner.len() { self.reader = 0; }
		}
		self.inner[self.writer] = a.clone();
		self.writer += 1;
		if self.writer >= self.inner.len() { self.writer = 0; }
		self.size += 1;
		if self.size > self.inner.len() { self.size = self.inner.len() }
	}

	pub fn pop(&mut self) -> T {
		if self.len() > 0 {
			let idx = self.reader;
			let d = self.inner[idx].clone();
			self.reader += 1;
			self.size -= 1;
			if self.reader >= self.inner.len() { self.reader = 0; }
			return d
		}
		panic!("Attempted to pop from empty circular buffer.")
	}

	pub fn len(&self) -> usize {
		self.size
	}
}