use std::os::unix::prelude::{MetadataExt, PermissionsExt};

use super::spaces;
use crate::color::{permissions_fmt, BLUE_L, CYAN, WHITE};
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
	format!(
		"{WHITE}{}{} {WHITE}{} {} {} {}",
		kind(f),
		permissions_fmt(f.md.permissions().mode()),
		username,
		group,
		file_size(f, unreadable),
		f.name,
	)
}

fn kind(f: &File) -> String {
	if f.dir {
		return format!("{BLUE_L}d");
	}
	if f.lnk {
		// read_link(f);
		return format!("{CYAN}l");
	}
	String::from(" ")
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
