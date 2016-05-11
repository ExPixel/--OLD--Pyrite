use ansi_term::Colour;

pub struct LogLevel(pub u32, pub &'static str, pub Colour);
pub const LOG_LEVEL_ALL: LogLevel = LogLevel(0, "ALL", Colour::Black);
pub const LOG_LEVEL_TRACE: LogLevel = LogLevel(1, "TRACE", Colour::Cyan);
pub const LOG_LEVEL_INFO: LogLevel = LogLevel(2, "INFO", Colour::Blue);
pub const LOG_LEVEL_WARN: LogLevel = LogLevel(3, "WARN", Colour::Yellow);
pub const LOG_LEVEL_ERROR: LogLevel = LogLevel(4, "ERROR", Colour::Red);
pub const LOG_LEVEL_NONE: LogLevel = LogLevel(5, "NONE", Colour::Black);

pub const LOG_OUTPUT_MIN_LEVEL: LogLevel = LOG_LEVEL_ALL;

macro_rules! debug_log {
	($level:expr, $message:expr, $($arg:tt)+) => (
		debug_log!($level, format!($message, $($arg)+));
	);

	($level:expr, $message:expr) => (
		if $level.0 >= ::debug::logging::LOG_OUTPUT_MIN_LEVEL.0 {
			println!("{}", $level.2.paint(format!("[{}] [{}:{}] {}", $level.1, file!(), line!(), 
				$message)));
		}
	);
}

macro_rules! debug_trace {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::logging::LOG_LEVEL_TRACE, $message, $($arg)+);
	);

	($message:expr) => (
		debug_log!(::debug::logging::LOG_LEVEL_TRACE, $message);
	);
}

macro_rules! debug_info {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::logging::LOG_LEVEL_INFO, $message, $($arg)*);
	);

	($message:expr) => (
		debug_log!(::debug::logging::LOG_LEVEL_INFO, $message);
	);
}

macro_rules! debug_warn {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::logging::LOG_LEVEL_WARN, $message, $($arg)*);
	);

	($message:expr) => (
		debug_log!(::debug::logging::LOG_LEVEL_WARN, $message);
	);
}

macro_rules! debug_error {
	($message:expr, $($arg:tt)+) => (
		debug_log!(::debug::logging::LOG_LEVEL_ERROR, $message, $($arg)*);
	);

	($message:expr) => (
		debug_log!(::debug::logging::LOG_LEVEL_ERROR, $message);
	);
}