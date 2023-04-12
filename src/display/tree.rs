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
pub static GROUND: &str = "   ";

pub fn trunc(width: usize, last_dir: usize) -> String {
	if last_dir == 0 {
		return (0..width).into_iter().map(|_| TRUNK).collect();
	}
	format!(
		"{}{}",
		(0..width).into_iter().map(|_| GROUND).collect::<String>(),
		(0..(width - last_dir)).into_iter().map(|_| TRUNK).collect::<String>(),
	)
}

pub fn list(path: &Path, fl: &Flags, w: &mut Width, lvl: usize, last_dir: usize) -> Vec<File> {
	let mut out = Vec::new();
	if fl.lvl <= lvl {
		return out;
	}

	let mut files = file::list(path, fl, w);
	if files.len() == 0 {
		return out;
	}

	if fl.Size_sort {
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

	let last = files.iter().last().unwrap();
	files.iter().for_each(|f| {
		let last_eq = std::ptr::eq(f, last);
		let mut fclone = f.clone();
		fclone.name = format!(
			"{WHITE}{}{} {}",
			trunc(lvl, last_dir),
			if last_eq { END } else { LEAF },
			f.name
		);
		out.push(fclone);

		if f.dir {
			let ld = if last_eq { last_dir + 1 } else { last_dir };
			let subt = list(path.join(Path::new(&f.sname)).as_path(), fl, w, lvl + 1, ld);
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
