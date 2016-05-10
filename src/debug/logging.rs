pub struct LogLevel(pub u32, pub &'static str);
pub const LOG_LEVEL_ALL: LogLevel = LogLevel(0, "ALL");
pub const LOG_LEVEL_TRACE: LogLevel = LogLevel(1, "TRACE");
pub const LOG_LEVEL_INFO: LogLevel = LogLevel(2, "INFO");
pub const LOG_LEVEL_WARN: LogLevel = LogLevel(3, "WARN");
pub const LOG_LEVEL_ERROR: LogLevel = LogLevel(4, "ERROR");
pub const LOG_LEVEL_NONE: LogLevel = LogLevel(5, "NONE");

pub const LOG_OUTPUT_MIN_LEVEL: LogLevel = LOG_LEVEL_ALL;

macro_rules! debug_log {
	($level:expr, $message:expr, $($arg:tt)+) => (
		debug_log!($level, format!($message, $($arg)+));
	);

	($level:expr, $message:expr) => (
		if $level.0 >= ::debug::logging::LOG_OUTPUT_MIN_LEVEL.0 {
			println!("[{}] [{}:{}] {}", $level.1, file!(), line!(), $message);
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