pub mod link;
pub mod mode;
pub mod name;
pub mod size;
pub mod time;

use std::fs;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use crate::{
	color::RED,
	display::GRID_GAP,
	ext::{unlibc::username_group, xattr::FileAttributes},
	{Flags, Width},
};

use self::{
	mode::{kind_fmt, permissions_fmt},
	name::{ext, ext_group, filename, filename_fmt},
	size::size_to_string,
};

pub const USEREXE: u32 = 64;

#[derive(Clone, Debug)]
pub struct File {
	pub sname: String,
	pub name: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub line: Option<FileLine>,
}

#[derive(Clone, Debug)]
pub struct FileLine {
	pub time: u64,
	pub size: u64,
	pub size_str: String,
	pub suf: String,
	pub user: String,
	pub group: String,
	pub perm: String,
	pub lnk: bool,
	pub xattr: bool,
}

fn grid_info(path: &PathBuf, sname: String) -> File {
	let md = std::fs::symlink_metadata(path).unwrap();

	let lnk = md.is_symlink();
	let exe = md.permissions().mode() & USEREXE == USEREXE; // S_IXUSR
	let (ext, egrp) = ext_group(ext(path));
	let mut dir = md.is_dir();
	let mut name = filename_fmt(&sname, &ext, egrp, dir, exe, lnk);
	let len = sname.chars().count() + GRID_GAP;

	if lnk {
		let nvalid;
		(_, _, _, dir, nvalid) = link::info(&path);
		if nvalid {
			name = format!("{RED}{sname}");
		}
	}
	return File {
		sname,
		name,
		ext,
		len,
		dir,
		line: None,
	};
}

fn list_info(path: &PathBuf, sname: String, wh: &mut Width, fl: &Flags) -> File {
	let md = std::fs::symlink_metadata(path).unwrap();

	let mut dir = md.is_dir();
	let lnk = md.is_symlink();
	let rwx = md.permissions().mode();
	let exe = rwx & USEREXE == USEREXE; // S_IXUSR
	let (ext, egrp) = ext_group(ext(path));
	let mut name = filename_fmt(&sname, &ext, egrp, dir, exe, lnk);

	let size = match dir && !lnk {
		true => match md.nlink() {
			s if s < 3 => 0,
			s => s - 2,
		},
		false => md.size(),
	};

	let mut size_str = "".to_string();
	let mut suf = "".to_string();
	let sn = if dir {
		size.to_string().len() + 1
	} else if fl.bytes {
		size.to_string().len()
	} else {
		(size_str, suf) = size_to_string(size);
		size_str.len() + suf.len()
	};
	if wh.szn < sn {
		wh.szn = sn
	}

	let (user, mut group) = username_group(md.uid(), md.gid());
	if wh.uid < user.len() {
		wh.uid = user.len()
	}
	if !fl.group {
		group = "".to_string();
	} else if wh.gid < group.len() {
		wh.gid = group.len()
	}

	if lnk {
		let fname;
		(fname, dir) = link::ref_fmt(&path, fl.full);
		name.push_str(&fname);
	}
	let len = sname.chars().count() + GRID_GAP;

	let xattr = match path.attributes() {
		Ok(xa) => xa.len() > 0,
		Err(_) => false,
	};
	if xattr {
		wh.xattr = true
	}

	return File {
		sname,
		name,
		ext,
		len,
		dir,
		line: Some(FileLine {
			time: time::unix(&md, fl),
			size,
			size_str,
			suf,
			user,
			group,
			perm: format!("{}{}", kind_fmt(lnk, dir, md.nlink()), permissions_fmt(rwx)),
			lnk,
			xattr,
		}),
	};
}

pub fn info(path: &PathBuf, fl: &Flags, wh: &mut Width) -> Option<File> {
	let sname = filename(path);
	let dot = sname.starts_with('.') && sname.len() > 1;

	if !dot || fl.all {
		let file = match fl.list_format {
			true => list_info(path, sname, wh, fl),
			false => grid_info(path, sname),
		};

		if fl.dir_only && !file.dir {
			return None;
		}
		return Some(file);
	}
	None
}

pub fn list(path: &Path, fl: &Flags, w: &mut Width) -> Vec<File> {
	match fs::read_dir(path) {
		Ok(list) => list
			.filter_map(|x| info(&x.unwrap().path(), &fl, w))
			.collect::<Vec<File>>(),
		Err(e) => {
			println!("read_dir: {}", e);
			return Vec::new();
		}
	}
}
