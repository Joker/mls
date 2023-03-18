// use std::env;
use std::cmp::Reverse;
use std::fs;
use std::path::{Path, PathBuf};

use crate::display::grid;

mod display;
mod termsize;

fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => path.display().to_string(),
	}
}
fn ext(path: &Path) -> String {
	match path.extension() {
		Some(ext) => ext.to_string_lossy().to_string(),
		_ => "".to_string(),
	}
}

#[derive(Clone, Debug)]
pub struct File {
	pub path: PathBuf,
	pub name: String,
	pub ext: String,
	pub dir: bool,
	pub dot: bool,
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let list = fs::read_dir("../..").unwrap();

	// let mut file_names = list
	// 	.map(|x| filename(&x.unwrap().path()))
	// 	.collect::<Vec<String>>();

	let flag = false;

	let mut file_names = list
		.filter_map(|x| {
			let path = &x.unwrap().path();
			let fname = filename(path);
			let dot = fname.chars().next().unwrap() == '.';
			let md = path.metadata().unwrap();

			if dot && flag || !dot {
				return Some(File {
					path: path.to_path_buf(),
					name: fname.clone(),
					ext: ext(path),
					dir: md.is_dir(),
					dot: dot,
				});
			} else {
				return None;
			}
		})
		.collect::<Vec<File>>();

	file_names.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.name.clone()));
	println!("{}", grid(&file_names, 3));
	// println!("{:?}", file_names);
}
