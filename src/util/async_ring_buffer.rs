use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::marker::Sync;
use std::mem;

/// A ring buffer that allows a single producer
/// and a single consumer possibly on different threads.
pub struct AsyncRingBuffer<T> {
	/// The index used when selecting the reader's chunk.
	/// The writer should be fast enough that this
	/// doesn't catch up to the writer's cursor.
	reader_cursor: AtomicUsize,

	/// The index used when selecting
	/// the writer's chunk. This should be ahead of
	/// the reader's chunk as much as possible to avoid
	/// blocking.
	writer_cursor: AtomicUsize,

	// #TODO remove these
	// these are just some variables I'm using to track
	// how often the reader is catching up the the writer
	// and vice versa.
	pub _stat_read_misses: AtomicUsize,
	pub _stat_write_misses: AtomicUsize,

	/// The data being shared between the reader and the writer.
	data: Vec<T>
}

impl<T> AsyncRingBuffer<T> {
	// #NOTE should I force the length to be a power of two 
	//       in case of an overflow if I ever use this code somewhere else?
	pub fn new<F>(length: usize, generator_fn: F) -> AsyncRingBuffer<T>
		where F: Fn() -> T {
		if length < 2 {
			panic!("The async ring buffer must have a length of at least 2! length: {}", length);
		}

		let mut ret = AsyncRingBuffer {
			reader_cursor: AtomicUsize::new(0),
			writer_cursor: AtomicUsize::new(0),

			_stat_read_misses: AtomicUsize::new(0),
			_stat_write_misses: AtomicUsize::new(0),

			data: Vec::with_capacity(length)
		};

		for _ in 0..length {
			ret.data.push(generator_fn());
		}

		return ret;
	}

	// pub fn new_default<D: Default>(length: usize) -> AsyncRingBuffer<D> {
	// 	if length < 2 {
	// 		panic!("The async ring buffer must have a length of at least 2! length: {}", length);
	// 	}

	// 	let mut ret = AsyncRingBuffer {
	// 		reader_cursor: AtomicUsize::new(0),
	// 		writer_cursor: AtomicUsize::new(0),
	// 		data: Vec::with_capacity(length),

	// 		_stat_read_misses: AtomicUsize::new(0),
	// 		_stat_write_misses: AtomicUsize::new(0)
	// 	};

	// 	for _ in 0..length {
	// 		ret.data.push(Default::default());
	// 	}

	// 	return ret;
	// }

	pub fn try_write<F>(&self, mut write_fn: F) -> bool
		where F: FnMut(&mut T) -> bool {
		let cur_writer_cursor = self.writer_cursor.load(Ordering::Acquire);
		let cur_reader_cursor = self.reader_cursor.load(Ordering::Relaxed);
		let writer_idx = cur_writer_cursor % self.data.len();
		let reader_idx = cur_reader_cursor % self.data.len();

		// If the writer index and the reader index are the same,
		// and the writer's cursor is greater than the reader's cursor,
		// that means that writer has "lapped" the reader and we need to wait
		// for the reader to process a piece of data before continuing to write.
		if writer_idx != reader_idx || cur_writer_cursor <= cur_reader_cursor {
			// #FIXME not sure if it's okay to use transmute here like this:
			let item_ptr: *mut T = unsafe { mem::transmute(&self.data[writer_idx]) };
			if write_fn(unsafe {mem::transmute(item_ptr)}) {
				self.writer_cursor.store(cur_writer_cursor + 1, Ordering::Release);
			}
			return true;
		}
		self._stat_write_misses.fetch_add(1, Ordering::Relaxed);
		return false;
	}

	pub fn try_read<F>(&self, mut read_fn: F) -> bool
		where F: FnMut(&T) -> bool {
		let cur_reader_cursor = self.reader_cursor.load(Ordering::Acquire);
		let cur_writer_cursor = self.writer_cursor.load(Ordering::Relaxed);
		let writer_idx = cur_writer_cursor % self.data.len();
		let reader_idx = cur_reader_cursor % self.data.len();

		// If the reader index is the same as the writer index,
		// and the reader's cursor is greater than or equal to the writer's cursor,
		// that means the reader has caught up and it should wait for the writer to produce more
		// data.
		if reader_idx != writer_idx || cur_reader_cursor < cur_writer_cursor {
			// #FIXME not sure if it's okay to use unsafe here like this:
			if read_fn(&self.data[reader_idx]) {
				self.reader_cursor.store(cur_reader_cursor + 1, Ordering::Release);
			}
			return true;
		}
		self._stat_read_misses.fetch_add(1, Ordering::Relaxed);
		return false;
	}
}