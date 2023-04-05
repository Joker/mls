use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::PathBuf;

use crate::color::{file_name_fmt, kind_fmt, permissions_fmt, RED};
use crate::display::{list::size_to_string, GRID_GAP};
use crate::fileinfo::{ext, ext_group, filename, link, link_line, time};
use crate::unsafelibc::username_group;
use crate::xattr::FileAttributes;
use crate::{Flags, Width};

pub const USEREXE: u32 = 64;

#[derive(Debug)]
pub struct File {
	pub sname: String,
	pub name: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub long: Option<FileLine>,
}

#[derive(Debug)]
pub struct FileLine {
	pub size: u64,
	pub time: u64,
	pub user: String,
	pub group: String,
	pub perm: String,
	pub str_size: String,
	pub suf: String,
	pub lnk: bool,
	pub xattr: bool,
}

fn f_info(path: &PathBuf, sname: String) -> File {
	let md = std::fs::symlink_metadata(path).unwrap();

	let lnk = md.is_symlink();
	let exe = md.permissions().mode() & USEREXE == USEREXE; // S_IXUSR
	let (ext, egrp) = ext_group(ext(path));
	let mut dir = md.is_dir();
	let mut name = file_name_fmt(&sname, &ext, egrp, dir, exe, lnk);
	let len = sname.chars().count() + GRID_GAP;

	if lnk {
		let nvalid;
		(_, _, _, dir, nvalid) = link(&path);
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
		long: None,
	};
}

fn l_info(path: &PathBuf, sname: String, wh: &mut Width, fl: &Flags) -> File {
	let md = std::fs::symlink_metadata(path).unwrap();

	let mut dir = md.is_dir();
	let lnk = md.is_symlink();
	let rwx = md.permissions().mode();
	let exe = rwx & USEREXE == USEREXE; // S_IXUSR
	let (ext, egrp) = ext_group(ext(path));
	let mut name = file_name_fmt(&sname, &ext, egrp, dir, exe, lnk);

	let size = match dir && !lnk {
		true => match md.nlink() {
			s if s < 3 => 0,
			s => s - 2,
		},
		false => md.size(),
	};

	let mut str_size = "".to_string();
	let mut suf = "".to_string();
	let sn = if dir {
		size.to_string().len() + 1
	} else if fl.bytes {
		size.to_string().len()
	} else {
		(str_size, suf) = size_to_string(size);
		str_size.len() + suf.len()
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
		(fname, dir) = link_line(&path, fl.full);
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
		long: Some(FileLine {
			size,
			time: time(&md, fl),
			user,
			group,
			perm: format!("{}{}", kind_fmt(lnk, dir, md.nlink()), permissions_fmt(rwx)),
			str_size,
			suf,
			lnk,
			xattr,
		}),
	};
}

pub fn file_info(path: &PathBuf, fl: &Flags, wh: &mut Width) -> Option<File> {
	let sname = filename(path);
	let dot = sname.chars().next().unwrap() == '.';

	if !dot || fl.all {
		let file = match fl.long || fl.Size_sort || fl.time_sort || fl.group {
			true => l_info(path, sname, wh, fl),
			false => f_info(path, sname),
		};

		if fl.dir_only && !file.dir {
			return None;
		}
		return Some(file);
	}
	None
}
