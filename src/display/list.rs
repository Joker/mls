use std::os::unix::prelude::MetadataExt;

use super::spaces;
use crate::color::WHITE;
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
	let (usr,grp) = username_group(f.md.uid(), f.md.gid());
	format!(
		"{WHITE}{} {} {} {} {}",
		usr,
		grp,
		size(f, false),
		size(f, unreadable),
		f.name
	)
}

fn size(f: &File, unreadable: bool) -> String {
	if f.dir {
		return spaces(if unreadable { 11 } else { 6 });
	}
	if unreadable {
		let spc = f.size.to_string().len();
		return format!("{WHITE}{}{}", spaces(11 - spc), f.size);
	}
	let size = size_fmt(f.size);
	let sp = spaces(6 - (size.len() - 14));
	format!("{}{}", sp, size)
}
