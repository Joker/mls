use super::spaces;
use crate::color::{BLACK_H, BLACK_L, GREEN, GREEN_L, WHITE};
use crate::datetime::date_time_fmt;
use crate::display::{FSIZE_WIDTH, TIMEZONE};
use crate::info::File;
use crate::{Flags, Width};

pub fn print(files: &Vec<File>, fl: &Flags, w: &Width) {
	println!(
		"{}",
		files
			.iter()
			.map(|f| line_fmt(f, fl, w))
			.collect::<Vec<_>>()
			.join("\n")
	)
}

fn line_fmt(f: &File, fl: &Flags, w: &Width) -> String {
	match &f.long {
		Some(l) => format!(
			"{WHITE}{}{WHITE}{: >ncu$}{: >ncg$} {}  {}  {}",
			l.perm,
			l.user,
			l.group,
			date_time_fmt(f.time + TIMEZONE),
			l.size,
			f.name,
			ncu = w.uid + 1,
			ncg = if fl.group { w.gid + 1 } else { 0 }
		),
		_ => "".to_string(),
	}
}

pub fn file_size(size: u64, dir: bool, lnk: bool, bitsize: bool) -> String {
	const BIT_WIDTH: usize = 11;
	if dir && (lnk || size == 0) {
		let gap = if bitsize {
			BIT_WIDTH - 1
		} else {
			FSIZE_WIDTH - 1
		};
		return format!("{BLACK_H}{}-", spaces(gap));
	}
	if dir {
		let sp = match bitsize {
			true => spaces(BIT_WIDTH - 5),
			false => "".to_string(),
		};
		return format!("{WHITE}{sp}{: >4}{BLACK_L}f", size);
	}
	if bitsize {
		let spc = size.to_string().len();
		return format!("{WHITE}{}{}", spaces(BIT_WIDTH - spc), size);
	}
	size_fmt(size)
}

fn size_fmt(bytes: u64) -> String {
	match bytes {
		bt if bt >= 1073741824 => color_size(short_size(bt as f64), "G"),
		bt if bt >= 1048576 => color_size(short_size(bt as f64), "M"),
		bt if bt >= 1024 => color_size(short_size(bt as f64), "k"),
		bt if bt >= 1 => color_size(bt.to_string(), ""),
		_ => format!("{GREEN_L}{}0{GREEN}", spaces(FSIZE_WIDTH - 1)),
	}
}

fn color_size(size: String, suffix: &str) -> String {
	let sp = spaces(FSIZE_WIDTH - size.len() - suffix.len());
	format!("{GREEN_L}{sp}{size}{GREEN}{suffix}")
}

const KB: f64 = 1024.0;
fn short_size(bytes: f64) -> String {
	let base = bytes.log10() / KB.log10();
	let ans = KB.powf(base - base.floor());
	if ans < 100.0 {
		return format!("{:.1}", ans).trim_end_matches(".0").to_owned();
	}
	format!("{:.0}", ans)
}
