use std::fs::Metadata;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::color::{file_name_fmt, kind_fmt, permissions_fmt, CYAN, RED, WHITE};
use crate::display::list::file_size;
use crate::display::GRID_GAP;
use crate::unsafelibc::username_group;
use crate::{Flags, Width};

#[derive(Clone, Debug)]
pub struct File {
	pub sname: String,
	pub name: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub size: u64,
	pub time: u64,
	pub long: Option<FileLine>,
}

#[derive(Clone, Debug)]
pub struct FileLine {
	pub user: String,
	pub group: String,
	pub perm: String,
	pub size: String,
}

pub fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => path.display().to_string(),
	}
}

pub fn basepath(path: &Path) -> String {
	let mut an = path.ancestors();
	an.next();
	match an.next() {
		Some(p) => p.to_string_lossy().to_string(),
		_ => "".to_string(),
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

fn time(md: &Metadata, fl: &Flags) -> u64 {
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

pub fn file_info(path: &PathBuf, fl: &Flags, wh: &mut Width) -> Option<File> {
	let sname = filename(path);
	let dot = sname.chars().next().unwrap() == '.';

	if !dot || fl.all {
		let md = std::fs::symlink_metadata(path).unwrap();

		let mut dir = md.is_dir();
		let lnk = md.is_symlink();
		let rwx = md.permissions().mode();
		let exe = rwx & 64 == 64; // let exe = rwx & S_IXUSR as u32 == S_IXUSR as u32;
		let (ext, egrp) = ext_group(ext(path));
		let mut name = file_name_fmt(&sname, &ext, egrp, dir, exe, lnk);

		let size = match dir && !lnk {
			true => match md.nlink() {
				s if s < 3 => 0,
				s => s - 2,
			},
			false => md.size(),
		};

		let long = fl.long.then(|| line_info(&md, fl, wh, lnk, dir, size, rwx));

		if lnk {
			let fname;
			(fname, dir) = link_info(&path, fl.full);
			if fl.long {
				name.push_str(&fname);
			}
		}
		let len = sname.chars().count() + GRID_GAP;

		if fl.dir_only && !dir {
			return None;
		}
		return Some(File {
			sname,
			name,
			ext,
			len,
			dir,
			size,
			time: time(&md, fl),
			long,
		});
	}
	return None;
}

fn link_info(pb: &PathBuf, abs: bool) -> (String, bool) {
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
	let exe = rwx & 64 == 64;

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
			file_name_fmt(&name, &ext, egrp, dir, exe, false)
		),
		dir,
	)
}

fn line_info(
	md: &Metadata,
	fl: &Flags,
	wh: &mut Width,
	dir: bool,
	lnk: bool,
	sz: u64,
	rwx: u32,
) -> FileLine {
	let (user, mut group) = username_group(md.uid(), md.gid());
	if wh.uid < user.len() {
		wh.uid = user.len()
	}

	if !fl.group {
		group = "".to_string();
	} else if wh.gid < group.len() {
		wh.gid = group.len()
	}

	FileLine {
		user,
		group,
		perm: format!("{}{}", kind_fmt(lnk, dir, md.nlink()), permissions_fmt(rwx)),
		size: file_size(sz, dir, lnk, fl.human),
	}
}
