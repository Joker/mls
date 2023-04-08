use std::{fs, path::Path};

use crate::{
	color::WHITE,
	file::{self, File},
	Flags, Width,
};

pub static END: &str = "└──";
pub static LEAF: &str = "├──";
pub static TRUNK: &str = "│  ";

pub fn trunc(width: usize) -> String {
	(0..width).into_iter().map(|_| TRUNK).collect()
}

fn line_fmt(f: &File, _fl: &Flags, _w: &Width) -> String {
	f.name.clone()
}

fn dir(path: &Path, fl: &Flags, w: &mut Width) -> Vec<File> {
	match fs::read_dir(path) {
		Ok(list) => list
			.filter_map(|x| file::info(&x.unwrap().path(), &fl, w))
			.collect::<Vec<File>>(),
		Err(e) => {
			println!("read_dir: {}", e);
			return Vec::new();
		}
	}
}

pub fn print(path: &Path, fl: &Flags, w: &mut Width, level: usize) {
	let files = dir(path, fl, w);
	let last = files.iter().last().unwrap();

	files.iter().for_each(|f| {
		if std::ptr::eq(f, last) {
			println!("{}{}{} {}", WHITE, trunc(level), END, line_fmt(f, fl, w))
		} else {
			println!("{}{}{} {}", WHITE, trunc(level), LEAF, line_fmt(f, fl, w))
		}
		if f.dir {
			print(path.join(Path::new(&f.sname)).as_path(), fl, w, level + 1)
		}
	})
}
