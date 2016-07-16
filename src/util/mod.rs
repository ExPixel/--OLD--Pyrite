pub mod frame_counter;
pub mod measure;
pub mod io;
pub mod async_ring_buffer;
pub mod circular_buffer;
pub mod sync_unsafe_cell;
pub mod atomic;

macro_rules! max {
	($a:expr, $b:expr) => ({
		let a = $a;
		let b = $b;
		if a > b { a } else { b }
	})
}

macro_rules! min {
	($a:expr, $b:expr) => ({
		let a = $a;
		let b = $b;
		if a < b { a } else { b }
	})
}

macro_rules! percentage {
	($total:expr, $part:expr, $in_type:ty, $out_type:ty) => ({
		let total = $total as $in_type;
		let part = $part as $in_type;
		if part == (0i32 as $in_type) { 0i32 as $out_type }
		else { ((part / total) * (100i32 as $in_type)) as $out_type }
	});

	($total:expr, $part:expr, $in_type:ty) => (
		percentage!($total, $part, $in_type, $in_type)
	);

	($total:expr, $part:expr) => (
		percentage!($total, $part, f64, f64)
	);
}

// These should be in the debug module, but whatever (I'll take time out to refactor this one day):

pub static mut PYRITE_DYN_DEBUG_ENABLED: bool = false;

// (last_start, last_time, total_samples, total_time)
pub static mut PYRITE_MEASURES: [(u64, u64, u64, u64); 16] = [(0u64, 0u64, 0u64, 0u64); 16];

macro_rules! pyrite_measure_start {
	($index:expr) => (
		unsafe {
			::util::PYRITE_MEASURES[$index].0 = ::time::precise_time_ns();
		}
	)
}

macro_rules! pyrite_measure_end {
	($index:expr) => (
		unsafe {
			::util::PYRITE_MEASURES[$index].1 = ::time::precise_time_ns() - ::util::PYRITE_MEASURES[$index].0;
			::util::PYRITE_MEASURES[$index].3 += ::util::PYRITE_MEASURES[$index].1;
			::util::PYRITE_MEASURES[$index].2 += 1;
		}
	)
}

macro_rules! pyrite_measure_print {
	($index:expr) => (
		unsafe {
			println!("------ PYRITE MEASURE [{}] ------", $index);
			println!("Last Time: {}ns", ::util::PYRITE_MEASURES[$index].1);
			println!("Total Samples: {}", ::util::PYRITE_MEASURES[$index].2);
			println!("Total Time: {}ns", ::util::PYRITE_MEASURES[$index].3);
			if ::util::PYRITE_MEASURES[$index].2 > 0 {
				println!("Avg. Time: {:.3}ns", (::util::PYRITE_MEASURES[$index].3 as f64) / (::util::PYRITE_MEASURES[$index].2 as f64));
			}
		}
	)
}

macro_rules! set_pyrite_dyn_debug {
	($value:expr) => (
		unsafe {
			::util::PYRITE_DYN_DEBUG_ENABLED = $value;
		}
	)
}

macro_rules! pyrite_debugging {
	($b:block) => (
		if unsafe {::util::PYRITE_DYN_DEBUG_ENABLED} {
			$b
		}
	);

	($b:block, $c:block) => (
		if unsafe {::util::PYRITE_DYN_DEBUG_ENABLED} {
			$b
		} else {
			$c
		}
	);
}

#[macro_export]
macro_rules! debug_print_vars {
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
macro_rules! debug_print_vars_ln {
	($($x:expr),*) => (
		println!("[DEBUG] [{}:{}] ", file!(), line!());
		$(
			print!("\t");
			print!(stringify!($x));
			println!(" = {}; ", $x);
		)*
	)
}

macro_rules! pd_println {
	($($x:expr),*) => (
		pyrite_debugging!({
			println!("[DEBUG] [{}:{}] ", file!(), line!());
			$(
				print!("\t");
				print!(stringify!($x));
				println!(" = {}; ", $x);
			)*
		});
	)
}

macro_rules! print_memory_table {
	($memory:expr, $start:expr, $end:expr, $columns:expr) => ({
		let columns = $columns;

		let mut cc = 0;
		let mut char_rep: [char; $columns] = ['.'; $columns];

		let is_dot_char = |c: u8| -> bool {
			return c < 32 || c > 126
		};

		for addr in $start..($end + 1) {
			if cc == 0 {
				println!("");
				print!("{:08X}", addr);
			}

			let data = $memory.read8(addr);
			print!(" {:02X}", data);
			char_rep[cc] = data as char;

			cc += 1;
			if cc >= columns {
				print!("  ");
				for cidx in 0..$columns {
					let cr = char_rep[cidx];
					if is_dot_char(cr as u8) { print!("."); }
					else { print!("{}", cr); }
				}
				cc = 0;
			}
		}

		if cc > 0 {
			print!("  ");
			for cidx in 0..min!(cc, $columns) {
				let cr = char_rep[cidx];
				if is_dot_char(cr as u8) { print!("."); }
				else { print!("{}", cr); }
			}
		}

		println!("");
	});
	($memory:expr, $start:expr, $end:expr) => ( print_memory_table!($memory, $start, $end, 16) );
}


pub static mut PYRITE_COUNTERS: [(u64); 16] = [(0u64); 16];

macro_rules! pyrite_counter_inc {
	($counter:expr) => ( pyrite_counter_inc!($counter, 1) );
	($counter:expr, $amt:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] += $amt as u64;
		}
	);
}

