pub mod link;
pub mod mode;
pub mod name;
pub mod size;
pub mod time;

#[cfg(feature = "xattr")]
pub mod attr;
#[cfg(feature = "xattr")]
use self::attr::{ext_attr, Xattr};

#[derive(Clone, Debug)]
#[cfg(not(feature = "xattr"))]
pub struct Xattr {}

use std::fs;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use crate::{
	args::Flags,
	color::{MAGENTA, RED, RESET, UNDERLINE, WHITE},
	display::GRID_GAP,
	ext::unlibc::username_group,
	Width,
};

use self::{
	mode::{permissions_fmt, underline, USER_EXE},
	name::{ext, ext_group, filename, filename_fmt},
	size::size_to_string,
};

#[derive(Clone, Debug)]
pub struct File {
	pub sname: String,
	pub name: String,
	pub ext: String,
	pub len: usize,
	pub dir: bool,
	pub line: Option<Box<FileLine>>,
}

#[derive(Clone, Debug)]
pub struct FileLine {
	pub time: u64,
	pub size: u64,
	pub size_str: String,
	pub size_suf: String,
	pub user: String,
	pub group: String,
	pub inode: String,
	pub perm: String,
	pub lnk: bool,
	pub attr: Option<Xattr>,
}

fn grid_info(path: &PathBuf, sname: String) -> File {
	let md = std::fs::symlink_metadata(path).unwrap();

	let lnk = md.is_symlink();
	let rwx = md.permissions().mode();
	let exe = rwx & USER_EXE == USER_EXE;
	// let (ext, egrp) = ext_group(ext(path));
	let mut dir = md.is_dir();
	let (ext, egrp) = if dir { (String::new(), 0) } else { ext_group(ext(path)) };
	let mut name = filename_fmt(&sname, &ext, egrp, dir, exe, lnk);
	let len = sname.chars().count() + GRID_GAP;

	if lnk {
		let error;
		(_, _, _, dir, error) = link::info(path);
		if error {
			name = format!("{RED}{sname}");
		}
	}
	if underline(rwx) {
		name = format!("{UNDERLINE}{name}{RESET}");
	}
	File {
		sname,
		name,
		ext,
		len,
		dir,
		line: None,
	}
}

fn list_info(path: &PathBuf, sname: String, wh: &mut Width, fl: &Flags) -> File {
	let md = if fl.follow {
		match std::fs::metadata(path) {
			Ok(mdata) => mdata,
			_ => std::fs::symlink_metadata(path).unwrap(),
		}
	} else {
		std::fs::symlink_metadata(path).unwrap()
	};
	let mut dir = md.is_dir();
	let lnk = md.is_symlink();
	let rwx = md.permissions().mode();
	let exe = rwx & USER_EXE == USER_EXE;
	// let (ext, egrp) = ext_group(ext(path));
	let (ext, egrp) = if dir { (String::new(), 0) } else { ext_group(ext(path)) };
	let mut name = filename_fmt(&sname, &ext, egrp, dir, exe, lnk);

	let size = match dir && !lnk {
		true => match md.nlink() {
			s if s < 3 => 0,
			s => s - 2,
		},
		false => md.size(),
	};

	let mut size_str = String::new();
	let mut size_suf = String::new();
	let sn = if dir {
		size.to_string().len() + 1
	} else if fl.bytes {
		size.to_string().len()
	} else {
		(size_str, size_suf) = size_to_string(size);
		size_str.len() + size_suf.len()
	};
	if wh.szn < sn {
		wh.szn = sn
	}

	let (user, mut group) = username_group(md.uid(), md.gid());
	if wh.uid < user.len() {
		wh.uid = user.len()
	}
	if !fl.group {
		group = String::new();
	} else if wh.gid < group.len() {
		wh.gid = group.len()
	}

	let mut inode = String::new();
	if fl.inode {
		inode = format!("{MAGENTA}{}  ", md.ino());
		if wh.inode < inode.len() {
			wh.inode = inode.len()
		}
	}

	if lnk {
		let fname;
		(fname, dir) = link::ref_fmt(path, fl.full);
		name.push_str(&fname);
	}
	if underline(rwx) {
		name = format!("{UNDERLINE}{name}{RESET}");
	}
	let len = sname.chars().count() + GRID_GAP;

	#[cfg(feature = "xattr")]
	let attr = ext_attr(path);
	#[cfg(feature = "xattr")]
	if attr.is_some() {
		wh.xattr = true;
	}
	#[cfg(not(feature = "xattr"))]
	let attr = None;

	File {
		sname,
		name,
		ext,
		len,
		dir,
		line: Some(Box::new(FileLine {
			time: time::unix(&md, fl),
			size,
			size_str,
			size_suf, // k M G
			user,
			group,
			perm: permissions_fmt(rwx, md.nlink(), fl),
			lnk,
			inode,
			attr,
		})),
	}
}

pub fn info(path: &PathBuf, fl: &Flags, wh: &mut Width) -> Option<File> {
	let sname = filename(path);
	let dot = sname.starts_with('.');

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
			.filter_map(|x| info(&x.unwrap().path(), fl, w))
			.collect::<Vec<File>>(),
		Err(e) => {
			println!("read path: {RED}{: <80}{WHITE}  {}", path.to_string_lossy(), e);
			Vec::new()
		}
	}
}
