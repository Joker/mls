// use std::env;
use std::fs;
use std::path::Path;

use crate::display::grid;

mod display;
mod termsize;

fn filename(path: &Path) -> String {
	if let Some(name) = path.components().next_back() {
		name.as_os_str().to_string_lossy().to_string()
	} else {
		path.display().to_string()
	}
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let files = fs::read_dir("..").unwrap();

	let file_names = files
		.map(|x| filename(&x.unwrap().path()))
		.collect::<Vec<String>>();

	println!("{}", grid(&file_names));
}