macro_rules! pyrite_counter_dec {
	($counter:expr) => ( pyrite_counter_dec!($counter, 1) );
	($counter:expr, $amt:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] -= $amt as u64;
		}
	);
}

macro_rules! pyrite_counter_set {
	($counter:expr, $value:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] = $value as u64;
		}
	);
}

macro_rules! pyrite_counter_get {
	($counter:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter]
		}
	)
}

macro_rules! pyrite_counter_bool {
	($counter:expr) => (
		unsafe {
			::util::PYRITE_COUNTERS[$counter] != 0
		}
	)
}

// Use this to track changing values and only do stuff when they change.
macro_rules! pyrite_counter_diff {
	($counter:expr, $value:expr) => ({
		if pyrite_counter_get!($counter) != $value as u64 {
			pyrite_counter_set!($counter, $value);
			true
		} else {
			false
		}
	})
}


/// GRAPHICS DEBUG STUFF:


////// DEBUG STUFF
pub static mut debug_visible_layers: [bool; 5] = [true; 5];
macro_rules! debug_toggle_layer {
	($layer: expr) => ({
		unsafe {
			::util::debug_visible_layers[$layer] = !::util::debug_visible_layers[$layer]
		}
		let layer_name = match $layer {
			0 => "BG 0",
			1 => "BG 1",
			2 => "BG 2",
			3 => "BG 3",
			4 => "OBJ",
			_ => panic!("BAD LAYER")
		};
		if unsafe { ::util::debug_visible_layers[$layer] } {
			println!("TURNED ON LAYER \"{}\"", layer_name);
		} else {
			println!("TURNED OFF LAYER \"{}\"", layer_name);
		}
	})
}

macro_rules! debug_turn_off_all_layers {
	() => ({
		unsafe {
			for layer in 0..5 { ::util::debug_visible_layers[layer] = false }
		}
		println!("TURNED OFF ALL LAYERS");
	})
}

macro_rules! debug_turn_on_all_layers {
	() => ({
		unsafe {
			for layer in 0..5 { ::util::debug_visible_layers[layer] = true }
		}
		println!("TURNED ON ALL LAYERS");
	})
}

macro_rules! debug_layer_on {
	($layer: expr) => (
		unsafe { ::util::debug_visible_layers[$layer] }
	)
}

// DEBUGGER LOGGING:
#[macro_export]
macro_rules! console_log_with_color {
	($color:expr, $message:expr, $($arg:tt)+) => (
		console_log_with_color!($color, format!($message, $($arg)+));
	);

	($color:expr, $message:expr) => (
		::debug::debugger::get_debugger().console.log($color, $message.into());
	);
}

#[macro_export]
macro_rules! console_log {
	($message:expr, $($arg:tt)+) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_NORMAL, $message, $($arg)+);
	);

	($message:expr) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_NORMAL, $message);
	);
}

#[macro_export]
macro_rules! console_warn {
	($message:expr, $($arg:tt)+) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_WARNING, $message, $($arg)+);
	);

	($message:expr) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_WARNING, $message);
	);
}

#[macro_export]
macro_rules! console_error {
	($message:expr, $($arg:tt)+) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_ERROR, $message, $($arg)+);
	);

	($message:expr) => (
		console_log_with_color!(::debug::debugger::CONSOLE_COLOR_ERROR, $message);
	);
}

// PROFILER MACROS:
macro_rules! profiler_w_nounsafe {
	() => (
		::debug::profiler::get_write_profiler()
	)
}
macro_rules! profiler_begin {
	($name:expr) => (
		unsafe {
			profiler_w_nounsafe!().begin($name)
		}
	)
}

macro_rules! profiler_end {
	() => (
		unsafe {
			profiler_w_nounsafe!().end()
		}
	)
}

macro_rules! profiler_swap {
	() => (
		unsafe {
			::debug::profiler::swap_profilers()
		}
	)
}

macro_rules! profiler_w {
	() => (
		unsafe {
			::debug::profiler::get_write_profiler()
		}
	)
}

macro_rules! profiler_r {
	() => (
		unsafe {
			::debug::profiler::get_read_profiler()
		}
	)
}

macro_rules! profiler_clear {
	() => (
		unsafe {
			profiler_w_nounsafe!().clear()
		}
	)
}

macro_rules! profiler_map {
	($id:expr, $name:expr) => (
		let profiler = unsafe {
			profiler_w_nounsafe!()
		};
		profiler.create_mapped_id($id, $name);
	);
	($name:expr) => (
		profiler_map!($name, $name);
	);
}

macro_rules! profiler_begin_id {
	($id:expr) => (
		let profiler = unsafe {
			profiler_w_nounsafe!()
		};
		profiler.begin_mapped($id)
	)
}

macro_rules! profiler_end_id {
	($id:expr) => (
		let profiler = unsafe {
			profiler_w_nounsafe!()
		};
		profiler.end_mapped($id)
	)
}