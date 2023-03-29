use std::os::unix::prelude::{MetadataExt, PermissionsExt};

use super::spaces;
use crate::color::{permissions_fmt, BLACK_H, BLACK_L, BLUE_L, CYAN, MAGENTA, WHITE};
use crate::datetime::date_time_fmt;
use crate::display::{SIZE_WIDTH, TIMEZONE};
use crate::{color::size_fmt, info::File};

pub fn to_string(files: &Vec<File>, unreadable: bool, name_width: usize) -> String {
	files
		.iter()
		.map(|f| line_fmt(f, unreadable, name_width))
		.collect::<Vec<_>>()
		.join("\n")
}

fn line_fmt(f: &File, unreadable: bool, name_width: usize) -> String {
	let username_group = match &f.user {
		Some(u) => u,
		None => "",
	};
	format!(
		"{WHITE}{}{} {WHITE}{: >ncv$} {}  {}  {}  ",
		kind(f),
		permissions_fmt(f.md.permissions().mode()),
		username_group,
		date_time_fmt(f.time + TIMEZONE),
		file_size(f, unreadable),
		f.name,
		ncv = name_width
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
	if f.dir && (f.lnk || f.size == 0) {
		return format!(
			"{BLACK_H}{}-",
			spaces(if unreadable {
				UNRDB_WIDTH - 1
			} else {
				SIZE_WIDTH - 1
			})
		);
	}
	if f.dir {
		let sp = match unreadable {
			true => spaces(UNRDB_WIDTH - 5),
			false => "".to_string(),
		};
		return format!("{WHITE}{sp}{: >4}{BLACK_L}f", f.size);
	}
	if unreadable {
		let spc = f.size.to_string().len();
		return format!("{WHITE}{}{}", spaces(UNRDB_WIDTH - spc), f.size);
	}
	size_fmt(f.size)
}
