pub mod armdis;
pub mod debugger;
pub mod bitdesc;

// not ready.
// pub mod armasm;

pub use ::gba::core::memory::GbaMemory;

use self::armdis::*;

pub struct LogLevel(pub u32, pub &'static str);
pub const LOG_LEVEL_TRACE: LogLevel = LogLevel(0, "TRACE");
pub const LOG_LEVEL_INFO: LogLevel = LogLevel(1, "INFO");
pub const LOG_LEVEL_WARN: LogLevel = LogLevel(2, "WARN");
pub const LOG_LEVEL_ERROR: LogLevel = LogLevel(3, "ERROR");

pub const LOWEST_LEVEL: LogLevel = LOG_LEVEL_TRACE;

macro_rules! debug_log {
	($level:expr, $message:expr, $($arg:tt)+) => (
		debug_log!($level, format!($message, $($arg)+));
	);

	($level:expr, $message:expr) => (
		if $level.0 >= ::debug::LOWEST_LEVEL.0 {
			println!("[{}] [{}:{}] {}", $level.1, file!(), line!(), $message);
		}
	);
}

macro_rules! debug_trace {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::LOG_LEVEL_TRACE, $message, $($arg)+);
	);

	($message:expr) => (
		debug_log!(::debug::LOG_LEVEL_TRACE, $message);
	);
}

macro_rules! debug_info {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::LOG_LEVEL_INFO, $message, $($arg)*);
	);

	($message:expr) => (
		debug_log!(::debug::LOG_LEVEL_INFO, $message);
	);
}

macro_rules! debug_warn {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::LOG_LEVEL_WARN, $message, $($arg)*);
	);

	($message:expr) => (
		debug_log!(::debug::LOG_LEVEL_WARN, $message);
	);
}

macro_rules! debug_error {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::LOG_LEVEL_ERROR, $message, $($arg)*);
	);

	($message:expr) => (
		debug_log!(::debug::LOG_LEVEL_ERROR, $message);
	);
}

pub fn print_gba_rom_disasm(memory: &mut GbaMemory, thumb_mode: bool) {
	if thumb_mode {
		print_gba_rom_disasm_thumb(memory);
	} else {
		print_gba_rom_disasm_arm(memory);
	}
}

fn print_gba_rom_disasm_arm(memory: &mut GbaMemory) {
	let mut buffer = String::new();
	let mut offset = 0x8000000u32;
	let len = memory.rom.len() as u32;
	let max = offset + len;

	while offset < max {
		buffer.clear();
		disasm_arm_into(&mut buffer, offset, memory, DIS_WRITE_ALL);
		println!("{}", buffer);
		offset += 4;
	}
}

fn print_gba_rom_disasm_thumb(memory: &mut GbaMemory) {
	let mut buffer = String::new();
	let mut offset = 0x8000000u32;
	let len = memory.rom.len() as u32;
	let max = offset + len;

	while offset < max {
		buffer.clear();
		disasm_thumb_into(&mut buffer, offset, memory, DIS_WRITE_ALL);
		println!("{}", buffer);
		offset += 2;
	}
}