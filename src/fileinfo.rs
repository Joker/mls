use std::fs::Metadata;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::color::{file_name_fmt, CYAN, RED, WHITE};
use crate::info::USEREXE;
use crate::Flags;

pub fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => path.display().to_string(),
	}
}

pub fn basepath(path: &Path) -> String {
	match path.parent() {
		Some(p) => p.to_string_lossy().to_string(),
		_ => "".to_string(),
	}
}

pub fn ext(path: &Path) -> String {
	match path.extension() {
		Some(ext) => ext.to_string_lossy().to_lowercase(),
		_ => "".to_string(),
	}
}

pub fn ext_group(ext: String) -> (String, u8) {
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

pub fn time(md: &Metadata, fl: &Flags) -> u64 {
	if fl.U_create {
		match md.created().ok() {
			Some(t) => match t.duration_since(UNIX_EPOCH) {
				Ok(s) => s.as_secs(),
				Err(_) => 0,
			},
			None => 0,
		}
	} else if fl.u_access {
		md.atime() as u64
	} else if fl.ctime {
		md.ctime() as u64
	} else {
		md.mtime() as u64
	}
}

pub fn link(pb: &PathBuf) -> (PathBuf, PathBuf, bool, bool, bool) {
	let mut path = PathBuf::new();
	let mut pb_path = PathBuf::new();
	let mut nvalid = false;
	let mut dir = false;
	let mut exe = false;
	match std::fs::read_link(pb) {
		Ok(p) => {
			pb_path = if p.is_relative() {
				PathBuf::from(format!("{}/{}", basepath(pb), p.to_string_lossy()))
			} else {
				p.clone()
			};
			path = p;
			match std::fs::metadata(&pb_path) {
				Ok(metadata) => {
					dir = metadata.is_dir();
					exe = metadata.permissions().mode() & USEREXE == USEREXE;
				}
				Err(_) => nvalid = true,
			}
		}
		Err(_) => nvalid = true,
	};
	(path, pb_path, exe, dir, nvalid)
}

pub fn link_line(pb: &PathBuf, abs: bool) -> (String, bool) {
	let (path, pb_path, exe, dir, nvalid) = link(pb);

	if nvalid {
		return (format!("{RED} -> {}", path.to_string_lossy()), false);
	}

	let (ext, egrp) = ext_group(ext(&path));
	let name = filename(&path);

	let path_to = if abs {
		match std::fs::canonicalize(&pb_path) {
			Ok(s) => basepath(s.as_path()),
			Err(_) => basepath(path.as_path()),
		}
	} else {
		basepath(path.as_path())
	};
	(
		format!(
			"{WHITE} -> {CYAN}{path_to}/{}",
			file_name_fmt(&name, &ext, egrp, dir, exe, false)
		),
		dir,
	)
}
