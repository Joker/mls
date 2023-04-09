use std::{cmp::Reverse, fs, path::Path};

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
	let mut flist = match fs::read_dir(path) {
		Ok(list) => list
			.filter_map(|x| file::info(&x.unwrap().path(), &fl, w))
			.collect::<Vec<File>>(),
		Err(e) => {
			println!("read_dir: {}", e);
			return Vec::new();
		}
	};
	flist.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
	flist
}

pub fn print(path: &Path, fl: &Flags, w: &mut Width, lvl: usize) {
	if fl.lvl <= lvl {
		return;
	}
	let files = dir(path, fl, w);
	if files.len() == 0 {
		return;
	}
	let last = files.iter().last().unwrap();

	files.iter().for_each(|f| {
		if std::ptr::eq(f, last) {
			println!("{}{}{} {}", WHITE, trunc(lvl), END, line_fmt(f, fl, w))
		} else {
			println!("{}{}{} {}", WHITE, trunc(lvl), LEAF, line_fmt(f, fl, w))
		}
		if f.dir {
			print(path.join(Path::new(&f.sname)).as_path(), fl, w, lvl + 1)
		}
	})
}
