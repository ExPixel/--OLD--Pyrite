pub mod frame_counter;

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