use std::fs;
use std::path::Path;

use termsize::terminal_size;

mod termsize;

fn filename(path: &Path) -> String {
	if let Some(name) = path.components().next_back() {
		name.as_os_str().to_string_lossy().to_string()
	} else {
		path.display().to_string()
	}
}

fn main() {
	let files = fs::read_dir(".").unwrap();

	for file in files {
		println!("{}", filename(&file.unwrap().path()))
	}

	println!("{:?}", terminal_size())
}
