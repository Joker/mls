use super::spaces;
use crate::color::{BLACK_H, BLACK_L, WHITE};
use crate::datetime::date_time_fmt;
use crate::display::{FSIZE_WIDTH, TIMEZONE};
use crate::{color::size_fmt, info::File};

pub fn print(files: &Vec<File>, unreadable: bool, name_width: usize) {
	println!(
		"{}",
		files
			.iter()
			.map(|f| line_fmt(f, unreadable, name_width))
			.collect::<Vec<_>>()
			.join("\n")
	)
}

fn line_fmt(f: &File, unreadable: bool, name_width: usize) -> String {
	let username_group = if let Some(u) = &f.user { u } else { "" };
	let perm = if let Some(u) = &f.perm { u } else { "" };

	format!(
		"{WHITE}{} {WHITE}{: >ncv$} {}  {}  {}  ",
		perm,
		username_group,
		date_time_fmt(f.time + TIMEZONE),
		file_size(f, unreadable),
		f.name,
		ncv = name_width
	)
}

fn file_size(f: &File, bitsize: bool) -> String {
	const BIT_WIDTH: usize = 11;
	if f.dir && (f.lnk || f.size == 0) {
		let gap = if bitsize {
			BIT_WIDTH - 1
		} else {
			FSIZE_WIDTH - 1
		};
		return format!("{BLACK_H}{}-", spaces(gap));
	}
	if f.dir {
		let sp = match bitsize {
			true => spaces(BIT_WIDTH - 5),
			false => "".to_string(),
		};
		return format!("{WHITE}{sp}{: >4}{BLACK_L}f", f.size);
	}
	if bitsize {
		let spc = f.size.to_string().len();
		return format!("{WHITE}{}{}", spaces(BIT_WIDTH - spc), f.size);
	}
	size_fmt(f.size)
}
