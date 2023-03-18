// use std::env;
use std::cmp::Reverse;
use std::fs;
use std::path::{Path, PathBuf};

use crate::display::grid;

mod display;
mod termsize;

fn filename(path: &Path) -> String {
	if let Some(name) = path.file_name() {
		name.to_string_lossy().to_string()
	} else {
		path.display().to_string()
	}
}
fn ext(fname: String) -> Option<String> {
	let e = fname.split(".").collect::<Vec<_>>();
	match e.len() {
		0 | 1 => None,
		n => Some(e[n - 1].to_string()),
	}
}

#[derive(Clone, Debug)]
pub struct File {
	pub path: PathBuf,
	pub name: String,
	pub ext: Option<String>,
	pub dir: bool,
	pub dot: bool,
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let list = fs::read_dir("../..").unwrap();

	// let mut file_names = list
	// 	.map(|x| filename(&x.unwrap().path()))
	// 	.collect::<Vec<String>>();

	let mut file_names = list
		.map(|x| {
			let path = &x.unwrap().path();
			let md = std::fs::symlink_metadata(path).unwrap();
			let fname = filename(path);
			let dot = fname.chars().next().unwrap() == '.';

			File {
				path: path.to_path_buf(),
				name: fname.clone(),
				ext: if dot { None } else { ext(fname) },
				dir: md.is_dir(),
				dot: dot,
			}
		})
		.collect::<Vec<File>>();

	file_names.sort_by_key(|f| (Reverse(f.dir), f.name.clone()));
	println!("{}", grid(&file_names, 3));
}
