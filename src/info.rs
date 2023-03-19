use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use libc::S_IXUSR;
use crate::INDENT;

#[derive(Clone, Debug)]
pub struct File {
	pub name: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub dot: bool,
	pub exe: bool,
	pub lnk: bool,
	pub rwx: u32,
}

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

pub fn file_info(path: &PathBuf, hide: bool) -> Option<File> {
	let fname = filename(path);
	let md = std::fs::symlink_metadata(path).unwrap();

	let dot = fname.chars().next().unwrap() == '.';
	let rwx = md.permissions().mode();
	println!("permissions: {:o} {}", rwx, fname);

	if dot && hide || !dot {
		return Some(File {
			name: fname.clone(),
			ext: ext(path),
			len: fname.chars().count() + INDENT,
			dot,
			exe: rwx & S_IXUSR as u32 == S_IXUSR as u32,
			lnk: md.is_symlink(),
			dir: md.is_dir(),
			rwx,
		});
	} else {
		return None;
	}
}
