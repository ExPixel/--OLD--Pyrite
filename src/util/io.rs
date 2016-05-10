use std::fs::File;
use std::io::prelude::*;

pub fn read_file_into_string<'a>(file_path: &'a str) -> Result<String, String> {
	let mut file_string = String::new();
	if let Err(e) = read_file_into_string_buf(file_path, &mut file_string) {
		return Err(e);
	} else {
		return Ok(file_string);
	}
}

pub fn read_file_into_string_buf<'a>(file_path: &'a str, buf: &mut String) -> Result<usize, String> {
	let mut f = match File::open(file_path) {
		Ok(file) => file,
		Err(e) => return Err(format!("Failed to open file `{}`. Error: {}", file_path, e)),
	};

	return match f.read_to_string(buf) {
		Ok(bytes_read) => Ok(bytes_read),
		Err(e) => Err(format!("Failed to read file `{}`. Error: {}", file_path, e))
	}
}

pub fn write_string_into_file<'a>(file_path: &'a str, buf: &'a str) {
	let mut f = File::create(file_path).unwrap();
	write!(f, "{}", buf).unwrap();
	write!(f, "{}", buf).unwrap();
	write!(f, "{}", buf).unwrap();
	write!(f, "{}", buf).unwrap();
}