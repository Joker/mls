use std::os::unix::prelude::{MetadataExt, PermissionsExt};

use super::spaces;
use crate::color::{permissions_fmt, BLUE_L, CYAN, MAGENTA, WHITE};
use crate::datetime::date_time_fmt;
use crate::display::SIZE_WIDTH;
use crate::unsafelibc::username_group;
use crate::{color::size_fmt, info::File};

pub fn to_string(files: &Vec<File>, unreadable: bool) -> String {
	files
		.iter()
		.map(|f| line_fmt(f, unreadable))
		.collect::<Vec<_>>()
		.join("\n")
}

fn line_fmt(f: &File, unreadable: bool) -> String {
	let (username, group) = username_group(f.md.uid(), f.md.gid());
	let mtm = f.md.modified().ok().unwrap();
	// let atm = md.accessed().ok().unwrap();
	// let ctm = md.created().ok().unwrap();
	format!(
		"{WHITE}{}{} {WHITE}{} {} {} {} {}",
		kind(f),
		permissions_fmt(f.md.permissions().mode()),
		username,
		group,
		date_time_fmt(mtm),
		file_size(f, unreadable),
		f.name,
	)
}

fn kind(f: &File) -> String {
	if f.lnk {
		return format!("{CYAN}l");
	}
	if f.dir {
		return format!("{BLUE_L}d");
	}
	match f.md.nlink() {
		n if n > 9 => return format!("{MAGENTA}*"),
		n if n > 1 => return format!("{MAGENTA}{n}"),
		_ => return String::from(" "),
	}
}

fn file_size(f: &File, unreadable: bool) -> String {
	const UNRDB_WIDTH: usize = 11;
	if f.dir {
		return format!(
			"{}-",
			spaces(if unreadable {
				UNRDB_WIDTH - 1
			} else {
				SIZE_WIDTH - 1
			})
		);
	}
	if unreadable {
		let spc = f.size.to_string().len();
		return format!("{WHITE}{}{}", spaces(UNRDB_WIDTH - spc), f.size);
	}
	size_fmt(f.size)
}
