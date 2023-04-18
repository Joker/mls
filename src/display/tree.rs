use std::{cmp::Reverse, path::Path};

use crate::{
	args::Flags,
	color::WHITE,
	file::{self, name::basepath, File},
	Width,
};

pub static END: &str = "└──";
pub static LEAF: &str = "├──";
pub static TRUNK: &str = "│  ";
pub static HOLLOW: &str = "   ";

pub fn list(path: &Path, fl: &Flags, w: &mut Width, lvl: usize, trunk: String) -> Vec<File> {
	let mut out = Vec::new();
	if fl.lvl <= lvl {
		return out;
	}

	let mut files = file::list(path, fl, w);
	if files.len() == 0 {
		return out;
	}

	if fl.size_sort {
		files.sort_by_key(|f| (Reverse(f.dir), f.line.as_ref().unwrap().size));
	} else if fl.time_sort {
		files.sort_by_key(|f| (f.line.as_ref().unwrap().time));
	} else {
		files.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
	}

	if lvl == 0 {
		let mut f = file::info(&path.to_path_buf(), &fl, w).unwrap();
		f.name = format!("{WHITE} {}{}", basepath(path), f.name);
		out.push(f);
	}

	let last_file = files.iter().last().unwrap();
	files.iter().for_each(|f| {
		let last = std::ptr::eq(f, last_file);
		let mut fclone = f.clone();
		fclone.name = format!("{WHITE}{}{} {}", trunk, if last { END } else { LEAF }, f.name);
		out.push(fclone);

		if f.dir {
			let subt = list(
				path.join(Path::new(&f.sname)).as_path(),
				fl,
				w,
				lvl + 1,
				if last { format!("{}{}", trunk, HOLLOW) } else { format!("{}{}", trunk, TRUNK) },
			);
			out.extend(subt);
		}
	});
	out
}

pub fn print(files: &Vec<File>) {
	println!(
		"{}",
		files.iter().map(|f| f.name.clone()).collect::<Vec<_>>().join("\n")
	)
}
