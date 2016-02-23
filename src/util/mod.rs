pub mod frame_counter;


// These should be in the debug module, but whatever.

pub static mut PYRITE_DYN_DEBUG_ENABLED: bool = false;

macro_rules! set_pyrite_dyn_debug {
	($value:expr) => (
		unsafe {
			::util::PYRITE_DYN_DEBUG_ENABLED = $value;
		}
	)
}

macro_rules! pyrite_debugging {
	() => (
		unsafe {
			::util::PYRITE_DYN_DEBUG_ENABLED
		}
	)
}

#[macro_export]
macro_rules! debug_print {
	($($x:expr),*) => (
		print!("[DEBUG] [{}:{}] ", file!(), line!());
		$(
			print!(stringify!($x));
			print!(" = {}; ", $x);
		)*
		println!("");
	)
}

#[macro_export]
macro_rules! debug_println {
	($($x:expr),*) => (
		println!("[DEBUG] [{}:{}] ", file!(), line!());
		$(
			print!("\t");
			print!(stringify!($x));
			println!(" = {}; ", $x);
		)*
	)
}

