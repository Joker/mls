use std::{cmp::Reverse, path::Path};

use crate::{
	color::WHITE,
	file::{self, name::basepath, File},
	Flags, Width,
};

pub static END: &str = "└──";
pub static LEAF: &str = "├──";
pub static TRUNK: &str = "│  ";

pub fn trunc(width: usize) -> String {
	(0..width).into_iter().map(|_| TRUNK).collect()
}

pub fn list(path: &Path, fl: &Flags, w: &mut Width, lvl: usize) -> Vec<File> {
	let mut out = Vec::new();
	if fl.lvl <= lvl {
		return out;
	}
	let mut files = file::list(path, fl, w);
	if files.len() == 0 {
		return out;
	}
	files.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));

	if lvl == 0 {
		let mut f = file::info(&path.to_path_buf(), &fl, w).unwrap();
		f.name = format!("{WHITE}{}{}", basepath(path), f.name);
		out.push(f);
	}

	let last = files.iter().last().unwrap();
	files.iter().for_each(|f| {
		let mut fclone = f.clone();
		fclone.name = format!(
			"{WHITE}{}{} {}",
			trunc(lvl),
			if std::ptr::eq(f, last) { END } else { LEAF },
			f.name
		);
		out.push(fclone);

		if f.dir {
			let subt = list(path.join(Path::new(&f.sname)).as_path(), fl, w, lvl + 1);
			out.extend(subt);
		}
	});
	out
}
