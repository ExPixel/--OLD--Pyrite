pub mod frame_counter;
pub mod measure;
pub mod io;
pub mod async_ring_buffer;
pub mod bipbuffer;

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
