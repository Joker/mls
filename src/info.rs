use std::fs::Metadata;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use libc::S_IXUSR;

use crate::color::{colorise, CYAN, RED, WHITE};
use crate::display::GRID_GAP;
use crate::unsafelibc::username_group;
use crate::Flag;

#[derive(Clone, Debug)]
pub struct File {
	pub name: String,
	pub sname: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub size: u64,
	pub time: u64,
	pub md: Metadata,
	// pub dot: bool,
	// pub exe: bool,
	pub lnk: bool,
	pub user: Option<String>,
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

fn time(md: &Metadata, fl: &Flag) -> u64 {
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
		md.atime() as u64
	} else {
		md.mtime() as u64
	}
}

pub fn file_info(path: &PathBuf, fl: &Flag, nlen: &mut usize) -> Option<File> {
	let sname = filename(path);
	let len = sname.chars().count() + GRID_GAP;
	let dot = sname.chars().next().unwrap() == '.';

	let md = std::fs::symlink_metadata(path).unwrap();
	let rwx = md.permissions().mode();
	let lnk = md.is_symlink();
	let time = time(&md, fl);

	let exe = rwx & S_IXUSR as u32 == S_IXUSR as u32;
	let (ext, egrp) = ext_group(ext(path));
	let mut dir = md.is_dir();
	let mut name = colorise(&sname, &ext, egrp, dir, exe, lnk);
	let size = match dir && !lnk {
		false => md.size(),
		true => {
			let s = md.nlink();
			if s < 3 {
				0
			} else {
				s - 2
			}
		}
	};
	let user = if fl.long {
		let uid_gid = username_group(md.uid(), md.gid());
		if *nlen < uid_gid.len() {
			*nlen = uid_gid.len()
		}
		Some(uid_gid)
	} else {
		None
	};

	if lnk {
		let (fname, d) = read_lnk(&path, fl.full);
		dir = d;
		if fl.long {
			name.push_str(&fname);
		}
	}

	if dot && fl.all || !dot {
		return Some(File {
			name,
			sname,
			size,
			time,
			ext,
			len,
			dir,
			lnk,
			md,
			user,
		});
	}

	return None;
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
