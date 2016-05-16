use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::mem;

// #TODO memory orderings and stuff.

pub struct BipBuffer<T> {
	buffer: Vec<T>,

	region_a_start: AtomicUsize, // inclusive
	region_a_end: AtomicUsize, // exclusive

	region_b_start: AtomicUsize, // inclusive
	region_b_end: AtomicUsize, // exclusive

	reserve_in_b: AtomicBool
}

impl<T> BipBuffer<T> {
	pub fn new<F>(length: usize, generator_fn: F) -> BipBuffer<T>
		where F: Fn()->T {
		let mut v = Vec::with_capacity(length);
		for _ in 0..length {
			v.push(generator_fn());
		}

		BipBuffer {
			buffer: v,

			region_a_start: AtomicUsize::new(0),
			region_a_end: AtomicUsize::new(0),

			region_b_start: AtomicUsize::new(0),
			region_b_end: AtomicUsize::new(0),

			reserve_in_b: AtomicBool::new(false)
		}
	}

	pub fn reserve<'a>(&'a mut self, len: usize) -> Result<&'a mut [T], usize> {
		let mut reserve_in_b = self.reserve_in_b.load(Relaxed);
		let mut region_a_start = self.region_a_start.load(Relaxed);

		// I use this for what is practically a goto into reserve_in_b
		loop {
			if reserve_in_b {
				let mut region_b_start = self.region_b_start.load(Relaxed);
				let mut region_b_end = self.region_b_end.load(Relaxed);
				let reservation_start = region_b_start;
				if region_b_end - region_b_start == 0 {
					region_b_start = 0;
					region_b_end = len;
				} else {
					region_b_end += len;
				}

				if region_b_end > region_a_start {
					return Err(region_a_start - reservation_start)
				} else if region_b_end > self.buffer.len() {
					return Err(self.buffer.len() - reservation_start)
				}

				self.region_b_end.store(region_b_end, Relaxed);
				self.region_b_start.store(region_b_start, Relaxed);

				return Ok(&mut self.buffer[reservation_start..region_b_end])
			} else {
				let mut region_a_end = self.region_a_end.load(Relaxed);
				let reservation_start = region_a_start;
				if region_a_end - region_a_start == 0 {
					region_a_start = 0;
					region_a_end = len;
				} else {
					region_a_end += len;
				}

				if region_a_end > self.buffer.len() {
					reserve_in_b = true;
					self.reserve_in_b.store(true, Relaxed);
					continue // GOTO reserve_in_b
				}

				return Ok(&mut self.buffer[reservation_start..region_a_end])
			}
		}
	}

	pub fn commit(&mut self, len: usize) {
		// #TODO I validate absolutely nothing in here, maybe I should.
		let reserve_in_b = self.reserve_in_b.load(Relaxed);
		if reserve_in_b {
			self.region_b_end.fetch_add(len, Relaxed);
		} else {
			self.region_a_end.fetch_add(len, Relaxed);
		}
	}

	pub fn get_block<'a>(&'a mut self) -> Option<&'a mut [T]> {
		let mut region_a_start = self.region_a_start.load(Relaxed);
		let mut region_a_end = self.region_a_end.load(Relaxed);
		if region_a_start < region_a_end {
			Some(&mut self.buffer[region_a_start..region_a_end])
		} else {
			None
		}
	}

	pub fn decommit(&mut self, len: usize) {
		// #TODO I don't check anything in here either.
		let mut region_a_start = self.region_a_start.load(Relaxed);
		let mut region_a_end = self.region_a_end.load(Relaxed);

		region_a_start += len;

		if region_a_start >= region_a_end {
			let mut region_b_start = self.region_b_start.load(Relaxed);
			let mut region_b_end = self.region_b_end.load(Relaxed);

			region_a_start = region_b_start;
			region_a_end = region_b_end;

			self.reserve_in_b.store(false, Relaxed);
			self.region_b_end.store(0, Relaxed);
			self.region_b_start.store(0, Relaxed);
			self.region_a_end.store(region_a_end, Relaxed);
		}

		self.region_a_start.store(region_a_start, Relaxed);
	}

	pub fn buffer_len(&self) -> usize {
		return self.buffer.len()
	}
}