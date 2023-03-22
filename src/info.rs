use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use libc::S_IXUSR;

use crate::color::colorise;
use crate::display::INDENT;

#[derive(Clone, Debug)]
pub struct File {
	pub name: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	// pub dot: bool,
	// pub exe: bool,
	// pub lnk: bool,
	// pub rwx: u32,
}

fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => path.display().to_string(),
	}
}

fn ext(path: &Path) -> String {
	match path.extension() {
		Some(ext) => ext.to_string_lossy().to_lowercase(),
		_ => "".to_string(),
	}
}

fn ext_group(ext: String) -> (String, u8) {
	match ext.as_str() {
		"png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" => (format!("1_{ext}"), 1),

		"7z" | "zip" | "tar" | "gz" | "bz2" | "rar" | "tgz" | "xz" | "txz" => {
			(format!("2_{ext}"), 2)
		}
		"djvu" | "doc" | "docx" | "dotx" | "odp" | "odt" | "pdf" | "ppt" | "pptx" | "rtf"
		| "xls" | "xlsx" => (format!("3_{ext}"), 3),

		"html" | "css" | "scss" | "sass" | "js" | "jsx" | "ts" | "tsx" | "go" | "rs" | "java" => {
			(format!("4a_{ext}"), 4)
		}
		"txt" | "md" | "csv" | "yaml" => (format!("4b_{ext}"), 4),

		"avi" | "flv" | "m2v" | "m4v" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "ogm" | "ogv"
		| "vob" | "wmv" | "webm" => (format!("5a_{ext}"), 5),
		"aac" | "m4a" | "mka" | "mp3" | "ogg" | "opus" | "wma" => (format!("5b_{ext}"), 6),
		"alac" | "ape" | "flac" | "wav" => (format!("5c_{ext}"), 7),

		"tmp" | "swp" | "swo" | "swn" | "bak" | "bkp" | "bk" | "parts" => (format!("zzz_{ext}"), 9),
		_ => (ext, 0),
	}
}

pub fn file_info(path: &PathBuf, hide: bool) -> Option<File> {
	let fname = filename(path);
	let md = std::fs::symlink_metadata(path).unwrap();

	let dot = fname.chars().next().unwrap() == '.';
	let rwx = md.permissions().mode(); // println!("permissions: {:o} {}", rwx, fname);
	let lnk = md.is_symlink();
	let exe = rwx & S_IXUSR as u32 == S_IXUSR as u32;

	let mut len = fname.chars().count() + INDENT;
	if lnk {
		len += 1
	}
	let (ext, egrp) = ext_group(ext(path));
	let dir = md.is_dir();
	let name = colorise(&fname, &ext, dir, exe, egrp, lnk);

	if dot && hide || !dot {
		return Some(File {
			name,
			ext,
			len,
			dir,
		});
	} else {
		return None;
	}
}
