use std::fs::Metadata;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use libc::S_IXUSR;

use crate::color::{colorise, CYAN, RED, WHITE};
use crate::display::GRID_GAP;

#[derive(Clone, Debug)]
pub struct File {
	pub name: String,
	pub sname: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub size: u64,
	pub md: Metadata,
	// pub dot: bool,
	// pub exe: bool,
	pub lnk: bool,
	// pub linkpath: Option<String>,
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
		"json" | "txt" | "md" | "csv" | "yaml" => (format!("4b_{ext}"), 4),

		"avi" | "flv" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "vob" | "wmv" | "webm" => {
			(format!("5a_{ext}"), 5)
		}
		"aac" | "mp3" | "ogg" | "opus" | "wma" | "flac" | "wav" => (format!("5b_{ext}"), 6),

		"tmp" | "swp" | "swo" | "swn" | "bak" | "bkp" | "bk" | "parts" => (format!("zzz_{ext}"), 9),
		_ => (ext, 0),
	}
}

pub fn file_info(path: &PathBuf, hide: bool, long: bool, abs: bool) -> Option<File> {
	let sname = filename(path);
	let len = sname.chars().count() + GRID_GAP;
	let dot = sname.chars().next().unwrap() == '.';

	let md = std::fs::symlink_metadata(path).unwrap();
	let rwx = md.permissions().mode();
	let lnk = md.is_symlink();
	let mut dir = md.is_dir();

	let exe = rwx & S_IXUSR as u32 == S_IXUSR as u32;
	let (ext, egrp) = ext_group(ext(path));

	let mut name = colorise(&sname, &ext, egrp, dir, exe, lnk);
	if lnk {
		let (fname, d) = read_lnk(&path, abs);
		dir = d;
		if long {
			name.push_str(&fname);
		}
	}

	if dot && hide || !dot {
		return Some(File {
			name,
			sname,
			size: if !dir { md.size() } else { 0 },
			ext,
			len,
			dir,
			lnk,
			md,
		});
	} else {
		return None;
	}
}

fn read_lnk(pb: &PathBuf, abs: bool) -> (String, bool) {
	let path = match std::fs::read_link(pb) {
		Ok(lnk) => lnk,
		Err(_) => return (String::from("link error"), false),
	};

	let dir;
	let rwx;
	let name = filename(&path);
	match std::fs::metadata(&path) {
		Ok(metadata) => {
			dir = metadata.is_dir();
			rwx = metadata.permissions().mode();
		}
		Err(_) => return (format!("{RED} -> {}", path.to_string_lossy()), false),
	}

	let (ext, egrp) = ext_group(ext(&path));
	let exe = rwx & S_IXUSR as u32 == S_IXUSR as u32;

	let path_to = if abs {
		match std::fs::canonicalize(&path) {
			Ok(s) => s.to_string_lossy().replace(&name, ""),
			Err(_) => path.to_string_lossy().replace(&name, ""),
		}
	} else {
		path.to_string_lossy().replace(&name, "")
	};
	(
		format!(
			"{WHITE} -> {CYAN}{path_to}{}",
			colorise(&name, &ext, egrp, dir, exe, false)
		),
		dir,
	)
}
